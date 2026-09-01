//! 📐️ 📐️ Forms play app commands command — `remove-vector-field`.

use crate::artifacts::forms::schema::update_block_operation;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shell
async fn remove_vector_field(spec: &FormsSnapshot, question_id: &str, field_key: &str) -> Option<FormMutation> {
    update_block_operation(spec, question_id, |question| {
        let mut fields = question.fields.take().unwrap_or_default();
        fields.retain(|entry| entry.key != field_key);
        question.fields = Some(fields);
    })
}
//#endregion 🔖️Shell

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-vector-field")]
pub struct RemoveVectorField {
    pub question_id: String,
    pub field_key: String,
}

pub async fn handle(payload: &RemoveVectorField, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    match remove_vector_field(doc.snapshot, &payload.question_id, &payload.field_key) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}
