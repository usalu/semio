//! ✍️ ✍️ Writer play app commands command — `load-document-json`.

use crate::editor::writer::reset_document_effect;
use crate::op::WriterMutation;
use crate::WriterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️TextEdit
//#endregion 🔖️TextEdit

//#region 🔖️SetText
//#endregion 🔖️SetText

//#region 🔖️SetSnapshot
//#endregion 🔖️SetSnapshot

//#region 🔖️OpenDocument
//#endregion 🔖️OpenDocument

//#region 🔖️JsonSetters
/// 🙈️ Loads the document from a raw JSON string as a `LoadDocument` effect outside history — no mutation rows —
/// silently no-op'ing on a parse failure (dev-only chrome loader, never user-facing).
fn parse_document_json(json: &str) -> Emit<WriterMutation, NoConfigMutation> {
    match semio_framework_pack_json::from_json_str::<WriterSnapshot>(json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "load-document-json")]
pub struct LoadDocumentJson {
    pub json: String,
}

pub fn handle(payload: &LoadDocumentJson, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Ok(parse_document_json(&payload.json))
}
