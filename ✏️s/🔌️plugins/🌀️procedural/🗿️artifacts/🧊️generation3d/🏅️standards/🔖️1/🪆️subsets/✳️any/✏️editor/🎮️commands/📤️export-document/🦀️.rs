//! 📤️ Generation3d play app commands command — `export-document`: the user-facing half of this
//! artifact's `🚪️io` export leaves.
//!
//! 🐛️ Until ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane the seven export leaves were
//! round-trip tested and unreachable — no command, menu item, button or keybinding named them, so a
//! user could not export anything from this editor at all
//! (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1).
//!
//! ⏳️ Progress and cancellation for the expensive half are the chain that already owns them, not a
//! second one: a geometry export reads the RETAINED [`FlowEvalSession`]'s already-tessellated
//! preview (`export_mesh_from_session`) and evaluates nothing itself. So an export taken after the
//! preview has settled does no kernel work at all, and one taken while it is still computing is the
//! same `flowEvalTick` chain the status pill reports `phase`/`ratio` for and the `cancelPreviewEval`
//! button retires. `txt` — the one full-fidelity target — touches no geometry and is always a
//! bounded print of the document's own text.
//!
//! @see ../../../🚪️io/🦀️.rs — `document_io`, the composition point this command calls.
//! @see ../🛑️cancel-preview-eval/🦀️.rs — the cancellation this export's expensive half rides on.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::io::document_io;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-document")]
pub struct ExportDocument {
    pub format: String,
}

/// 📤️ Encodes the document in the picked format and hands the shell one download.
///
/// 🚨️ A format this artifact does not claim, a document that evaluates to no geometry, and a
/// geometry a format cannot represent are all TYPED faults carrying why — never an empty `Emit` that
/// looks to the user exactly like a successful export of nothing.
pub fn emit(
    payload: &ExportDocument,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    preview: Option<&semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot>,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let export = document_io::export_document_with_preview(doc.snapshot, &payload.format, preview).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("generation3d.io.export"), error.to_string()))?;
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data: export.data, encoding: export.encoding }))
}

/// 📤️ The session-aware entry point: the retained evaluation IS the geometry, so the export reads
/// its merged preview and falls back to an in-process evaluation only when that session is empty.
pub fn handle(
    payload: &ExportDocument,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit(payload, doc, retained_preview(doc, cfg, session).as_ref())
}

/// 👁️ The retained session's merged preview as this repo's own typed mesh, or `None` when nothing
/// has been evaluated yet — in which case the caller's fallback says so honestly.
pub fn retained_preview(
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &FlowEvalSession,
) -> Option<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot> {
    let mesh = crate::editor::generation3d::export_mesh_from_session(doc.snapshot, cfg.snapshot, session);
    crate::standards::v1::subsets::any::io::mesh_bridge::semio_mesh_from_mesh_data(&mesh).ok()
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
