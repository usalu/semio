//! en1997 -> xlsx
use crate::artifacts::en1997::En1997Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &En1997Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<En1997Snapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
