//! generation3d <- ply
use crate::artifacts::generation3d::Generation3dSnapshot;
use semio_s_plugin_stdio::artifacts::ply::{PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PlySnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    let _ = (STDIO_PLY_DOCUMENT_SCHEMA, from);
    Ok(Generation3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Generation3dSnapshot::default())
}
