//! 📐️ 📐️ Forms play app commands command — `add-vector-field`.

use crate::artifacts::forms::schema::update_block_operation;
use crate::artifacts::forms::{op::FormMutation, FormVectorField, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shell
async fn add_vector_field(spec: &FormsSnapshot, question_id: &str, key: &str) -> Option<FormMutation> {
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
//#endregion 🔖️Shell

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-vector-field")]
pub struct AddVectorField {
    pub question_id: String,
    pub field_key: String,
}

pub async fn handle(payload: &AddVectorField, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    match add_vector_field(doc.snapshot, &payload.question_id, &payload.field_key) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}
