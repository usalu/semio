//! 🏔️ gisterrain → gltf — the terrain's composed surface mesh (`gis_terrain_mesh_from_snapshot`) as
//! glTF 2.0 JSON with the geometry buffer embedded as a data URI, written by `s.stdio.semio/v1/mesh`'s own export leaf.
//!
//! 🔖 `IoFidelity::Lossy`: the file carries the surface geometry, not the terrain document
//! (`exaggeration` and the imported feature overlay are not part of any mesh format).
use crate::{gis_terrain_mesh_from_snapshot, GisTerrainSnapshot};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::{encode_mesh, SemioMeshFormat};

pub fn register() {}

pub fn serialize_bytes(snapshot: &GisTerrainSnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_mesh(&gis_terrain_mesh_from_snapshot(snapshot), SemioMeshFormat::Gltf).map_err(|error| store::TextError::new(format!("gisterrain→gltf: {error}"), dsl::TextSpan::at(1, 1)))
}
