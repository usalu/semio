//! 🏔️ gisterrain → ply — the terrain's composed surface mesh (`gis_terrain_mesh_from_snapshot`) as
//! PLY: one shared vertex element plus a triangle face list, written by `s.stdio.semio/v1/mesh`'s own export leaf.
//!
//! 🔖 `IoFidelity::Lossy`: the file carries the surface geometry, not the terrain document
//! (`exaggeration` and the imported feature overlay are not part of any mesh format).
use crate::{gis_terrain_mesh_from_snapshot, GisTerrainSnapshot};
use {semio_s_artifact_stdio_semio_mesh_export::encode_mesh, semio_s_artifact_stdio_semio_mesh::io::SemioMeshFormat};

pub fn register() {}

pub fn serialize_bytes(snapshot: &GisTerrainSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    gis_terrain_mesh_from_snapshot(snapshot).map_err(str::to_owned)
        .and_then(|mesh| encode_mesh(&mesh, SemioMeshFormat::Ply).map_err(|error| error.to_string()))
        .map_err(|error| semio_framework_diagnostic::TextError::new(
            semio_framework_value::ValueRefusalKind::InvalidValue,
            format!("gisterrain→ply: {error}"),
            semio_framework_diagnostic::TextSpan::at(1, 1),
        ))
}
