//! SQLite persistence: stores the full hydrated kit as JSON in a single row.
//! A normalized multi-table layout can be layered on later without changing
//! [`KitStore::from_full_dto`] / [`KitStore::to_json`] boundaries.

use std::path::Path;

use rusqlite::Connection;

use crate::error::Result;
use crate::kit::{KitStore, KitStoreRef};

impl KitStore {
    /// Preferred API (plan): write kit JSON snapshot to `path`.
    pub fn save_sqlite(&self, path: &Path) -> Result<()> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS semio_kit_snapshot (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                payload TEXT NOT NULL
            );",
        )?;
        let payload = self.to_json()?;
        conn.execute(
            "INSERT OR REPLACE INTO semio_kit_snapshot (id, payload) VALUES (1, ?1)",
            [&payload],
        )?;
        Ok(())
    }

    /// Preferred API (plan): load kit from JSON snapshot stored in SQLite.
    pub fn load_sqlite(path: &Path) -> Result<KitStoreRef> {
        let conn = Connection::open(path)?;
        let payload: String = conn.query_row(
            "SELECT payload FROM semio_kit_snapshot WHERE id = 1",
            [],
            |r| r.get(0),
        )?;
        KitStore::from_json_str(&payload)
    }

    /// Back-compat alias for [`KitStore::load_sqlite`].
    pub fn from_sqlite(path: &Path) -> Result<KitStoreRef> {
        Self::load_sqlite(path)
    }

    /// Back-compat alias for [`KitStore::save_sqlite`].
    pub fn to_sqlite(&self, path: &Path) -> Result<()> {
        self.save_sqlite(path)
    }
}
