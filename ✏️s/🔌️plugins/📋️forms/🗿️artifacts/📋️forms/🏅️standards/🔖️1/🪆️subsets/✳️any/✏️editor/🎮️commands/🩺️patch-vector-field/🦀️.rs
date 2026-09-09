//! 📐️ 📐️ Forms play app commands command — `patch-vector-field`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::parse_value_json;
use crate::schema::update_block_operation;
use crate::{op::FormMutation, FormsSnapshot};
use dsl::os_pack::json::Value;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shell
fn patch_vector_field(spec: &FormsSnapshot, question_id: &str, field_key: &str, field: &str, raw_value: &Value) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        if let Some(entry) = fields.iter_mut().find(|item| item.key == field_key) {
            match field {
                "label" => entry.label = raw_value.as_str().map(str::to_string),
                "value" => entry.value = raw_value.as_f64(),
                _ => {}
            }
        }
        question.fields = Some(fields);
    })
}
//#endregion 🔖️Shell

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-vector-field")]
pub struct PatchVectorField {
    pub question_id: String,
    pub field_key: String,
    pub field: String,
    pub value_json: String,
}

pub fn handle(payload: &PatchVectorField, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let raw_value = parse_value_json(&payload.value_json);
    match patch_vector_field(doc.snapshot, &payload.question_id, &payload.field_key, &payload.field, &raw_value) {
        Some(operation) => Ok(Emit::amend(vec![operation], format!("patch-vector:{}:{}:{}", payload.question_id, payload.field_key, payload.field))),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
