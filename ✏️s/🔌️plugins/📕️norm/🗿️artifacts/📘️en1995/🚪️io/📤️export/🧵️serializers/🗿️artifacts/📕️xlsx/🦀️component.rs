//! en1995 -> xlsx
use crate::artifacts::en1995::En1995Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1995Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1995Snapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
