//! en1998 -> xlsx
use crate::artifacts::en1998::En1998Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1998Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1998Snapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
