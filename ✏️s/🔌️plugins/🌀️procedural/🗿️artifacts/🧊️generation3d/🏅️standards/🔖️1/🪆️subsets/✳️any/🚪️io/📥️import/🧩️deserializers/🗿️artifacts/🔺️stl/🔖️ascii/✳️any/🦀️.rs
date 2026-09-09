//! 🔺️ generation3d ← `s.stdio.stl@ascii` — a real STL file becomes a real, previewable document.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf was `let _ = bytes;
//! Ok(Generation3dSnapshot::default())` — it discarded the file and reported success, so importing
//! an STL silently produced an EMPTY document.
//!
//! A generation3d document has no mesh field to put a mesh in: its geometry is whatever its flow
//! graph EVALUATES to. So the import plants the file in the graph — a note holding the STL bytes
//! feeding a `brep.io.importStl` neuron (`preview: true`) feeding a preview sink — which the flow
//! evaluator turns into real BRep geometry through the same kernel every bundled example uses. The
//! bytes are decoded FIRST (stl's own `decode_stl_auto`, ASCII or binary) so a file that is not STL
//! fails loudly here instead of becoming a graph that fails later at evaluation time.
use crate::standards::v1::subsets::any::io::mesh_bridge::{base64_encode, import_document, io_error};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use semio_s_artifact_stdio_stl::StlSnapshot;

/// 🔧️ The neuron kind that re-enters STL bytes into the flow graph.
pub const IMPORT_NEURON_KIND: &str = "brep.io.importStl";

pub fn register() {}

/// 🔺️ The geometry half, isolated so the round-trip test can assert triangle counts and bounds
/// without a live flow evaluator.
pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::TextError> {
    let stl = semio_s_artifact_stdio_stl::engine::decode_stl_auto(bytes).map_err(|error| io_error(format!("generation3d←stl: {error}")))?;
    mesh_from_snapshot(&stl)
}

pub fn mesh_from_snapshot(from: &StlSnapshot) -> Result<SemioMeshSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshFromStl::deserialize(from)).map_err(|error| io_error(format!("generation3d←stl: {error}")))
}

pub fn deserialize(from: &StlSnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    mesh_from_snapshot(from)?;
    Ok(import_document(IMPORT_NEURON_KIND, base64_encode(&semio_s_artifact_stdio_stl::engine::encode_stl_ascii(from).into_bytes())))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    mesh_from_bytes(bytes)?;
    Ok(import_document(IMPORT_NEURON_KIND, base64_encode(bytes)))
}
