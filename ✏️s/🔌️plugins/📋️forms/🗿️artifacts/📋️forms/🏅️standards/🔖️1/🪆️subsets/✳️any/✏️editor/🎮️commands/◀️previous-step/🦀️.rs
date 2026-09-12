//! ◀️ Move one exact Forms Try window to its previous step.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::modes::blueprint::windows::try_wizard::config::FormsTryWindowConfig;
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "previous-step")]
pub struct PreviousStep {
    pub window_id: String,
    pub window_kind_id: String,
}

pub(crate) fn handle_window(payload: &PreviousStep, config: &FormsTryWindowConfig) -> Result<FormsTryWindowConfig, Fault> {
    if payload.window_id.is_empty() || payload.window_kind_id != crate::editor::forms::modes::blueprint::windows::try_wizard::FORMS_PLAY_WINDOW_TRY {
        return Err(Fault::from("forms-previous-step-window-required"));
    }
    Ok(FormsTryWindowConfig { current_step_index: config.current_step_index.saturating_sub(1) })
}

pub fn handle(_payload: &PreviousStep, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::default())
}
