//! process3d <- dwg
use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::{DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &DwgSnapshot) -> Result<Process3dSnapshot, store::TextError> {
    let _ = STDIO_DWG_DOCUMENT_SCHEMA;
    let bytes = <DwgSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Process3dSnapshot, store::TextError> {
    <Process3dSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes)))
}
