//! 🔘️ 🔘️ Forms play app commands command — `patch-question-options`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::parse_value_json;
use crate::artifacts::forms::schema::update_block_operation;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Shell
async fn patch_question_option(spec: &FormsSnapshot, question_id: &str, option_value: &str, field: &str, raw_value: &Value) -> Option<FormMutation> {
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
//#endregion 🔖️Shell




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-question-options")]
pub struct PatchQuestionOptions {
    pub question_ids: Vec<String>,
    pub option_value: String,
    pub field: String,
    pub value_json: String,
}

pub async fn handle(payload: &PatchQuestionOptions, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let spec = doc.snapshot;
    let raw_value = parse_value_json(&payload.value_json);
    let operations: Vec<FormMutation> = payload.question_ids.iter().filter_map(|question_id| patch_question_option(spec, question_id, &payload.option_value, &payload.field, &raw_value)).collect();
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::amend(operations, format!("patch-option:{}:{}", payload.option_value, payload.field)))
}
