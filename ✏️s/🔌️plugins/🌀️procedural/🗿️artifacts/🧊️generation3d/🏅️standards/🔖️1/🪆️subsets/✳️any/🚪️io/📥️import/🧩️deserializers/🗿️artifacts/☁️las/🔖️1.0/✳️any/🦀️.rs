//! ☁️ generation3d ← `s.stdio.las@1.0` — decoded honestly, and refused honestly.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf discarded the bytes and returned an
//! empty document (see the stl sibling's doc comment for the shared defect).
//!
//! The bytes ARE decoded here — `decode_las` + `SemioMeshFromLas` produce a real
//! `s.stdio.semio@v1/mesh` point cloud, and [`mesh_from_bytes`] is the public, tested way to get
//! it. What does NOT exist is an inverse for the DOCUMENT: a generation3d document's geometry is
//! whatever its flow graph evaluates to, every BRep import operator the graph offers
//! (`brep.io.importStl`/`importObj`/`importDwg`/`importStep`) consumes a SURFACE, and LAS carries no
//! connectivity at all — it is the one format in this artifact's IO surface that is deliberately
//! export-only (the paired export leaf writes the preview mesh's vertices as a point cloud, and
//! says so).
//!
//! Turning those points into a surface would mean choosing a reconstruction (hull, polyline order,
//! triangulation) that the file never stated, so `deserialize`/`deserialize_bytes` return a typed
//! error naming exactly that. An empty document — the previous behaviour — was the one answer that
//! looked like success.
use crate::standards::v1::subsets::any::io::mesh_bridge::io_error;
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_las::LasSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🚫️ Why LAS is export-only for this artifact, in one sentence the UI can show verbatim.
const NO_SURFACE: &str = "generation3d←las: LAS is a point cloud with no face connectivity, and every BRep import operator this artifact's flow graph offers consumes a surface — reconstructing one would invent geometry the file never stated. Export to LAS is supported; import is not.";

pub fn register() {}

/// ☁️ The real decode — a point-cloud `SemioMeshSnapshot`. Public because the point cloud itself is
/// genuine data, even though no document can be built from it.
pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::TextError> {
    mesh_from_snapshot(&semio_s_artifact_stdio_las::engine::decode_las(bytes).map_err(|error| io_error(format!("generation3d←las: {error}")))?)
}

pub fn mesh_from_snapshot(from: &LasSnapshot) -> Result<SemioMeshSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshFromLas::deserialize(from)).map_err(|error| io_error(format!("generation3d←las: {error}")))
}

pub fn deserialize(_from: &LasSnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    Err(io_error(NO_SURFACE))
}

pub fn deserialize_bytes(_bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    Err(io_error(NO_SURFACE))
}
