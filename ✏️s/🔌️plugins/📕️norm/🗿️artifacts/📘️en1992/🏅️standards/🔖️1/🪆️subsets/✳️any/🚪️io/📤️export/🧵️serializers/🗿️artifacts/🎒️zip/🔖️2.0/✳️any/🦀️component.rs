//! en1992 -> zip
use crate::artifacts::en1992::En1992Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1992Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1992Snapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
