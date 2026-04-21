//! SQLite backend stub. The previous implementation was removed during the
//! pointer-graph rewrite; re-introducing it will simply require `impl Kit`
//! methods that map DTOs to rows, using the shared schema under `semio/sqlite`.

use crate::error::{Result, SemioError};
use crate::kit::{Kit, KitRef};

impl Kit {
    /// Load a kit from a SQLite database file.
    pub fn from_sqlite(_path: &std::path::Path) -> Result<KitRef> {
        Err(SemioError::InvalidOperation(
            "Kit::from_sqlite is not yet implemented in the OO rewrite".into(),
        ))
    }

    /// Store this kit to a SQLite database file.
    pub fn to_sqlite(&self, _path: &std::path::Path) -> Result<()> {
        Err(SemioError::InvalidOperation(
            "Kit::to_sqlite is not yet implemented in the OO rewrite".into(),
        ))
    }
}
