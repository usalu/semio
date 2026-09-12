//! ▶️ Advance one exact Forms Try window when its visible step is valid.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::effective_try_values;
use crate::editor::forms::modes::blueprint::windows::try_wizard::config::FormsTryWindowConfig;
use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::FormsTryWindowTransient;
use crate::schema::can_advance;
use crate::{forms_steps, op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "next-step")]
pub struct NextStep {
    pub window_id: String,
    pub window_kind_id: String,
}

pub(crate) fn handle_window(payload: &NextStep, spec: &FormsSnapshot, config: &FormsTryWindowConfig, transient: &FormsTryWindowTransient) -> Result<FormsTryWindowConfig, Fault> {
    if payload.window_id.is_empty() || payload.window_kind_id != crate::editor::forms::modes::blueprint::windows::try_wizard::FORMS_PLAY_WINDOW_TRY {
        return Err(Fault::from("forms-next-step-window-required"));
    }
    let index = config.current_step_index as usize;
    let steps = forms_steps(spec);
    if index + 1 >= steps.len() { return Ok(config.clone()); }
    let values = effective_try_values(spec, transient).iter().map(|(key, value)| (key.to_owned(), crate::schema::value_to_dsl(value))).collect();
    Ok(if can_advance(&steps[index], &values) { FormsTryWindowConfig { current_step_index: config.current_step_index + 1 } } else { config.clone() })
}

pub fn handle(_payload: &NextStep, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::default())
}
