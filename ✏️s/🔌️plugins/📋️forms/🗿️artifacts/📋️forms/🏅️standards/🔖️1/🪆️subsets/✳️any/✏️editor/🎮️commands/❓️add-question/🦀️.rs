//! ❓️ ❓️ Forms play app commands command — `add-question`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::schema::create_form_id;
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

use crate::editor::forms::questions::default_question_for_kind;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-question")]
pub struct AddQuestion {
    pub kind: String,
    pub step_id: Option<String>,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new question used to also become
// the selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
// framework's own injected `interactionSelect` handling, never by an app command's `Emit` (mirrors
// note's `add-block`).
pub fn handle(payload: &AddQuestion, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let question = default_question_for_kind(&payload.kind, create_form_id("q"));
    let mutation = crate::editor::forms::questions::placement::create_question_event(&spec.definition, question, payload.step_id.as_deref(), &create_form_id("step")).map_err(|error| Fault::from(format!("forms.question.{error}")))?;
    Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
