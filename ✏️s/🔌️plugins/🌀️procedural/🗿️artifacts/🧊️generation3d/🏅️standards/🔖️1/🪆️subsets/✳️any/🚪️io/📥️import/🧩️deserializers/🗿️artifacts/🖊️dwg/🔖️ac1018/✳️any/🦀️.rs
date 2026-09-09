//! 🖊️ generation3d ← `s.stdio.dwg` — a real DWG drawing becomes a real, previewable document.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf discarded the bytes and returned an
//! empty document (see the stl sibling's doc comment for the shared defect and the fixture design).
//! That was the exact path the plugin root's registered mesh-import host-media handler leans on
//! (`register_mesh_dwg_import_handler`, `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`), so dropping a DWG on
//! this app produced a blank canvas.
//!
//! DWG has its own flow operator (`brep.io.importDwg`, base64 `data` channel), so the native bytes
//! are planted verbatim rather than normalized through STL — the kernel's own DWG reader keeps the
//! layer/entity structure this artifact's export writes. The container is decoded first
//! (`decode_dwg`) and the drawing is required to actually carry mesh geometry
//! (`SemioMeshFromDwg`), so a curve-only or non-DWG file fails loudly here rather than evaluating
//! to nothing later.
use crate::standards::v1::subsets::any::io::mesh_bridge::{base64_encode, import_document, io_error};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_dwg::DwgSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::dwg::v_ac1024::any::SemioMeshFromDwg;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🔧️ The neuron kind that re-enters DWG bytes into the flow graph.
pub const IMPORT_NEURON_KIND: &str = "brep.io.importDwg";

pub fn register() {}

pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::TextError> {
    mesh_from_snapshot(&semio_s_artifact_stdio_dwg::schema::snapshot::decode_dwg(bytes).map_err(|error| io_error(format!("generation3d←dwg: {error}")))?)
}

pub fn mesh_from_snapshot(from: &DwgSnapshot) -> Result<SemioMeshSnapshot, store::TextError> {
    let mesh = semio_framework_plugin::resolve_ready(SemioMeshFromDwg::deserialize(from)).map_err(|error| io_error(format!("generation3d←dwg: {error}")))?;
    if mesh.meshes.iter().all(|entry| entry.primitives.is_empty()) {
        return Err(io_error("generation3d←dwg: the drawing carries no polyface-mesh or 3d-face entity, so it holds no geometry this artifact can preview"));
    }
    Ok(mesh)
}

pub fn deserialize(from: &DwgSnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    mesh_from_snapshot(from)?;
    let bytes = semio_s_artifact_stdio_dwg::schema::snapshot::encode_dwg(from).map_err(|error| io_error(format!("generation3d←dwg: {error}")))?;
    Ok(import_document(IMPORT_NEURON_KIND, base64_encode(&bytes)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    mesh_from_bytes(bytes)?;
    Ok(import_document(IMPORT_NEURON_KIND, base64_encode(bytes)))
}
