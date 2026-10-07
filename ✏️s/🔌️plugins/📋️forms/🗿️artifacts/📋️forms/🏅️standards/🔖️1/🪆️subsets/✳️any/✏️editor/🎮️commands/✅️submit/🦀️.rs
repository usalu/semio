//! 📨️ Validate and persist one response from the addressed answering window.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::forms::modes::blueprint::windows::try_wizard::{config::FormsTryWindowConfig, transient::FormsTryWindowTransient};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "submit")]
pub struct Submit {
    pub window_id: String,
    pub window_kind_id: String,
}

pub fn handle(_payload: &Submit, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Err(Fault::from("forms-submit-window-context-required"))
}

pub fn handle_window(
    spec: &FormsSnapshot,
    config: &FormsTryWindowConfig,
    transient: &FormsTryWindowTransient,
    definition_version: String,
) -> Result<(Emit<FormMutation, FormsConfigMutation>, FormsTryWindowConfig), Fault> {
    if config.submitted_response_id.as_ref().is_some_and(|id| spec.responses.iter().any(|response| &response.id == id)) { return Ok((Emit::default(), config.clone())); }
    if spec.definition.steps.is_empty() { return Err(Fault::from("forms-submit-empty-form")); }
    let values = crate::editor::forms::effective_try_values(spec, transient).iter().map(|(key, value)| (key.to_owned(), crate::standards::v1::subsets::any::io::text::snapshot::value_to_dsl(value))).collect();
    let response = match crate::schema::response::prepare_response(&spec.definition, &values, crate::schema::create_form_id("response"), dsl::os_identity::unix_millis(), definition_version) {
        Ok(response) => response,
        Err(errors) => {
            let current_step_index = spec.definition.steps.iter().position(|step| step.blocks.iter().any(|question| errors.iter().any(|error| error.question_id == question.id))).unwrap_or(0) as u32;
            return Ok((Emit::default(), FormsTryWindowConfig { current_step_index, submitted_response_id: None }));
        }
    };
    let next = FormsTryWindowConfig { submitted_response_id: Some(response.id.clone()), ..config.clone() };
    let mutation = FormMutation::CommitResponse(crate::mutations::commit_response::mutation::CommitResponse { response, index: None });
    Ok((Emit { artifact_mutations: vec![mutation], ..Default::default() }, next))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
