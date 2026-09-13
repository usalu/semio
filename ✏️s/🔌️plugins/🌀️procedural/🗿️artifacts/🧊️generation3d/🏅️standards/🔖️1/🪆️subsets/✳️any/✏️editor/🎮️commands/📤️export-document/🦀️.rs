//! 📤️ Generation3d play app commands command — `export-document`: the user-facing half of this
//! artifact's `🚪️io` export leaves.
//!
//! 🐛️ Until ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane the seven export leaves were
//! round-trip tested and unreachable — no command, menu item, button or keybinding named them, so a
//! user could not export anything from this editor at all
//! (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1).
//!
//! ⏳️ Progress and cancellation for the expensive half are the chain that already owns them, not a
//! second one: every geometry format's bytes come from the document's EVALUATED preview, and
//! `mesh_data_for_preview_handle` reads the retained [`FlowEvalSession`]'s already-tessellated
//! meshes first. So an export taken after the preview has settled does no kernel work at all, and an
//! export taken while it is still computing is the same `flowEvalTick` chain the status pill reports
//! `phase`/`ratio` for and the `cancelPreviewEval` button retires. `txt` — the one full-fidelity
//! target — touches no geometry and is always a bounded print of the document's own text.
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
pub fn emit(payload: &ExportDocument, doc: &ArtifactView<'_, Generation3dSnapshot>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let export = document_io::export_document(doc.snapshot, &payload.format).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("generation3d.io.export"), error.to_string()))?;
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data: export.data, encoding: export.encoding }))
}

pub fn handle(
    payload: &ExportDocument,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit(payload, doc)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
