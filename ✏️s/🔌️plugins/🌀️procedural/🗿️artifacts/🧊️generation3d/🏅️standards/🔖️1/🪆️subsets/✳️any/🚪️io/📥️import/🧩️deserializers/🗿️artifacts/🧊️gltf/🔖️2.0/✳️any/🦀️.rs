//! 🧊️ generation3d ← `s.stdio.gltf@2.0` — a real glTF/GLB file becomes a real, previewable document.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf discarded the bytes and returned an
//! empty document (see the stl sibling's doc comment for the shared defect and the fixture design).
//!
//! Both container forms are accepted — the `glTF` magic selects gltf's own `decode_glb`, anything
//! else goes through `parse_gltf_document` — because a user picking "import glTF" does not
//! distinguish them. The flow evaluator has no `brep.io.importGltf` operator, so geometry is
//! normalized through `SemioMeshFromGltf` → `SemioMeshToStl` → ASCII STL onto a `brep.io.importStl`
//! neuron, exactly as the ply sibling does.
//!
//! 🔖 Documented lossiness: only geometry crosses. glTF materials, textures, animations, skins,
//! cameras and the scene graph have no counterpart in a BRep flow graph and do not survive; a
//! non-triangle primitive is refused by `SemioMeshToStl` as a typed error rather than approximated.
use crate::standards::v1::subsets::any::io::mesh_bridge::{base64_encode, import_document, io_error, stl_ascii_bytes};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_gltf::GltfSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🔧️ The neuron kind glTF geometry re-enters the flow graph through, after normalization.
pub const IMPORT_NEURON_KIND: &str = "brep.io.importStl";

/// 🪄️ GLB container magic (glTF 2.0 §4.4.1) — the one byte-level cue that picks the container.
const GLB_MAGIC: &[u8; 4] = b"glTF";

pub fn register() {}

pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::TextError> {
    mesh_from_snapshot(&decode(bytes)?)
}

pub fn mesh_from_snapshot(from: &GltfSnapshot) -> Result<SemioMeshSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshFromGltf::deserialize(from)).map_err(|error| io_error(format!("generation3d←gltf: {error}")))
}

fn decode(bytes: &[u8]) -> Result<GltfSnapshot, store::TextError> {
    let decoded = if bytes.starts_with(GLB_MAGIC) { semio_s_artifact_stdio_gltf::engine::decode_glb(bytes) } else { semio_s_artifact_stdio_gltf::engine::parse_gltf_document(bytes) };
    decoded.map_err(|error| io_error(format!("generation3d←gltf: {error}")))
}

pub fn deserialize(from: &GltfSnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    Ok(import_document(IMPORT_NEURON_KIND, base64_encode(&stl_ascii_bytes(&mesh_from_snapshot(from)?)?)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    Ok(import_document(IMPORT_NEURON_KIND, base64_encode(&stl_ascii_bytes(&mesh_from_bytes(bytes)?)?)))
}
