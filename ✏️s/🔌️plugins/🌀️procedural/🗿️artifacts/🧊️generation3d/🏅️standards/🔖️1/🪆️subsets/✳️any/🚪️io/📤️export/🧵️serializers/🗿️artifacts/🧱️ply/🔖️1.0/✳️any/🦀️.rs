//! 🧱️ generation3d → `s.stdio.ply@1.0` — the document's EVALUATED preview mesh as real ASCII PLY.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf emitted the artifact's own DSL text
//! under a `.ply` name (see the stl sibling's doc comment for the shared defect).
//!
//! Composition: caller-prepared `SemioMeshSnapshot` → `SemioMeshToPly` → ply's own `encode_ply`.
//! PLY is the one mesh target here that also accepts a `Points` primitive, so a wire/point preview
//! exports as a real, valid PLY point cloud with no `face` element rather than erroring.
use crate::standards::v1::subsets::any::io::mesh_bridge::io_error;
use semio_framework_plugin::ArtifactSerializer;
use semio_s_artifact_stdio_ply::PlySnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::ply::v1_0::any::SemioMeshToPly;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

pub fn serialize_mesh(mesh: &SemioMeshSnapshot) -> Result<PlySnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshToPly::serialize(mesh)).map_err(|error| io_error(format!("generation3d→ply: {error}")))
}

pub fn serialize_mesh_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_artifact_stdio_ply::engine::encode_ply(&serialize_mesh(mesh)?).map_err(|error| io_error(format!("generation3d→ply: {error}")))
}
