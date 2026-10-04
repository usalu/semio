//! 🏔️ gisterrain → las — the terrain's composed surface mesh (`gis_terrain_mesh_from_snapshot`) as
//! LAS 1.x: every surface vertex becomes one point record, written by `s.stdio.semio/v1/mesh`'s own export leaf.
//!
//! 🔖 `IoFidelity::Lossy`: the file carries the surface geometry, not the terrain document
//! (`exaggeration` and the imported feature overlay are not part of any mesh format).
use crate::{gis_terrain_mesh_from_snapshot, GisTerrainSnapshot};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::{encode_mesh, SemioMeshFormat};

pub fn register() {}

pub fn serialize_bytes(snapshot: &GisTerrainSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    encode_mesh(&gis_terrain_mesh_from_snapshot(snapshot), SemioMeshFormat::Las).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("gisterrain→las: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}
