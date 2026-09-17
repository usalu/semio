//! lowpoly <- stl
//!
//! Real STL geometry import: ASCII (`engine::decode_stl_ascii`) or binary
//! (`engine::decode_stl_binary`), picked by content — binary when the 84-byte header's triangle
//! count exactly frames the file length, ASCII when the text starts with `solid`. Triangles are
//! welded on bit-identical vertex positions (STL has no shared-vertex topology) and become ONE
//! lowpoly object named after the `solid` (or "STL Mesh"). Degenerate triangles (two corners
//! welded together) are dropped; a file with no usable triangle is rejected loudly.
use crate::io::mesh_geometry::{snapshot_from_parts, text_error, PolygonPart};
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_stl::engine::{decode_stl_ascii, decode_stl_binary};
use semio_s_artifact_stdio_stl::StlSnapshot;

pub fn register() {}

pub fn deserialize(from: &StlSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let mut part = PolygonPart { name: if from.solid_name.trim().is_empty() { "STL Mesh".into() } else { from.solid_name.trim().to_string() }, ..Default::default() };
    let mut welded: std::collections::HashMap<[u32; 3], u32> = std::collections::HashMap::new();
    for triangle in &from.triangles {
        let mut face = [0u32; 3];
        for (corner, vertex) in triangle.vertices.iter().enumerate() {
            let position = [vertex[0] as f32, vertex[1] as f32, vertex[2] as f32];
            // ➕️ `+ 0.0` folds -0.0 into 0.0 so both weld together.
            let key = [(position[0] + 0.0).to_bits(), (position[1] + 0.0).to_bits(), (position[2] + 0.0).to_bits()];
            face[corner] = *welded.entry(key).or_insert_with(|| {
                part.positions.push(position);
                (part.positions.len() - 1) as u32
            });
        }
        if face[0] != face[1] && face[1] != face[2] && face[0] != face[2] {
            part.faces.push(face.to_vec());
        }
    }
    if part.faces.is_empty() {
        return Err(text_error("stl->lowpoly: the file contains no non-degenerate triangles to import"));
    }
    snapshot_from_parts("stl", vec![part])
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let binary_framed = bytes.len() >= 84 && (u32::from_le_bytes([bytes[80], bytes[81], bytes[82], bytes[83]]) as usize).checked_mul(50).and_then(|body| body.checked_add(84)) == Some(bytes.len());
    let snap = if binary_framed {
        decode_stl_binary(bytes)
    } else {
        match std::str::from_utf8(bytes) {
            Ok(text) if text.trim_start().starts_with("solid") => decode_stl_ascii(text.trim_start()),
            _ => decode_stl_binary(bytes),
        }
    }
    .map_err(|e| text_error(format!("stl->lowpoly: {e}")))?;
    deserialize(&snap)
}
