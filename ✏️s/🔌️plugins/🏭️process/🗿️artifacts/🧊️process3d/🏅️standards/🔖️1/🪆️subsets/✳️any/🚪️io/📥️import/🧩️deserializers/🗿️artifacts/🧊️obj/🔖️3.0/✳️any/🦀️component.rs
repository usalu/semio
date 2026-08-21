//! process3d <- obj
use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &ObjSnapshot) -> Result<Process3dSnapshot, store::TextError> {
    let _ = STDIO_OBJ_DOCUMENT_SCHEMA;
    let bytes = <ObjSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Process3dSnapshot, store::TextError> {
    <Process3dSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes)))
}
