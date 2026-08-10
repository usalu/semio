//! en1991 -> xlsx
use crate::artifacts::en1991::En1991Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1991Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1991Snapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
