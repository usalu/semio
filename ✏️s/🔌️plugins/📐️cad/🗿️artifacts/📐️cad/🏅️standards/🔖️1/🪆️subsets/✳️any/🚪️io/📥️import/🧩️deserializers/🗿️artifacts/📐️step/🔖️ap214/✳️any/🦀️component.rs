//! Deserialize cad via stdio.step.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::{cad_from_wire, pack_err_as_text};
use semio_s_plugin_stdio::artifacts::step::engine::brep::analyze_brep_mesh;
use semio_s_plugin_stdio::artifacts::step::{StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};

//#region Deserialize
pub fn register() {}

pub fn deserialize(from: &StepSnapshot) -> Result<CadSnapshot, store::TextError> {
    let _ = STDIO_STEP_DOCUMENT_SCHEMA;
    let mesh = analyze_brep_mesh(&from.to_part21_document()).mesh;
    let mut bytes = Vec::with_capacity(mesh.vertices.len() * 12);
    for v in &mesh.vertices {
        bytes.extend_from_slice(&(v.x as f32).to_le_bytes());
        bytes.extend_from_slice(&(v.y as f32).to_le_bytes());
        bytes.extend_from_slice(&(v.z as f32).to_le_bytes());
    }
    cad_from_wire(&bytes).map_err(pack_err_as_text)
}

pub fn deserialize_text(text: &str) -> Result<CadSnapshot, store::TextError> {
    <CadSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
//#endregion Deserialize
