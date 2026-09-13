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
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-document")]
#[value(rename_all = "camelCase")]
pub struct ExportDocument {
    pub format: String,
}

/// 📤️ Encodes the viewed document in the picked format and hands the shell one download.
pub fn handle(payload: &ExportDocument, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dViewConfig>) -> Result<ViewEmit<Generation3dViewConfigMutation>, Fault> {
    let export = document_io::export_document(doc.snapshot, &payload.format).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("generation3d.io.export"), error.to_string()))?;
    Ok(ViewEmit::effect(Effect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data: export.data, encoding: export.encoding }))
}
