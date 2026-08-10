//! procedural3d <- glb
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GlbSnapshot) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = (STDIO_GLB_DOCUMENT_SCHEMA, from);
    Ok(Procedural3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Procedural3dSnapshot::default())
}
