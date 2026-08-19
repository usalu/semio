//! gisterrain -> ply
use crate::artifacts::gisterrain::GisTerrainSnapshot;

pub async fn register() {}

pub async fn serialize_bytes(snapshot: &GisTerrainSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<GisTerrainSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
