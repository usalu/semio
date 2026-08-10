//! Deserialize cad via stdio.gltf.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::{cad_from_wire, pack_err_as_text};
use semio_s_plugin_stdio::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};

//#region Deserialize
pub fn register() {}

pub fn deserialize(from: &GltfSnapshot) -> Result<CadSnapshot, store::TextError> {
    let _ = STDIO_GLTF_DOCUMENT_SCHEMA;
    serde_json::from_value(from.document.clone()).map_err(|e| store::TextError::new(format!("cad<-gltf: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
//#endregion Deserialize
