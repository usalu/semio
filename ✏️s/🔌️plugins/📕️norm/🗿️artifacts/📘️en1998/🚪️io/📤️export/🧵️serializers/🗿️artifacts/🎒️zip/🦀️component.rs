//! en1998 -> zip
use crate::artifacts::en1998::En1998Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1998Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1998Snapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
