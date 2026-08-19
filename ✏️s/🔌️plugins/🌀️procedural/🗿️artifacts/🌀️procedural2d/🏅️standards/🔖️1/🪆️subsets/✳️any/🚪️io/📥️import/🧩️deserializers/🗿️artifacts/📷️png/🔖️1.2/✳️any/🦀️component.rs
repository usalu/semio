//! procedural2d <- png
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PngSnapshot) -> Result<Procedural2dSnapshot, store::TextError> {
    let _ = (STDIO_PNG_DOCUMENT_SCHEMA, from);
    Ok(Procedural2dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Procedural2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Procedural2dSnapshot::default())
}
