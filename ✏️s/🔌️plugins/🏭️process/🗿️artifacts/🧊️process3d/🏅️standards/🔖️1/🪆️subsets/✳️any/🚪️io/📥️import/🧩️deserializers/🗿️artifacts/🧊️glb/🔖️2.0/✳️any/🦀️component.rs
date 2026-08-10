//! process3d <- glb
use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GlbSnapshot) -> Result<Process3dSnapshot, store::TextError> {
    let _ = STDIO_GLB_DOCUMENT_SCHEMA;
    let bytes = <GlbSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Process3dSnapshot, store::TextError> {
    <Process3dSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| {
        <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
