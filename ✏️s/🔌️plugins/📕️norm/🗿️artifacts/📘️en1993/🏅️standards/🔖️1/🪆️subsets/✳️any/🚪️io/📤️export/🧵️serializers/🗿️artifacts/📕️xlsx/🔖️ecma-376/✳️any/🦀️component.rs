//! en1993 -> xlsx
use crate::artifacts::en1993::En1993Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1993Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1993Snapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
