//! procedural3d -> dwg
use crate::artifacts::procedural3d::Procedural3dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Procedural3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Procedural3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
