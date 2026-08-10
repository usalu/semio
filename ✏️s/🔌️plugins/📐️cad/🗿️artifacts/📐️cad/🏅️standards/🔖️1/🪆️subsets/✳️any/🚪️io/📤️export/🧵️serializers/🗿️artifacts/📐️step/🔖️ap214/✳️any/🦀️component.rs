//! Serialize cad to stdio.step.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::cad_to_wire;
use semio_s_plugin_stdio::artifacts::step::engine::brep::{brep_mesh_to_part21, BrepMesh, BrepVertex};
use semio_s_plugin_stdio::artifacts::step::{StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(from: &CadSnapshot) -> Result<StepSnapshot, store::PackError> {
    let raw = cad_to_wire(from);
    let mut vertices = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        let mut chunk = [0u8; 12];
        let n = (raw.len() - i).min(12);
        chunk[..n].copy_from_slice(&raw[i..i + n]);
        vertices.push(BrepVertex {
            x: f64::from(f32::from_le_bytes(chunk[0..4].try_into().unwrap())),
            y: f64::from(f32::from_le_bytes(chunk[4..8].try_into().unwrap())),
            z: f64::from(f32::from_le_bytes(chunk[8..12].try_into().unwrap())),
        });
        i += 12;
    }
    let document = brep_mesh_to_part21(&BrepMesh { vertices, faces: Vec::new() });
    Ok(StepSnapshot { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), document })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
