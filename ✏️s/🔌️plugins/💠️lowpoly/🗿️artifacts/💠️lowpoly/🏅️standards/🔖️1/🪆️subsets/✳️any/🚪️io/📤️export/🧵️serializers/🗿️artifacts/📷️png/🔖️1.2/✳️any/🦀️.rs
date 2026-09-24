//! 💠️ lowpoly -> png — a shaded isometric picture of the model: every object's world-space n-gons
//! (`world_parts`, fan-triangulated) become one `s.stdio.semio/v1/mesh` painted by that subset's own
//! view leaf. Lowpoly is y-up and the view is z-up, so `(x, y, z)` is drawn as `(x, -z, y)`.
//!
//! 🔖 `IoFidelity::Lossy`: a picture, not the model — there is no png import.
use crate::io::mesh_geometry::world_parts;
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::{encode_mesh, SemioMeshFormat};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology};

pub fn register() {}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    let meshes = world_parts("png", snapshot)?
        .into_iter()
        .map(|part| SemioMesh {
            id: part.name.clone(),
            primitives: vec![SemioPrimitive {
                id: part.name,
                topology: SemioTopology::Triangles,
                positions: part.positions.iter().map(|p| SemioPoint3 { x: p[0], y: -p[2], z: p[1] }).collect(),
                normals: Vec::new(),
                uvs: Vec::new(),
                colors: Vec::new(),
                indices: part.faces.iter().filter(|face| face.len() >= 3).flat_map(|face| (1..face.len() - 1).flat_map(move |i| [face[0], face[i], face[i + 1]])).collect(),
                material_id: None,
            }],
        })
        .collect();
    encode_mesh(&SemioMeshSnapshot { meshes, ..SemioMeshSnapshot::default() }, SemioMeshFormat::Png).map_err(|error| store::TextError::new(format!("lowpoly->png: {error}"), dsl::TextSpan::at(1, 1)))
}
