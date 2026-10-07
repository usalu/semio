//! 🩹️ Atomic validated field edits produce undoable document events.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::questions::patch_question;
use crate::standards::v1::subsets::any::io::text::snapshot::{value_to_dsl};
use crate::schema::{locate_question,question_edit_mutations};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_pack_json::Value;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "patch-questions")]
pub struct PatchQuestions {
    pub question_ids: Vec<String>,
    pub field: String,
    pub value_json: String,
    pub param_key: Option<String>,
}

pub fn handle(payload: &PatchQuestions, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let raw_value: Value = semio_framework_pack_json::parse(&payload.value_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| Fault::from("forms.patch.invalid-json"))?;
    let mut operations = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for id in &payload.question_ids {
        if !seen.insert(id) { continue; }
        let Some(location) = locate_question(doc.snapshot, id) else { continue; };
        let next = if let Some(field) = payload.field.strip_prefix("condition.") {
            let mut next = location.question.clone();
            next.condition = crate::editor::forms::questions::visibility::patch_condition(next.condition.as_ref(), payload.param_key.as_deref().unwrap_or(""), field, &raw_value).map_err(|error| Fault::from(format!("forms.patch.{error}")))?;
            next
        } else if payload.field == "param" {
            let key = payload.param_key.as_deref().filter(|key| !key.trim().is_empty()).ok_or_else(|| Fault::from("forms.patch.parameter-required"))?;
            let mut next = location.question.clone();
            let mut params = next.params.take().unwrap_or(semio_framework_value::DslValue::Object(Vec::new()));
            let semio_framework_value::DslValue::Object(entries) = &mut params else { return Err(Fault::from("forms.patch.invalid-parameters")); };
            let value = value_to_dsl(&raw_value);
            if let Some((_, slot)) = entries.iter_mut().find(|(name, _)| name == key) { *slot = value; }
            else { entries.push((key.to_owned(), value)); }
            next.params = Some(params);
            next
        } else {
            patch_question(&location.question, &payload.field, &raw_value).map_err(|error| Fault::from(format!("forms.patch.{error}")))?
        };
        operations.extend(question_edit_mutations(&location.step_id, &location.question, &next));
    }
    Ok(Emit::mutations(operations))
}
