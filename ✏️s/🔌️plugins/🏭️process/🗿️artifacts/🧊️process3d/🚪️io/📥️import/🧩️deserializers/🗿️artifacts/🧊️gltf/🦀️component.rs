//! process3d <- gltf
use crate::artifacts::process3d::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GltfSnapshot) -> Result<Process3dSnapshot, store::TextError> {
    let _ = STDIO_GLTF_DOCUMENT_SCHEMA;
    let bytes = semio_s_plugin_stdio::artifacts::gltf::engine::encode_gltf(from)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    <Process3dSnapshot as store::DocumentPack>::decode_pack(&bytes)
        .or_else(|_| <Process3dSnapshot as store::DocumentDsl>::parse_dsl(&String::from_utf8_lossy(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Process3dSnapshot, store::TextError> {
    deserialize(&semio_s_plugin_stdio::artifacts::gltf::engine::decode_gltf(bytes)
        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?)
}
