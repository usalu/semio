# Owned OBJ and PLY Syntax Constructor Binding

Two ordinary Lowpoly TextError mappings now explicitly author InvalidValue after the complete owned producers and their fallible helpers were read. OBJ decode rejects literal grammar/indices/face shapes; PLY encode rejects missing declared property values or non-list list values. Neither producer returns controlled cancellation or quotas. Original 1:1 spans and exact messages are retained. This does not claim the ordinary synchronous codecs are controlled, nor that a higher native package has passed.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-obj-ply-owned-syntax-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs

```rust
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

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<PlySnapshot, semio_framework_diagnostic::TextError> {
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
            count: u64::try_from(vertex_rows.len()).map_err(|_| crate::io::mesh_geometry::text_error("PLY vertex occurrence count exceeds u64"))?,
            properties: ["x", "y", "z"].iter().map(|n| PlyProperty::Scalar { name: (*n).into(), kind: PlyScalarType::Float }).collect(),
            rows: vertex_rows,
        });
        ply.elements.push(PlyElement { name: "face".into(), count: u64::try_from(face_rows.len()).map_err(|_| crate::io::mesh_geometry::text_error("PLY face occurrence count exceeds u64"))?, properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind, value_kind: PlyScalarType::Int }], rows: face_rows });
    }
    Ok(ply)
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    encode_ply(&serialize(snapshot)?).map_err(|e| semio_framework_diagnostic::TextError::new(e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}

```

## ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs

```rust
//! lowpoly <- obj
//!
//! Geometry over the real `engine::decode_obj` grammar (never a second bespoke parser): any OBJ
//! (Blender, etc.) becomes real lowpoly objects — one per `o`
//!    object (falling back to `g` groups, then a single object), names from the file, vertex
//!    indices remapped per object, n-gons kept as n-gons. More than 64 parts merge into one
//!    object; a file without faces is rejected loudly.
use crate::io::mesh_geometry::{compact_part, snapshot_from_parts, text_error, PolygonPart};
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_obj::engine::decode_obj;
use semio_s_artifact_stdio_obj::ObjSnapshot;

pub fn register() {}

pub fn deserialize(from: &ObjSnapshot) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    snapshot_from_obj_geometry(from)
}

/// 🕸️ Real-geometry import (see module docs).
pub fn snapshot_from_obj_geometry(from: &ObjSnapshot) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    let positions: Vec<[f32; 3]> = from.vertices.iter().map(|v| [v.x as f32, v.y as f32, v.z as f32]).collect();
    let polygon = |face_index: usize| -> Vec<u32> { from.faces[face_index].vertices.iter().map(|fv| fv.vertex).collect() };

    // 🏷️ Face ownership: `o` objects first, else the first `g` group naming the face.
    let named: Vec<(String, &Vec<u64>)> = if !from.objects.is_empty() {
        from.objects.iter().map(|o| (o.name.clone(), &o.faces)).collect()
    } else {
        from.groups.iter().map(|g| (g.name.clone(), &g.faces)).collect()
    };
    let mut owner: Vec<Option<usize>> = vec![None; from.faces.len()];
    for (part_index, (_, faces)) in named.iter().enumerate() {
        for &face in faces.iter() {
            if let Some(slot) = usize::try_from(face).ok().and_then(|index| owner.get_mut(index)) {
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

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let snap = decode_obj(text).map_err(|e| semio_framework_diagnostic::TextError::new(e, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    deserialize(&snap)
}

```

