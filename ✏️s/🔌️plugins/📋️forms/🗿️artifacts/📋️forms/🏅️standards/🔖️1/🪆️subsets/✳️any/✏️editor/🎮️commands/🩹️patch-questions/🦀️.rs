//! 🩹️ Atomic validated field edits produce undoable document events.

use crate::forms_config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::questions::patch_question;
use crate::standards::v1::subsets::any::io::text::snapshot::{value_to_dsl};
use crate::mutations::question_edit_mutations;
use crate::schema::locate_question;
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_pack_json::Value;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-questions")]
#[value(rename_all = "camelCase")]
pub struct PatchQuestions {
    pub question_ids: Vec<String>,
    pub field: String,
    pub value_json_chunks: Vec<String>,
    pub param_key: Option<String>,
}

pub fn handle(payload: &PatchQuestions, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let raw_value: Value = semio_framework_pack_json::parse(&payload.value_json()?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| Fault::from("forms.patch.invalid-json"))?;
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

pub const MAXIMUM_VALUE_BYTES: usize = 32_770;

impl PatchQuestions {
    /// 🧾️ Validates direct typed callers as well as action ingress before reading a value.
    pub fn value_json(&self) -> Result<String, Fault> {
        if self.question_ids.is_empty() || self.question_ids.len() > 64 || self.question_ids.iter().any(|id| id.is_empty() || id.len() > 512) || self.question_ids.iter().map(String::len).sum::<usize>() > 8_192 || self.value_json_chunks.len() > 65 || self.field.is_empty() || self.field.len() > 128 || self.param_key.as_ref().is_some_and(|key| key.is_empty() || key.len() > 128) { return Err(Fault::from("forms.patch.invalid-target")); }
        semio_ui_contract::UiValue::join_text_chunks(self.value_json_chunks.iter().map(String::as_str), MAXIMUM_VALUE_BYTES).map_err(|_| Fault::from("forms.patch.invalid-chunks"))
    }
}

fn argument<'a>(args: &'a [(String, semio_framework_value::DslValue)], camel: &str, snake: &str) -> Result<Option<&'a semio_framework_value::DslValue>, Fault> {
    let mut values = args.iter().filter(|(key, _)| key == camel || key == snake);
    let value = values.next().map(|(_, value)| value);
    if values.next().is_some() { return Err(Fault::from("forms.patch.duplicate-argument")); }
    Ok(value)
}

fn split_value(value: &str) -> Result<Vec<String>, Fault> {
    use semio_ui_contract::UiValue;
    let UiValue::List(chunks) = UiValue::try_text_chunks(value, MAXIMUM_VALUE_BYTES).map_err(|_| Fault::from("forms.patch.value-too-large"))? else { return Err(Fault::from("forms.patch.invalid-chunks")); };
    chunks.iter().map(|chunk| match chunk { UiValue::Text(text) => Ok(text.as_str().to_owned()), _ => Err(Fault::from("forms.patch.invalid-chunks")) }).collect()
}

/// 📥️ Normalizes scalar controls and complete paged editors into one canonical typed payload.
pub fn from_action(args: Option<&semio_framework_value::DslValue>) -> Result<PatchQuestions, Fault> {
    use semio_framework_value::DslValue;
    let args = args.and_then(DslValue::as_object).ok_or_else(|| Fault::from("forms.patch.arguments-required"))?;
    let question_ids = match argument(args, "questionIds", "question_ids")? { Some(DslValue::Array(ids)) if !ids.is_empty() && ids.len() <= 64 => ids.iter().map(|id| id.as_str().map(str::to_owned).ok_or_else(|| Fault::from("forms.patch.invalid-target"))).collect::<Result<Vec<_>,_>>()?, _ => return Err(Fault::from("forms.patch.invalid-target")) };
    let field = argument(args, "field", "field")?.and_then(DslValue::as_str).ok_or_else(|| Fault::from("forms.patch.invalid-field"))?.to_owned();
    let param_key = match argument(args, "paramKey", "param_key")? { None | Some(DslValue::Null) => None, Some(DslValue::String(key)) => Some(key.clone()), _ => return Err(Fault::from("forms.patch.invalid-target")) };
    if argument(args, "valueJson", "value_json")?.is_some() { return Err(Fault::from("forms.patch.unsupported-transport")); }
    let scalar = argument(args, "value", "value")?;
    let chunks = argument(args, "valueJsonChunks", "value_json_chunks")?;
    let value_json_chunks = match (scalar, chunks) {
        (Some(value), None) => split_value(&semio_framework_pack_json::to_json_string(value))?,
        (None, Some(DslValue::Array(chunks))) => {
            if chunks.is_empty() || chunks.len() > 65 { return Err(Fault::from("forms.patch.invalid-chunks")); }
            chunks.iter().map(|chunk| chunk.as_str().filter(|text| text.len() <= 512).map(str::to_owned).ok_or_else(|| Fault::from("forms.patch.invalid-chunks"))).collect::<Result<Vec<_>,_>>()?
        },
        _ => return Err(Fault::from("forms.patch.mixed-transport")),
    };
    let payload = PatchQuestions { question_ids, field, param_key, value_json_chunks };
    semio_framework_pack_json::parse(&payload.value_json()?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| Fault::from("forms.patch.invalid-json"))?;
    Ok(payload)
}
