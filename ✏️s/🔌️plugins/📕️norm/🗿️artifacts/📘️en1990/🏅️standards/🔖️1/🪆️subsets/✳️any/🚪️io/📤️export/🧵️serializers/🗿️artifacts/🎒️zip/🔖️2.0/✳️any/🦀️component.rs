//! en1990 -> zip
use crate::artifacts::en1990::En1990Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1990Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1990Snapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
