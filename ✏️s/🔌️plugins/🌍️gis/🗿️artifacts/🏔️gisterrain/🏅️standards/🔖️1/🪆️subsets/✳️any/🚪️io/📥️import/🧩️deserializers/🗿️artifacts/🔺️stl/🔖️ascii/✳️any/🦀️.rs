//! gisterrain <- stl
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_s_artifact_stdio_stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &StlSnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = (STDIO_STL_DOCUMENT_SCHEMA, from);
    Ok(GisTerrainSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisTerrainSnapshot::default())
}
