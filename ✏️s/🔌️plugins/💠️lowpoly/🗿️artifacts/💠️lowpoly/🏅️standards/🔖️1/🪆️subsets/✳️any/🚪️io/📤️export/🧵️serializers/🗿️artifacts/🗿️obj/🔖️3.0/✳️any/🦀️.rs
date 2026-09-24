//! lowpoly -> obj
//!
//! Writes real Wavefront OBJ geometry through `engine::encode_obj` (never a second bespoke
//! grammar): every object whose persisted `mesh_content` is non-empty contributes an `o <name>`
//! block of world-space `v` positions (scale → Euler-degree rotation → translation) and its
//! original n-gon `f` polygons. Object names are made single-token (OBJ readers, including
//! `decode_obj`, keep only the first token of `o`) and unique (same-named `o` blocks merge on
//! read).
//!
//! 🔖 `IoFidelity::Lossy`: geometry and object names survive; paint, materials and the object
//! transforms (already applied to the positions) do not.
use crate::io::mesh_geometry::world_parts;
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_obj::engine::encode_obj;
use semio_s_artifact_stdio_obj::schema::snapshot::{ObjFace, ObjFaceVertex, ObjObject, ObjVertex};
use semio_s_artifact_stdio_obj::ObjSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<ObjSnapshot, store::TextError> {
    let mut obj = ObjSnapshot::default();
    let mut used_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    for part in world_parts("obj", snapshot)? {
        let base: String = part.name.split_whitespace().collect::<Vec<_>>().join("_");
        let base = if base.is_empty() { "Object".to_string() } else { base };
        let mut name = base.clone();
        let mut suffix = 2;
        while !used_names.insert(name.clone()) {
            name = format!("{base}.{suffix}");
            suffix += 1;
        }
        let offset = obj.vertices.len() as u32;
        obj.vertices.extend(part.positions.iter().map(|p| ObjVertex { x: p[0], y: p[1], z: p[2], w: None }));
        let mut object = ObjObject { name, faces: Vec::with_capacity(part.faces.len()) };
        for face in &part.faces {
            object.faces.push(obj.faces.len());
            obj.faces.push(ObjFace { vertices: face.iter().map(|&v| ObjFaceVertex { vertex: v + offset, texcoord: None, normal: None }).collect() });
        }
        obj.objects.push(object);
    }
    Ok(obj)
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(encode_obj(&serialize(snapshot)?).into_bytes())
}
