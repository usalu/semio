//! procedural2d <- svg
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use semio_s_plugin_stdio::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &SvgSnapshot) -> Result<Procedural2dSnapshot, store::TextError> {
    let _ = (STDIO_SVG_DOCUMENT_SCHEMA, from);
    Ok(Procedural2dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Procedural2dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Procedural2dSnapshot::default())
}
