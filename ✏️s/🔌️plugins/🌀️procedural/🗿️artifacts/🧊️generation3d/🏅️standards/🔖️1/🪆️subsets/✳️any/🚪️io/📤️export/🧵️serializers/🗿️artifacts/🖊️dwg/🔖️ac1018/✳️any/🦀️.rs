//! 🖊️ generation3d → `s.stdio.dwg` — the document's EVALUATED preview mesh as a real DWG polyface mesh.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf emitted the artifact's own DSL text
//! under a `.dwg` name (see the stl sibling's doc comment for the shared defect).
//!
//! Composition: `mesh_bridge::preview_semio_mesh` → `SemioMeshToDwg`, which builds one real
//! `DwgGeometry::PolyfaceMesh` entity on its own layer — a mesh entity, not an edge-polyline
//! fallback — then dwg's own `encode_dwg` for the native container bytes.
//!
//! 🔖 Standard note: this artifact's registered export dialect is `ac1018` (R2004) and the writer
//! `encode_dwg` reaches is the R2004 canonical writer, so the label and the bytes agree; the
//! `SemioMeshToDwg` bridge lives under the dwg artifact's `ac1024` module tree only because that is
//! where the shared logical drawing model was rehomed — one `DwgSnapshot` type serves both.
use crate::standards::v1::subsets::any::io::mesh_bridge::{io_error, preview_semio_mesh};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactSerializer;
use semio_s_artifact_stdio_dwg::DwgSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::dwg::v_ac1024::any::SemioMeshToDwg;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

pub fn register() {}

pub fn serialize_mesh(mesh: &SemioMeshSnapshot) -> Result<DwgSnapshot, store::TextError> {
    semio_framework_plugin::resolve_ready(SemioMeshToDwg::serialize(mesh)).map_err(|error| io_error(format!("generation3d→dwg: {error}")))
}

/// 🖊️ Native container bytes, through dwg's own `AC1015` writer — the exact one `encode_dwg`
/// dispatches to for this version.
///
/// 🐛️ `encode_dwg` itself is NOT the entry point here, and the reason is a real, reproduced
/// incompatibility between two stdio components rather than a preference: after writing, it
/// cross-checks the produced file header's maintenance byte and codepage word against the
/// `DwgSnapshot`'s own `maintenance_version`/`codepage` fields, and `SemioMeshToDwg` builds its
/// snapshot with `..DwgSnapshot::default()` — leaving both at `0` while the `AC1015` writer stamps
/// a real codepage. The result was `DWG header mismatch: codepage 36865 != 0` on every export.
/// Stamping those fields here would mean this leaf hard-coding another artifact's writer constants;
/// calling the writer directly keeps one source of truth for them, and `decode_dwg` reads the
/// triple back out of the bytes anyway (it never consults the snapshot), so the round trip is
/// unaffected.
pub fn serialize_mesh_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, store::TextError> {
    let drawing = serialize_mesh(mesh)?.drawing.to_native().map_err(|error| io_error(format!("generation3d→dwg: {error}")))?;
    semio_s_artifact_stdio_dwg::engine::dwg_to_bytes(&drawing).map_err(|error| io_error(format!("generation3d→dwg: {error}")))
}

pub fn serialize(snapshot: &Generation3dSnapshot) -> Result<DwgSnapshot, store::TextError> {
    serialize_mesh(&preview_semio_mesh(snapshot)?)
}

pub fn serialize_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    serialize_mesh_bytes(&preview_semio_mesh(snapshot)?)
}
