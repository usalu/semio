//! en1994 -> zip
use crate::artifacts::en1994::En1994Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1994Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1994Snapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
