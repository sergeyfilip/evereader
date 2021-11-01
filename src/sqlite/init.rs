

use crate::logger::log;
use crate::resource::Resource;
use rusqlite::params;

pub fn init_db(db: &mut rusqlite::Connection, prefix: &str) -> Result<(), rusqlite::Error> {
    let version = db
        .query_row("select max(version) from schema", params![], |row| {
            let version: i64 = row.get(0).unwrap();
            Ok(version)
        })
        .unwrap_or(-1);
    let mut next_version = version + 1;

    loop {
        let filename = format!("{}/V{}.sql", prefix, next_version);
        if let Some(asset) = Resource::get(&filename) {
            if next_version == 0 {
                log::info!("Initializing SQLite database ({})", prefix)
            } else {
                log::info!(
                    "Updating SQLite database to schema version {} ({})",
                    next_version,
                    prefix
                );
            }
            let asset = String::from_utf8_lossy(&asset);
            let tx = db.transaction()?;
            tx.execute_batch(&asset)?;
            tx.execute(
                "INSERT INTO schema (version, timestamp) VALUES (?1, date('now'))",
                params![next_version],
            )?;
            tx.commit()?;
            next_version += 1;
        } else {
            break;
        }
    }

    Ok(())
}
