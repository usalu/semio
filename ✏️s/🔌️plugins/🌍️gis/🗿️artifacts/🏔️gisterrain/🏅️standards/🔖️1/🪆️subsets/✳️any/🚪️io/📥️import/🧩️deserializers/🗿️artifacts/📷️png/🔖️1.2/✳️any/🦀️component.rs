//! gisterrain <- png
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PngSnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = (STDIO_PNG_DOCUMENT_SCHEMA, from);
    Ok(GisTerrainSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisTerrainSnapshot::default())
}
