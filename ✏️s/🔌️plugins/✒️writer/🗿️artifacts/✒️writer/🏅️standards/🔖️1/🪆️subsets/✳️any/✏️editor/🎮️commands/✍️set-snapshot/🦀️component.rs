//! ✍️ ✍️ Writer play app commands command — `set-snapshot`.

use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use crate::editor::writer::reset_document_effect;
use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️TextEdit
//#endregion 🔖️TextEdit

//#region 🔖️SetText
//#endregion 🔖️SetText

//#region 🔖️SetSnapshot
//#endregion 🔖️SetSnapshot

//#region 🔖️OpenDocument
//#endregion 🔖️OpenDocument

//#region 🔖️JsonSetters
/// 🙈️ Shared body for `SetSnapshotJson`/`SetFixtureJson` — both replace the whole document from a raw
/// JSON string, silently no-op'ing on a parse failure (dev-only chrome setters, never user-facing).
async fn parse_document_json(json: &str) -> Emit<WriterMutation, WriterConfigMutation> {
    match serde_json::from_str::<WriterSnapshot>(json) {
        Ok(document) => Emit { effects: vec![reset_document_effect(&document)], ..Default::default() },
        Err(_) => Emit::default(),
    }
}

//#endregion 🔖️JsonSetters

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

//#region 🔖️FormatDocument
//#endregion 🔖️FormatDocument

//#region 🔖️CommitRename
//#endregion 🔖️CommitRename

/// 🔧️ `snapshot` is JSON text, not a nested `#[dsl(block)]` struct field — `WriterSnapshot` no
/// longer implements `dsl::DslField` now that `document` is a composed `ArtifactChild<S>` slot
/// (no `DslField` impl reachable from this crate, same gap `📐️cad`/`💠️lowpoly` hit for their own
/// composed-child snapshot types). Functionally identical to `SetSnapshotJson` — kept as its own
/// row for wire-format/manifest stability rather than folding the two together mid-ticket.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    pub json: String,
}

pub async fn handle(payload: &SetSnapshot, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    Ok(parse_document_json(&payload.json))
}
