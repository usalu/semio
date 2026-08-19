//! gisterrain <- las
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_s_plugin_stdio::artifacts::las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &LasSnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = (STDIO_LAS_DOCUMENT_SCHEMA, from);
    Ok(GisTerrainSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisTerrainSnapshot::default())
}
