//! gismap <- png
use crate::artifacts::gismap::GisMapSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PngSnapshot) -> Result<GisMapSnapshot, store::TextError> {
    let _ = (STDIO_PNG_DOCUMENT_SCHEMA, from);
    Ok(GisMapSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<GisMapSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisMapSnapshot::default())
}
