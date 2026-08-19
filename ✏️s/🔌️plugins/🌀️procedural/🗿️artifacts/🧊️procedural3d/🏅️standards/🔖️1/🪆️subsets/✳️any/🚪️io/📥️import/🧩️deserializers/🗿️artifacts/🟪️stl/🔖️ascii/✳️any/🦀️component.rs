//! procedural3d <- stl
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &StlSnapshot) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = (STDIO_STL_DOCUMENT_SCHEMA, from);
    Ok(Procedural3dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Procedural3dSnapshot::default())
}
