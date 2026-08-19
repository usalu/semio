//! procedural3d -> png
use crate::artifacts::procedural3d::Procedural3dSnapshot;

pub async fn register() {}

pub async fn serialize_bytes(snapshot: &Procedural3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Procedural3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
