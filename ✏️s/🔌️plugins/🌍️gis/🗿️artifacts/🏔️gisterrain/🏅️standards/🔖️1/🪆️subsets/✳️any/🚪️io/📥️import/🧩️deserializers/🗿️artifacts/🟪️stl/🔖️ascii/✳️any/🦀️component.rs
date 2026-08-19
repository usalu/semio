//! gisterrain <- stl
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &StlSnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = (STDIO_STL_DOCUMENT_SCHEMA, from);
    Ok(GisTerrainSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisTerrainSnapshot::default())
}
