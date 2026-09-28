//! ❓️ ❓️ Forms play app commands command — `drop-question-kind`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::schema::{create_form_id, locate_question};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

use crate::editor::forms::questions::default_question_for_kind;

//#region 🔖️Shell
/// 🌳️ Resolves a document-tree drop target id (`"step:<id>"` or a question id) back to its owning step.
fn resolve_step_id_from_tree_target(spec: &FormsSnapshot, target_id: &str) -> Option<String> {
    if let Some(step_id) = target_id.strip_prefix("step:") {
        return Some(step_id.to_string());
    }
    locate_question(spec, target_id).map(|location| location.step_id)
}

//#endregion 🔖️Shell

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "drop-question-kind")]
pub struct DropQuestionKind {
    pub kind: String,
    pub target_id: String,
    pub drop_position: String,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the dropped question used to also
// become the selection here — selection is framework-owned `InteractionState` now, only ever mutated
// by the framework's own injected `interactionSelect` handling, never by an app command's `Emit`
// (mirrors note's `add-block`).
pub fn handle(payload: &DropQuestionKind, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let step_id = resolve_step_id_from_tree_target(spec, &payload.target_id).ok_or_else(|| Fault::from("forms.question.missing-target"))?;
    let index = crate::editor::forms::questions::placement::question_insert_index(&spec.definition, &step_id, &payload.target_id, &payload.drop_position, None).map_err(|error| Fault::from(format!("forms.question.{error}")))?;
    let question = default_question_for_kind(&payload.kind, create_form_id("q"));
    Ok(Emit { artifact_mutations: vec![FormMutation::CreateBlock(crate::mutations::create_block::mutation::CreateBlock { step_id, block: question, index: Some(index) })], ..Default::default() })
}
