//! generation3d -> gltf
use crate::artifacts::generation3d::Generation3dSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Generation3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
