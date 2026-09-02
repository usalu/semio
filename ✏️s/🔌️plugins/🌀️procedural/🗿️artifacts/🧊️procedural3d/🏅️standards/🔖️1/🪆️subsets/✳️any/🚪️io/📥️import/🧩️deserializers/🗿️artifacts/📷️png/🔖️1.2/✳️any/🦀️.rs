//! procedural3d <- png
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PngSnapshot) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = (STDIO_PNG_DOCUMENT_SCHEMA, from);
    Ok(Procedural3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Procedural3dSnapshot::default())
}
