//! gisterrain <- gltf
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use semio_s_plugin_stdio::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &GltfSnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = (STDIO_GLTF_DOCUMENT_SCHEMA, from);
    Ok(GisTerrainSnapshot::default())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let _ = bytes;
    Ok(GisTerrainSnapshot::default())
}
