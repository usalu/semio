//! ✍️ ✍️ Writer play app commands command — `set-snapshot`.

use crate::apps::writer::config::{WriterConfig, WriterConfigMutation, WriterEditorSelection};
use crate::apps::writer::reset_document_effect;
use crate::artifacts::writer::schema::{apply_jack_rename, format_writer_text, jack_symbol_at_offset, JackSymbolKind};
use crate::artifacts::writer::dsl::{dag_jack_example_document, jack_example_document};
use crate::artifacts::writer::op::{EditText, WriterMutation};
use crate::artifacts::writer::{writer_snapshot_with_text, writer_text, WriterSnapshot};
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
fn parse_document_json(json: &str) -> Emit<WriterMutation, WriterConfigMutation> {
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

pub fn handle(payload: &SetSnapshot, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    Ok(parse_document_json(&payload.json))
}
