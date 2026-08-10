//! en1999 -> zip
use crate::artifacts::en1999::En1999Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1999Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1999Snapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
