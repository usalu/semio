//! generation3d <- las
use crate::artifacts::generation3d::Generation3dSnapshot;
use semio_s_artifact_stdio_las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &LasSnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    let _ = (STDIO_LAS_DOCUMENT_SCHEMA, from);
    Ok(Generation3dSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Generation3dSnapshot::default())
}
