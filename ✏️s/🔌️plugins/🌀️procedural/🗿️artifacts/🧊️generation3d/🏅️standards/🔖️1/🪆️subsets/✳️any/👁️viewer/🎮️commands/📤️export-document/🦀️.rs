//! 📤️ Generation3d viewer command — `export-document`. Read-only by construction: the emitted
//! `ViewEmit` carries ONE `Effect::DownloadMediaExport` and no mutation of any lane.
//!
//! 🐛️ Until ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane this viewer had eight
//! commands, all view-only, and named none of the artifact's nine `🚪️io` leaves — a reader could
//! look at a generation but could not take it away with them
//! (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1).
//!
//! 🔒️ Export is the ONE io direction a viewer may have: it reads the document and hands the shell a
//! file. Import replaces the document, so it stays on the editor — `no_viewer_tool_publishes_on_the_
//! artifact_lane` is the law that keeps the distinction honest.
//!
//! @see ../../../🚪️io/🦀️.rs — `document_io::export_document`, shared verbatim with the editor.
//! @see ../../../✏️editor/🎮️commands/📤️export-document/🦀️.rs — the editor's binding of the same verb.

use crate::standards::v1::subsets::any::io::document_io;
use crate::viewer::generation3d::config::{Generation3dViewConfig, Generation3dViewConfigMutation};
use crate::Generation3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Fault, FaultCode, FaultOrigin, ViewEmit};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-document")]
#[value(rename_all = "camelCase")]
pub struct ExportDocument {
    pub format: String,
}

/// 👁️ The retained session's merged preview as this repo's own typed mesh, or `None` when nothing
/// has been evaluated yet.
///
/// 🐛️ A viewer's geometry is its RETAINED evaluation, exactly as the editor's is: the fallback in
/// `document_io` builds a fresh `FlowHost` and evaluates in-process, which resolves nothing in a
/// guest because the operators are host-contributed and reached only through the asynchronous
/// extension chain (measured live on 6018, ticket 26/09/09/PROCEDURAL-3D-END-TO-END io-surface lane).
pub fn retained_preview(doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dViewConfig>, session: &semio_framework_os_flow::FlowEvalSession) -> Option<SemioMeshSnapshot> {
    let payload = crate::viewer::generation3d::modes::view::windows::preview::preview_payload(session.eval_json(), &doc.snapshot.fixture, cfg.snapshot, Some(session), &Default::default());
    let meshes: Vec<semio_framework_plugin::MeshData> = dsl::json::parse(&payload.meshes_json)
        .ok()
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|entry| entry.get("data").cloned())
        .filter_map(|data| dsl::FromValue::from_value(dsl::json::to_dsl_value(&data)).ok())
        .collect();
    let merged = crate::editor::generation3d::merge_preview_meshes(&meshes);
    crate::standards::v1::subsets::any::io::mesh_bridge::semio_mesh_from_mesh_data(&merged).ok()
}

/// 📤️ Encodes the viewed document in the picked format and hands the shell one download.
pub fn handle(payload: &ExportDocument, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    emit(payload, doc, None)
}

/// 📤️ The session-aware entry point the retained route takes.
pub fn emit(payload: &ExportDocument, doc: &ArtifactView<'_, Generation3dSnapshot>, preview: Option<&SemioMeshSnapshot>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    let export = document_io::export_document_with_preview(doc.snapshot, &payload.format, preview).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("generation3d.io.export"), error.to_string()))?;
    Ok(ViewEmit::effect(Effect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data: export.data, encoding: export.encoding }))
}
