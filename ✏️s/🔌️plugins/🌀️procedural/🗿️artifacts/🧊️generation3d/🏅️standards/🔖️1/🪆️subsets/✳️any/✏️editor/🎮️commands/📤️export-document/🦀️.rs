//! 📤️ Generation3d play app commands command — `export-document`: the user-facing half of this
//! artifact's `🚪️io` export leaves.
//!
//! 🐛️ Until ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane the seven export leaves were
//! round-trip tested and unreachable — no command, menu item, button or keybinding named them, so a
//! user could not export anything from this editor at all
//! (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1).
//!
//! ⏳️ Progress and cancellation for the expensive half are the chain that already owns them, not a
//! second one: a geometry export reads the RETAINED [`FlowEvalSession`]'s already-prepared
//! surfaces (`export_meshes_from_session`) and evaluates nothing itself. So an export taken after the
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-document")]
#[value(rename_all = "camelCase")]
pub struct ExportDocument {
    pub format: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
}

/// 📤️ Encodes the document in the picked format and hands the shell one download.
///
/// 🚨️ A format this artifact does not claim, a document that evaluates to no geometry, and a
/// geometry a format cannot represent are all TYPED faults carrying why — never an empty `Emit` that
/// looks to the user exactly like a successful export of nothing.
pub fn emit(
    payload: &ExportDocument,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    meshes: Option<&[semio_framework_plugin::MeshData]>,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let export = (if payload.format == "txt" {
        document_io::export_document(doc.snapshot)
    } else {
        meshes.ok_or_else(|| crate::standards::v1::subsets::any::io::mesh_bridge::io_error("generation3d geometry export requires prepared geometry from the retained evaluation"))
            .and_then(|meshes| document_io::export_prepared_geometry(meshes, &payload.format))
    }).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new(crate::GENERATION3D_IO_EXPORT), error.to_string()))?;
    Ok(Emit::effect(Effect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data: export.data, encoding: export.encoding }))
}

/// 📤️ The session-aware entry point: the retained evaluation IS the geometry, so the export reads
/// its prepared surfaces; absent geometry produces a named export fault.
pub fn handle(
    payload: &ExportDocument,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let meshes = if payload.format == "txt" { None } else { retained_meshes(doc, cfg, session, payload.widget_id.as_deref())? };
    emit(payload, doc, meshes.as_deref())
}

/// 👁️ Reads the existing retained evaluation's prepared surfaces without reducing authored channels.
pub fn retained_meshes(
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    session: &FlowEvalSession,
    widget_id: Option<&str>,
) -> Result<Option<Vec<semio_framework_plugin::MeshData>>, Fault> {
    let meshes = crate::editor::generation3d::export_meshes_from_session(doc.snapshot, cfg.snapshot, session, widget_id);
    meshes.map(Some)
        .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new(crate::GENERATION3D_IO_EXPORT), error.to_string()))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
