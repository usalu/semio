//! lowpoly <- ply
//!
//! Two paths over the real `engine::decode_ply` (ascii and both binary encodings):
//! 1. 🔒️ Lossless: the export leaf's hex-embedded lowpoly DSL `comment` is read back when present.
//! 2. 🕸️ Geometry: otherwise the `vertex` element's `x`/`y`/`z` columns plus the `face` element's
//!    `vertex_indices` (or `vertex_index`) list become ONE lowpoly object, n-gons kept. A file
//!    without faces is rejected loudly.
use crate::io::mesh_geometry::{compact_part, snapshot_from_parts, text_error};
use crate::schema::snapshot::text::parse_dsl;
use crate::schema::snapshot::{dec_str, LowpolySnapshot};
use semio_s_artifact_stdio_ply::engine::decode_ply;
use semio_s_artifact_stdio_ply::schema::snapshot::{PlyElement, PlyValue};
use semio_s_artifact_stdio_ply::PlySnapshot;

pub fn register() {}

pub fn deserialize(from: &PlySnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let prefix = crate::io::export::serializers::artifacts::ply::v1_0::any::LOWPOLY_DSL_COMMENT_PREFIX;
    if let Some(hex) = from.comments.iter().find_map(|c| c.strip_prefix(prefix)) {
        let text = dec_str(hex).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
        return parse_dsl(&text);
    }
    snapshot_from_ply_geometry(from)
}

fn scalar_f64(value: &PlyValue) -> Option<f64> {
    Some(match value {
        PlyValue::Char(v) => f64::from(*v),
        PlyValue::UChar(v) => f64::from(*v),
        PlyValue::Short(v) => f64::from(*v),
        PlyValue::UShort(v) => f64::from(*v),
        PlyValue::Int(v) => f64::from(*v),
        PlyValue::UInt(v) => f64::from(*v),
        PlyValue::Float(v) => f64::from(*v),
        PlyValue::Double(v) => *v,
        PlyValue::List(_) => return None,
    })
}

fn column(element: &PlyElement, names: &[&str]) -> Option<usize> {
    element.properties.iter().position(|p| names.contains(&p.name()))
}

/// 🕸️ Real-geometry import (see module docs).
pub fn snapshot_from_ply_geometry(from: &PlySnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let vertex = from.elements.iter().find(|e| e.name == "vertex").ok_or_else(|| text_error("ply->lowpoly: no `vertex` element"))?;
    let face = from.elements.iter().find(|e| e.name == "face").ok_or_else(|| text_error("ply->lowpoly: no `face` element (point clouds are not meshes)"))?;
    let (Some(x), Some(y), Some(z)) = (column(vertex, &["x"]), column(vertex, &["y"]), column(vertex, &["z"])) else {
        return Err(text_error("ply->lowpoly: `vertex` element lacks x/y/z properties"));
    };
    let indices = column(face, &["vertex_indices", "vertex_index"]).ok_or_else(|| text_error("ply->lowpoly: `face` element lacks a vertex_indices/vertex_index list"))?;

    let mut positions = Vec::with_capacity(vertex.rows.len());
    for (row_index, row) in vertex.rows.iter().enumerate() {
        let read = |col: usize| row.values.get(col).and_then(scalar_f64).map(|v| v as f32).ok_or_else(|| text_error(format!("ply->lowpoly: vertex {row_index} has a non-scalar coordinate")));
        positions.push([read(x)?, read(y)?, read(z)?]);
    }
    let mut polygons = Vec::with_capacity(face.rows.len());
    for (row_index, row) in face.rows.iter().enumerate() {
        let Some(PlyValue::List(items)) = row.values.get(indices) else {
            return Err(text_error(format!("ply->lowpoly: face {row_index} vertex index list is missing")));
        };
        let mut polygon = Vec::with_capacity(items.len());
        for item in items {
            let value = scalar_f64(item).filter(|v| *v >= 0.0 && v.fract() == 0.0).ok_or_else(|| text_error(format!("ply->lowpoly: face {row_index} has an invalid vertex index")))?;
            polygon.push(value as u32);
        }
        polygons.push(polygon);
    }
    let part = compact_part("PLY Mesh", &positions, &polygons).map_err(|e| text_error(format!("ply->lowpoly: {e}")))?;
    snapshot_from_parts("ply", vec![part])
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let snap = decode_ply(bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    deserialize(&snap)
}
