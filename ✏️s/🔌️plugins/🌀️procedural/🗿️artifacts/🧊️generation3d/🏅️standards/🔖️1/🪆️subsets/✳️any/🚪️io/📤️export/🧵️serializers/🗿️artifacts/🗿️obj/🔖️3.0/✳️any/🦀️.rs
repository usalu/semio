//! 🗿️ generation3d → `s.stdio.obj@3.0` — the document's EVALUATED preview mesh as real Wavefront OBJ.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf emitted the artifact's own DSL text
//! under an `.obj` name (see the stl sibling's doc comment for the shared defect).
//!
//! Composition, not a second grammar: `mesh_bridge::preview_semio_mesh` →
//! `SemioMeshToObj` (the `s.stdio.semio@v1/mesh` bridge, already unit-tested) → obj's own
//! `encode_obj`. One `o` block per `SemioMesh`, so a re-import recovers the same mesh boundary.
//!
//! 🔖 Documented lossiness is `SemioMeshToObj`'s own: triangles only (a wire/point preview is a
//! typed error, never invented faces), and no material/colour model crosses.
use crate::standards::v1::subsets::any::io::mesh_bridge::{io_error, preview_semio_mesh};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactSerializer;
use semio_s_artifact_stdio_obj::ObjSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

pub fn register() {}

pub fn serialize_mesh(mesh: &SemioMeshSnapshot) -> Result<ObjSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshToObj::serialize(mesh)).map_err(|error| io_error(format!("generation3d→obj: {error}")))
}

pub fn serialize_mesh_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(semio_s_artifact_stdio_obj::engine::encode_obj(&serialize_mesh(mesh)?).into_bytes())
}

pub fn serialize(snapshot: &Generation3dSnapshot) -> Result<ObjSnapshot, store::TextError> {
    serialize_mesh(&preview_semio_mesh(snapshot)?)
}

pub fn serialize_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize_mesh_bytes(&preview_semio_mesh(snapshot)?)
}
