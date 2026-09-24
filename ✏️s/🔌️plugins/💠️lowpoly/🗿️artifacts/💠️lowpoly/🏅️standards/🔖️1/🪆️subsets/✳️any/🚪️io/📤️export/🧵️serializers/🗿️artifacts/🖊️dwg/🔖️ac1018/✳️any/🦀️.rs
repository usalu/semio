//! lowpoly -> dwg
//!
//! Real DWG export: mesh vertices/faces (world space, fan-triangulated) go through
//! `mesh_to_dwg_drawing` -> `DwgLogicalDrawing::from_native` -> `encode_dwg` (AC1015).
//!
//! 🔖 `IoFidelity::Lossy`: one polyface mesh of triangles — objects, names and paint do not survive.
use crate::io::mesh_geometry::world_parts;
use crate::schema::snapshot::LowpolySnapshot;
use semio_framework_plugin::MeshData;
use semio_s_artifact_stdio_dwg::schema::snapshot::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg::{dwg_to_bytes, mesh_to_dwg_drawing, DwgSnapshot};

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<DwgSnapshot, store::TextError> {
    let mut mesh = MeshData::default();
    for part in world_parts("dwg", snapshot)? {
        let base = (mesh.positions.len() / 3) as u32;
        for p in &part.positions {
            mesh.positions.extend_from_slice(&[p[0] as f32, p[1] as f32, p[2] as f32]);
        }
        for face in &part.faces {
            if face.len() < 3 {
                continue;
            }
            for i in 1..face.len() - 1 {
                mesh.indices.extend_from_slice(&[base + face[0], base + face[i], base + face[i + 1]]);
            }
        }
    }
    let drawing = mesh_to_dwg_drawing(&mesh);
    let logical = DwgLogicalDrawing::from_native(&drawing).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    Ok(DwgSnapshot { version: "AC1015".into(), drawing: logical, ..Default::default() })
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    let snap = serialize(snapshot)?;
    let drawing = snap.drawing.to_native().map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    dwg_to_bytes(&drawing).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
