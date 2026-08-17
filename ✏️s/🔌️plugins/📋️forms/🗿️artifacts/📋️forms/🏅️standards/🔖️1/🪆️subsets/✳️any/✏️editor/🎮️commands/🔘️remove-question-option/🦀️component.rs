//! 🔘️ 🔘️ Forms play app commands command — `remove-question-option`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::artifacts::forms::schema::{create_form_id, update_block_operation};
use crate::artifacts::forms::{op::FormMutation, FormQuestionOption, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Shell
fn patch_question_option(spec: &FormsSnapshot, question_id: &str, option_value: &str, field: &str, raw_value: &Value) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        if let Some(option) = options.iter_mut().find(|entry| entry.value == option_value) {
            if field == "label" {
                option.label = raw_value.as_str().unwrap_or("").to_string();
            }
        }
        question.options = Some(options);
    })
}

fn add_question_option(spec: &FormsSnapshot, question_id: &str, label: &str) -> Option<FormMutation> {
    let value = create_form_id("opt");
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.push(FormQuestionOption { value, label: label.into() });
        question.options = Some(options);
    })
}

fn remove_question_option(spec: &FormsSnapshot, question_id: &str, option_value: &str) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut options = question.options.take().unwrap_or_default();
        options.retain(|entry| entry.value != option_value);
        question.options = Some(options);
    })
}
//#endregion 🔖️Shell




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-question-option")]
pub struct RemoveQuestionOption {
    pub question_id: String,
    pub option_value: String,
}

pub fn handle(payload: &RemoveQuestionOption, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    match remove_question_option(doc.snapshot, &payload.question_id, &payload.option_value) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}
