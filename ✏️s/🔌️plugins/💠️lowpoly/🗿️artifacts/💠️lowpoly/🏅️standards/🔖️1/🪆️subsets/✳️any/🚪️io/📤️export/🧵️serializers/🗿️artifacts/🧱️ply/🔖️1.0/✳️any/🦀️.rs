//! lowpoly -> ply
//!
//! Real ASCII PLY export through `engine::encode_ply`: a `vertex` element (`float x/y/z`, world
//! space — scale → Euler-degree rotation → translation) and a `face` element
//! (`list uchar int vertex_indices`, widened to a `uint` count past 255 corners) holding every
//! object's original n-gons, concatenated. Objects with empty `mesh_content` contribute nothing.
//!
//! 🔖 `IoFidelity::Lossy`: geometry survives with n-gons kept; object boundaries, names and paint
//! do not.
use crate::io::mesh_geometry::world_parts;
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_ply::engine::encode_ply;
use semio_s_artifact_stdio_ply::schema::snapshot::{PlyElement, PlyProperty, PlyRow, PlyScalarType, PlyValue};
use semio_s_artifact_stdio_ply::PlySnapshot;

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<PlySnapshot, store::TextError> {
    let mut vertex_rows = Vec::new();
    let mut face_rows = Vec::new();
    let mut max_corners = 0usize;
    for part in world_parts("ply", snapshot)? {
        let offset = vertex_rows.len() as u32;
        vertex_rows.extend(part.positions.iter().map(|p| PlyRow { values: vec![PlyValue::Float(p[0] as f32), PlyValue::Float(p[1] as f32), PlyValue::Float(p[2] as f32)] }));
        for face in &part.faces {
            max_corners = max_corners.max(face.len());
            face_rows.push(PlyRow { values: vec![PlyValue::List(face.iter().map(|&v| PlyValue::Int((v + offset) as i32)).collect())] });
        }
    }
    let mut ply = PlySnapshot::default();
    if !vertex_rows.is_empty() {
        let count_kind = if max_corners > 255 { PlyScalarType::UInt } else { PlyScalarType::UChar };
        ply.elements.push(PlyElement {
            name: "vertex".into(),
            count: vertex_rows.len(),
            properties: ["x", "y", "z"].iter().map(|n| PlyProperty::Scalar { name: (*n).into(), kind: PlyScalarType::Float }).collect(),
            rows: vertex_rows,
        });
        ply.elements.push(PlyElement { name: "face".into(), count: face_rows.len(), properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind, value_kind: PlyScalarType::Int }], rows: face_rows });
    }
    Ok(ply)
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_ply(&serialize(snapshot)?).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
