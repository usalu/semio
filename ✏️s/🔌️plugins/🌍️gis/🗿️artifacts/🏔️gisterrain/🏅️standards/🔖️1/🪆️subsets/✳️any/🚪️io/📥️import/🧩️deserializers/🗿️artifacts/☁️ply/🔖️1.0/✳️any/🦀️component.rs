//! gisterrain <- ply
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_s_plugin_stdio::artifacts::ply::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PlySnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = (STDIO_PLY_DOCUMENT_SCHEMA, from);
    Ok(GisTerrainSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisTerrainSnapshot::default())
}
