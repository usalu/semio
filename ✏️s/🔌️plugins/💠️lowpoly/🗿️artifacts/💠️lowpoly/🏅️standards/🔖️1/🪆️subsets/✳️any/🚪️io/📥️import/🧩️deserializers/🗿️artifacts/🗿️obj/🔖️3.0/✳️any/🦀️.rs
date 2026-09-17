//! lowpoly <- obj
//!
//! Two paths, both over the real `engine::decode_obj` grammar (never a second bespoke parser):
//! 1. 🔒️ Lossless: when the export leaf's hex-embedded lowpoly DSL comment
//!    (`LOWPOLY_DSL_COMMENT_PREFIX`) is present, the full document is read back from it.
//! 2. 🕸️ Geometry: any other OBJ (Blender, etc.) becomes real lowpoly objects — one per `o`
//!    object (falling back to `g` groups, then a single object), names from the file, vertex
//!    indices remapped per object, n-gons kept as n-gons. More than 64 parts merge into one
//!    object; a file without faces is rejected loudly.
use crate::io::mesh_geometry::{compact_part, snapshot_from_parts, text_error, PolygonPart};
use crate::schema::snapshot::text::parse_dsl;
use crate::schema::snapshot::{dec_str, LowpolySnapshot};
use semio_s_artifact_stdio_obj::engine::decode_obj;
use semio_s_artifact_stdio_obj::ObjSnapshot;

pub fn register() {}

pub fn deserialize(from: &ObjSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let prefix = crate::io::export::serializers::artifacts::obj::v3_0::any::LOWPOLY_DSL_COMMENT_PREFIX;
    if let Some(hex) = from.unknown_statements.iter().find_map(|u| u.raw.strip_prefix(prefix)) {
        let text = dec_str(hex).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
        return parse_dsl(&text);
    }
    snapshot_from_obj_geometry(from)
}

/// 🕸️ Real-geometry import (see module docs).
pub fn snapshot_from_obj_geometry(from: &ObjSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let positions: Vec<[f32; 3]> = from.vertices.iter().map(|v| [v.x as f32, v.y as f32, v.z as f32]).collect();
    let polygon = |face_index: usize| -> Vec<u32> { from.faces[face_index].vertices.iter().map(|fv| fv.vertex).collect() };

    // 🏷️ Face ownership: `o` objects first, else the first `g` group naming the face.
    let named: Vec<(String, &Vec<usize>)> = if !from.objects.is_empty() {
        from.objects.iter().map(|o| (o.name.clone(), &o.faces)).collect()
    } else {
        from.groups.iter().map(|g| (g.name.clone(), &g.faces)).collect()
    };
    let mut owner: Vec<Option<usize>> = vec![None; from.faces.len()];
    for (part_index, (_, faces)) in named.iter().enumerate() {
        for &face in faces.iter() {
            if let Some(slot) = owner.get_mut(face) {
                if slot.is_none() {
                    *slot = Some(part_index);
                }
            }
        }
    }
    let mut buckets: Vec<Vec<Vec<u32>>> = vec![Vec::new(); named.len()];
    let mut unowned: Vec<Vec<u32>> = Vec::new();
    for (face_index, slot) in owner.iter().enumerate() {
        match slot {
            Some(part_index) => buckets[*part_index].push(polygon(face_index)),
            None => unowned.push(polygon(face_index)),
        }
    }

    let mut parts: Vec<PolygonPart> = Vec::new();
    if !unowned.is_empty() {
        let name = if named.is_empty() { "Imported Mesh" } else { "Default" };
        parts.push(compact_part(name, &positions, &unowned).map_err(|e| text_error(format!("obj->lowpoly: {e}")))?);
    }
    for ((name, _), polygons) in named.iter().zip(buckets.iter()) {
        parts.push(compact_part(name, &positions, polygons).map_err(|e| text_error(format!("obj->lowpoly: {e}")))?);
    }
    snapshot_from_parts("obj", parts)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let snap = decode_obj(text).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;
    deserialize(&snap)
}
