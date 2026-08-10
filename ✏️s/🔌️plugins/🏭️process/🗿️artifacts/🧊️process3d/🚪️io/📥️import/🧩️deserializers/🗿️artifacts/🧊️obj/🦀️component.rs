//! process3d <- obj
use crate::artifacts::process3d::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &ObjSnapshot) -> Result<Process3dSnapshot, store::TextError> {
    let _ = STDIO_OBJ_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::obj::engine::encode_obj(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <Process3dSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <Process3dSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Process3dSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::obj::engine::decode_obj(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
