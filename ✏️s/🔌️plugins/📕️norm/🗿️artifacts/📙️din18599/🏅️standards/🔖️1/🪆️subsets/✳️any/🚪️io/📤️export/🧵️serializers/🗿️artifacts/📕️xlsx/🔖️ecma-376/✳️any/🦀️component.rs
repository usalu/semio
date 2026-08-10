//! din18599 -> xlsx
use crate::artifacts::din18599::Din18599Snapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Din18599Snapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Din18599Snapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
