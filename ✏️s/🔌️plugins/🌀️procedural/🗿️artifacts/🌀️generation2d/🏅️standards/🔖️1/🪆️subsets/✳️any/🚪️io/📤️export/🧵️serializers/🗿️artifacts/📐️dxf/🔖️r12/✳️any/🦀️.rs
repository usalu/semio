//! generation2d -> dxf
use crate::artifacts::generation2d::Generation2dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Generation2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Generation2dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
