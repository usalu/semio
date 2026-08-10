//! en1996 -> xlsx
use crate::artifacts::en1996::En1996Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1996Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1996Snapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
