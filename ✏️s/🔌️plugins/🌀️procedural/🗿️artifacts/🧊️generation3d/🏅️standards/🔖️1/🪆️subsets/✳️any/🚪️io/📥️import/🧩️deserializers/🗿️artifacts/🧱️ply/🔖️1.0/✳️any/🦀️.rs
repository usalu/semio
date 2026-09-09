//! 🧱️ generation3d ← `s.stdio.ply@1.0` — a real PLY file becomes a real, previewable document.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf discarded the bytes and returned an
//! empty document (see the stl sibling's doc comment for the shared defect and the fixture design).
//!
//! The flow evaluator has no `brep.io.importPly` operator, so this import NORMALIZES: ply's own
//! `decode_ply` → `SemioMeshFromPly` → `SemioMeshToStl` → ASCII STL, planted on a
//! `brep.io.importStl` neuron. That is a real conversion through two already-tested bridges, not a
//! second PLY grammar, and it is honest about its limit — a PLY with no `face` element is a point
//! cloud, `SemioMeshToStl` refuses it by name, and the refusal reaches the caller as a typed error
//! instead of an empty document.
use crate::standards::v1::subsets::any::io::mesh_bridge::{base64_encode, import_document, io_error, stl_ascii_bytes};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_ply::PlySnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🔧️ The neuron kind PLY geometry re-enters the flow graph through, after normalization.
pub const IMPORT_NEURON_KIND: &str = "brep.io.importStl";

pub fn register() {}

pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::TextError> {
    mesh_from_snapshot(&semio_s_artifact_stdio_ply::engine::decode_ply(bytes).map_err(|error| io_error(format!("generation3d←ply: {error}")))?)
}

pub fn mesh_from_snapshot(from: &PlySnapshot) -> Result<SemioMeshSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshFromPly::deserialize(from)).map_err(|error| io_error(format!("generation3d←ply: {error}")))
}

pub fn deserialize(from: &PlySnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    Ok(import_document(IMPORT_NEURON_KIND, base64_encode(&stl_ascii_bytes(&mesh_from_snapshot(from)?)?)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    Ok(import_document(IMPORT_NEURON_KIND, base64_encode(&stl_ascii_bytes(&mesh_from_bytes(bytes)?)?)))
}
