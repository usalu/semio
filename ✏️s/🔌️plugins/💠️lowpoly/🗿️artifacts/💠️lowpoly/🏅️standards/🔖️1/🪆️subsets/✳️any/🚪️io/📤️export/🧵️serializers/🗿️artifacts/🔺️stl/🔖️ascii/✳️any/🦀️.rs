//! lowpoly -> stl
//!
//! Real ASCII STL export through `engine::encode_stl_ascii`: every object whose persisted
//! `mesh_content` is non-empty is transformed into world space (scale -> Euler-degree rotation ->
//! translation) and each n-gon is fan-triangulated, one facet per triangle with its computed unit
//! face normal.
//!
//! 🔖 `IoFidelity::Lossy`: STL is one triangle soup — object boundaries, names, n-gons, paint and
//! transforms (already applied) do not survive.
use crate::io::mesh_geometry::{triangle_normal, world_parts};
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_stl::engine::encode_stl_ascii;
use semio_s_artifact_stdio_stl::schema::snapshot::StlTriangle;
use semio_s_artifact_stdio_stl::StlSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<StlSnapshot, store::TextError> {
    let mut stl = StlSnapshot { solid_name: "lowpoly".into(), ..Default::default() };
    for part in world_parts("stl", snapshot)? {
        for face in &part.faces {
            let corner = |i: usize| part.positions[face[i] as usize];
            for i in 1..face.len() - 1 {
                let vertices = [corner(0), corner(i), corner(i + 1)];
                stl.triangles.push(StlTriangle { normal: triangle_normal(vertices[0], vertices[1], vertices[2]), vertices });
            }
        }
    }
    Ok(stl)
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(encode_stl_ascii(&serialize(snapshot)?).into_bytes())
}
