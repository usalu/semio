//! ↩️ Reset one exact Forms Try window.

use crate::editor::forms::commands::set_try_value::{cancel_pending_generations, TryWindowCommandOutput};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::modes::blueprint::windows::try_wizard::config::FormsTryWindowConfig;
use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::{FormsTryWindowLease, FormsTryWindowTransient};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reset-try")]
pub struct ResetTry {
    pub window_id: String,
    pub window_kind_id: String,
}

pub(crate) fn handle_window(
    payload: &ResetTry,
    operation: &semio_framework_plugin::AppOperationContext,
    lease: &FormsTryWindowLease,
) -> Result<(TryWindowCommandOutput, FormsTryWindowConfig), Fault> {
    if !lease.matches(&payload.window_id, &payload.window_kind_id, lease.window_generation, lease.document_generation) {
        return Err(Fault::from("forms-reset-try-window-stale"));
    }
    cancel_pending_generations(operation, lease);
    Ok((TryWindowCommandOutput { emit: Emit::default(), transient: Some(FormsTryWindowTransient::default()) }, FormsTryWindowConfig::default()))
}

pub fn handle(_payload: &ResetTry, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::default())
}
