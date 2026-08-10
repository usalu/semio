//! Serialize cad to stdio.stl.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::cad_to_wire;
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(from: &CadSnapshot) -> Result<StlSnapshot, store::PackError> {
    let raw = cad_to_wire(from);
    let mut vertices = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        let mut chunk = [0u8; 12];
        let n = (raw.len() - i).min(12);
        chunk[..n].copy_from_slice(&raw[i..i + n]);
        vertices.push(semio_s_plugin_stdio::artifacts::stl::schema::snapshot::MeshVertex {
            x: f32::from_le_bytes(chunk[0..4].try_into().unwrap()),
            y: f32::from_le_bytes(chunk[4..8].try_into().unwrap()),
            z: f32::from_le_bytes(chunk[8..12].try_into().unwrap()),
        });
        i += 12;
    }
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices, faces: Vec::new() })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::DocumentDsl>::print_dsl(from))
}
//#endregion Serialize
