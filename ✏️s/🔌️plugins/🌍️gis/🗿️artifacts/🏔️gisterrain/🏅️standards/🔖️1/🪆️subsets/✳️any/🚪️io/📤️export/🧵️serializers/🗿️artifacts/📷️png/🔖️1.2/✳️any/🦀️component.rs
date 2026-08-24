//! gisterrain -> png
use crate::artifacts::gisterrain::GisTerrainSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &GisTerrainSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<GisTerrainSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
