//! 🔘️ 🔘️ Forms play app commands command — `patch-question-options`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::questions::patch_choice;
use crate::schema::{locate_question, question_edit_mutations};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-question-options")]
pub struct PatchQuestionOptions {
    pub question_ids: Vec<String>,
    pub option_value: String,
    pub field: String,
    pub value_json: String,
}

pub fn handle(payload: &PatchQuestionOptions, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let raw_value = dsl::os_pack::json::parse(&payload.value_json).map_err(|_| Fault::from("forms.choice.invalid-json"))?;
    let mut operations = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for id in &payload.question_ids {
        if !seen.insert(id) { continue; }
        let Some(location) = locate_question(doc.snapshot, id) else { continue; };
        let next = patch_choice(&location.question, &payload.option_value, &payload.field, &raw_value).map_err(|error| Fault::from(format!("forms.choice.{error}")))?;
        operations.extend(question_edit_mutations(&location.step_id, &location.question, &next));
    }
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(operations))
}
