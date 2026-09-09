//! 🔺️ generation3d → `s.stdio.stl@ascii` — the document's EVALUATED preview mesh as real ASCII STL.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf returned
//! `print_dsl(snapshot).into_bytes()` — the artifact's own native DSL TEXT, mislabelled `.stl`. It
//! never emitted a single facet and never failed, so "Export to STL" looked like it worked.
//!
//! The real conversion is a two-step composition of code that already exists and is already tested:
//! `mesh_bridge::preview_semio_mesh` runs the editor's own preview pipeline to a
//! `s.stdio.semio@v1/mesh` document, and `SemioMeshToStl` (that subset's own bridge, with its own
//! unit tests) turns it into a `StlSnapshot`. Bytes come from stl's own `encode_stl_ascii` grammar —
//! deliberately NOT `ArtifactDsl::print_dsl`, which wraps the facets in a `semio` envelope preamble
//! that no STL reader accepts.
//!
//! 🔖 Documented lossiness is `SemioMeshToStl`'s own: STL is triangles-only and non-indexed, so a
//! preview that carries no face connectivity (a wire or point preview) is a typed error rather than
//! invented geometry, and per-vertex normals collapse to one facet normal.
use crate::standards::v1::subsets::any::io::mesh_bridge::{io_error, preview_semio_mesh};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactSerializer;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use semio_s_artifact_stdio_stl::StlSnapshot;

pub fn register() {}

/// 🔺️ The mesh half, isolated so the round-trip test can drive it without a live flow evaluator.
pub fn serialize_mesh(mesh: &SemioMeshSnapshot) -> Result<StlSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(mesh)).map_err(|error| io_error(format!("generation3d→stl: {error}")))
}

pub fn serialize_mesh_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(semio_s_artifact_stdio_stl::engine::encode_stl_ascii(&serialize_mesh(mesh)?).into_bytes())
}

pub fn serialize(snapshot: &Generation3dSnapshot) -> Result<StlSnapshot, store::TextError> {
    serialize_mesh(&preview_semio_mesh(snapshot)?)
}

pub fn serialize_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize_mesh_bytes(&preview_semio_mesh(snapshot)?)
}
