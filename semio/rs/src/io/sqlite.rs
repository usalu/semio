//! SQLite backend stub. The previous implementation was removed during the
//! pointer-graph rewrite; re-introducing it will simply require `impl KitStore`
//! methods that map DTOs to rows, using the shared schema under `semio/sqlite`.

use crate::error::{Result, SemioError};
use crate::kit::{KitStore, KitStoreRef};

impl KitStore {
    /// Load a kit from a SQLite database file.
    pub fn from_sqlite(_path: &std::path::Path) -> Result<KitStoreRef> {
        Err(SemioError::InvalidOperation(
            "KitStore::from_sqlite is not yet implemented in the OO rewrite".into(),
        ))
    }

    /// Store this kit to a SQLite database file.
    pub fn to_sqlite(&self, _path: &std::path::Path) -> Result<()> {
        Err(SemioError::InvalidOperation(
            "KitStore::to_sqlite is not yet implemented in the OO rewrite".into(),
        ))
    }
}
