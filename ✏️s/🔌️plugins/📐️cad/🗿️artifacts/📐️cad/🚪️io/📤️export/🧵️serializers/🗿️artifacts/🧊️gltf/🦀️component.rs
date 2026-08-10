//! Serialize cad to stdio.gltf.

use crate::artifacts::cad::CadSnapshot;
use semio_s_plugin_stdio::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(from: &CadSnapshot) -> Result<GltfSnapshot, store::PackError> {
    let document = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), vertices: Vec::new(), document })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::DocumentDsl>::print_dsl(from))
}
//#endregion Serialize
