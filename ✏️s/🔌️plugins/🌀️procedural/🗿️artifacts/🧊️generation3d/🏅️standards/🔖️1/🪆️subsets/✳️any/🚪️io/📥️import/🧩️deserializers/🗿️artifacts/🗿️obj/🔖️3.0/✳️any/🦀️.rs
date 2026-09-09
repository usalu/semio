//! 🗿️ generation3d ← `s.stdio.obj@3.0` — a real Wavefront OBJ becomes a real, previewable document.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf discarded the bytes and returned an
//! empty document (see the stl sibling's doc comment for the shared defect and the fixture design).
//!
//! OBJ is the one mesh format besides STL/DWG with its own flow operator, and its `data` channel
//! takes PLAIN TEXT rather than base64 (`brep.io.importObj` reads the channel with `read_text`), so
//! the note carries the OBJ source verbatim and stays human-readable in the graph. The text is
//! parsed first (obj's own `decode_obj`) so a non-OBJ payload fails here, not at evaluation time.
use crate::standards::v1::subsets::any::io::mesh_bridge::{import_document, io_error};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_obj::ObjSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🔧️ The neuron kind that re-enters OBJ text into the flow graph.
pub const IMPORT_NEURON_KIND: &str = "brep.io.importObj";

pub fn register() {}

pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::TextError> {
    mesh_from_snapshot(&decode(bytes)?)
}

pub fn mesh_from_snapshot(from: &ObjSnapshot) -> Result<SemioMeshSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshFromObj::deserialize(from)).map_err(|error| io_error(format!("generation3d←obj: {error}")))
}

fn decode(bytes: &[u8]) -> Result<ObjSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| io_error(format!("generation3d←obj: obj is not valid utf-8: {error}")))?;
    semio_s_artifact_stdio_obj::engine::decode_obj(text).map_err(|error| io_error(format!("generation3d←obj: {error}")))
}

pub fn deserialize(from: &ObjSnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    mesh_from_snapshot(from)?;
    Ok(import_document(IMPORT_NEURON_KIND, semio_s_artifact_stdio_obj::engine::encode_obj(from)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    deserialize(&decode(bytes)?)
}
