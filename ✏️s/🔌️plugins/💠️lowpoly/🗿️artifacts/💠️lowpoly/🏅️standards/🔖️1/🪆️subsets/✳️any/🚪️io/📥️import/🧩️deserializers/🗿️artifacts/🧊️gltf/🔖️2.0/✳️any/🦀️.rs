//! lowpoly <- gltf
//!
//! Geometry over the real `io::decode_glb` grammar: every mesh primitive with POSITION (+ optional indices) becomes one
//!    lowpoly object; n-gons fan-triangulated on export are re-imported as triangles.
use crate::io::mesh_geometry::{snapshot_from_parts, text_error, PolygonPart};
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_gltf::io::{decode_accessor, decode_glb, GltfAccessorType};
use semio_s_artifact_stdio_gltf::schema::snapshot::GltfSnapshot;

pub fn register() {}

pub fn deserialize(from: &GltfSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let mut parts = Vec::new();
    for (mesh_index, mesh) in from.document.meshes.iter().enumerate() {
        let name = mesh.name.clone().unwrap_or_else(|| format!("Mesh {mesh_index}"));
        let mut part = PolygonPart { name, ..Default::default() };
        for primitive in &mesh.primitives {
            let pos_index = primitive.attributes.iter().find_map(|(k, v)| (k == "POSITION").then_some(*v)).ok_or_else(|| text_error("gltf->lowpoly: primitive missing POSITION accessor"))?;
            let decoded = decode_accessor(&from.document, &from.buffers, pos_index).map_err(|e| text_error(format!("gltf->lowpoly: {e}")))?;
            if decoded.accessor_type != GltfAccessorType::Vec3 {
                return Err(text_error("gltf->lowpoly: POSITION accessor must be VEC3"));
            }
            let base = part.positions.len() as u32;
            for chunk in decoded.components.chunks_exact(3) {
                part.positions.push([chunk[0] as f32, chunk[1] as f32, chunk[2] as f32]);
            }
            if let Some(idx) = primitive.indices {
                let indices = decode_accessor(&from.document, &from.buffers, idx).map_err(|e| text_error(format!("gltf->lowpoly: {e}")))?;
                for tri in indices.components.chunks_exact(3) {
                    part.faces.push(vec![base + tri[0] as u32, base + tri[1] as u32, base + tri[2] as u32]);
                }
            } else {
                let count = decoded.count as u32;
                for i in (0..count).step_by(3) {
                    if i + 2 < count {
                        part.faces.push(vec![base + i, base + i + 1, base + i + 2]);
                    }
                }
            }
        }
        if !part.faces.is_empty() {
            parts.push(part);
        }
    }
    snapshot_from_parts("gltf", parts)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let snap = decode_glb(bytes).map_err(|e| text_error(format!("gltf->lowpoly: {e}")))?;
    deserialize(&snap)
}
