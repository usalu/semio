//! 📐️ 📐️ Forms play app commands command — `remove-vector-field`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::parse_value_json;
use crate::artifacts::forms::schema::update_block_operation;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot, FormVectorField};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

fn add_vector_field(spec: &FormsSnapshot, question_id: &str, key: &str) -> Option<FormMutation> {
    let location = crate::artifacts::forms::schema::locate_question(spec, question_id)?;
    if location.question.fields.iter().flatten().any(|entry| entry.key == key) {
        return None;
    }
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.push(FormVectorField { key: key.into(), label: Some(key.into()), value: Some(0.0) });
        question.fields = Some(fields);
    })
}

fn remove_vector_field(spec: &FormsSnapshot, question_id: &str, field_key: &str) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.retain(|entry| entry.key != field_key);
        question.fields = Some(fields);
    })
}
//#endregion 🔖️Shell




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-vector-field")]
pub struct RemoveVectorField {
    pub question_id: String,
    pub field_key: String,
}

pub fn handle(payload: &RemoveVectorField, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    match remove_vector_field(doc.snapshot, &payload.question_id, &payload.field_key) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}
