//! 🧊️ generation3d → `s.stdio.gltf@2.0` — the document's EVALUATED preview mesh as real glTF 2.0 JSON.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf emitted the artifact's own DSL text
//! under a `.gltf` name (see the stl sibling's doc comment for the shared defect).
//!
//! Composition: `mesh_bridge::preview_semio_mesh` → `SemioMeshToGltf` (which packs one
//! little-endian buffer plus tightly-packed accessors/bufferViews) → gltf's own
//! `serialize_gltf_document`, which is what turns the still-`uri`-less geometry buffer into the
//! `data:application/octet-stream;base64,…` uri a `.gltf` file needs. `.glb` is deliberately NOT
//! emitted here: this leaf's registered dialect is the JSON form.
//!
//! 🔖 glTF is the one mesh target that carries every `SemioTopology`, so a wire or point preview
//! exports as a real `mode: 0/1/3` primitive instead of erroring.
use crate::standards::v1::subsets::any::io::mesh_bridge::{io_error, preview_semio_mesh};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactSerializer;
use semio_s_artifact_stdio_gltf::GltfSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

pub fn register() {}

pub fn serialize_mesh(mesh: &SemioMeshSnapshot) -> Result<GltfSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshToGltf::serialize(mesh)).map_err(|error| io_error(format!("generation3d→gltf: {error}")))
}

pub fn serialize_mesh_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(semio_s_artifact_stdio_gltf::engine::serialize_gltf_document(&serialize_mesh(mesh)?))
}

pub fn serialize(snapshot: &Generation3dSnapshot) -> Result<GltfSnapshot, store::TextError> {
    serialize_mesh(&preview_semio_mesh(snapshot)?)
}

pub fn serialize_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize_mesh_bytes(&preview_semio_mesh(snapshot)?)
}
