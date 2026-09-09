//! ☁️ generation3d → `s.stdio.las@1.0` — the document's EVALUATED preview mesh as a real LAS point cloud.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf emitted the artifact's own DSL text
//! under a `.las` name (see the stl sibling's doc comment for the shared defect).
//!
//! Composition: `mesh_bridge::preview_semio_mesh` → `SemioMeshToLas` → las's own `encode_las`.
//!
//! 🔖 LAS has NO face/topology concept, so this export keeps the preview mesh's VERTICES and drops
//! its connectivity — that is what "export a mesh to a point-cloud format" means, and it is why the
//! paired import leaf refuses the inverse by name rather than fabricating triangles. Coordinates are
//! quantized by LAS's own scaled-integer storage (`SemioMeshToLas` writes a fine `0.0001` scale);
//! this is a real property of the format, never claimed bit-exact.
use crate::standards::v1::subsets::any::io::mesh_bridge::{io_error, preview_semio_mesh};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactSerializer;
use semio_s_artifact_stdio_las::LasSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::las::v1_0::any::SemioMeshToLas;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

pub fn register() {}

pub fn serialize_mesh(mesh: &SemioMeshSnapshot) -> Result<LasSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshToLas::serialize(mesh)).map_err(|error| io_error(format!("generation3d→las: {error}")))
}

pub fn serialize_mesh_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, store::TextError> {
    semio_s_artifact_stdio_las::engine::encode_las(&serialize_mesh(mesh)?).map_err(|error| io_error(format!("generation3d→las: {error}")))
}

pub fn serialize(snapshot: &Generation3dSnapshot) -> Result<LasSnapshot, store::TextError> {
    serialize_mesh(&preview_semio_mesh(snapshot)?)
}

pub fn serialize_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize_mesh_bytes(&preview_semio_mesh(snapshot)?)
}
