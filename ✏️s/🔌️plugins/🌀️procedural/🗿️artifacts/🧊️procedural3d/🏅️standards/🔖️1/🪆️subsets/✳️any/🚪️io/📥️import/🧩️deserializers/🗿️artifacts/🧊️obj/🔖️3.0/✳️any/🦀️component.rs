//! procedural3d <- obj
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &ObjSnapshot) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = (STDIO_OBJ_DOCUMENT_SCHEMA, from);
    Ok(Procedural3dSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Procedural3dSnapshot, store::TextError> {
    let _ = bytes;
    Ok(Procedural3dSnapshot::default())
}
