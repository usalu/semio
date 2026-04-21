//! Zip archive backend stub; the rewrite defers the binary format work until
//! the JSON/SQLite paths have settled.

use crate::error::{Result, SemioError};
use crate::kit::{KitStore, KitStoreRef};

impl KitStore {
    pub fn from_zip(_path: &std::path::Path) -> Result<KitStoreRef> {
        Err(SemioError::InvalidOperation(
            "KitStore::from_zip is not yet implemented in the OO rewrite".into(),
        ))
    }

    pub fn to_zip(&self, _path: &std::path::Path) -> Result<()> {
        Err(SemioError::InvalidOperation(
            "KitStore::to_zip is not yet implemented in the OO rewrite".into(),
        ))
    }
}
