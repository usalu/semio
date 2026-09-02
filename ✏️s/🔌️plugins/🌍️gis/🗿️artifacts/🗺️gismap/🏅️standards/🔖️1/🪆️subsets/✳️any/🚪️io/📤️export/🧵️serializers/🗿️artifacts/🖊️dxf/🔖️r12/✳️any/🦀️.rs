//! gismap -> dxf
use crate::artifacts::gismap::GisMapSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &GisMapSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<GisMapSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
