//! din4108 -> zip
use crate::artifacts::din4108::Din4108Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Din4108Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Din4108Snapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
