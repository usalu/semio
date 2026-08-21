//! ✍️ ✍️ Writer play app commands command — `set-fixture-json`.

use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use crate::editor::writer::config::{WriterConfig, WriterConfigMutation};
use crate::editor::writer::reset_document_effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "fixture-json")]
pub struct SetFixtureJson {
    pub json: String,
}

pub async fn handle(payload: &SetFixtureJson, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterMutation, WriterConfigMutation>, Fault> {
    Ok(parse_document_json(&payload.json))
}
