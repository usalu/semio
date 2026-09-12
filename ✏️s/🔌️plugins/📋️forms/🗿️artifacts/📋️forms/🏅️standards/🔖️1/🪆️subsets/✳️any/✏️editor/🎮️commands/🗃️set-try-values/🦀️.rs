//! 🗃️ Bounded bulk Try-value input for one exact Forms Try window lease.

use crate::editor::forms::commands::set_try_value::{stage_command_input, start_bulk_window, ChunkAddressableJson, TryWindowCommandOutput};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::{FormsTryWindowLease, FormsTryWindowTransient};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "try-values")]
pub struct SetTryValues {
    pub values_json: ChunkAddressableJson,
    pub input_id: Option<String>,
    pub input_index: Option<u64>,
    pub input_count: Option<u64>,
    pub window_id: String,
    pub window_kind_id: String,
}

pub(crate) fn start_window(
    payload: &SetTryValues,
    operation: &semio_framework_plugin::AppOperationContext,
    transient: &FormsTryWindowTransient,
    lease: &FormsTryWindowLease,
) -> Result<TryWindowCommandOutput, Fault> {
    if !lease.matches(&payload.window_id, &payload.window_kind_id, lease.window_generation, lease.document_generation) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-values.window-stale"), "the Forms Try-values command does not address the captured window lease"));
    }
    let input_count = payload.input_count.unwrap_or(1);
    if input_count > 1 && payload.input_id.is_none() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("forms.try-values.input-id-required"), "multi-chunk Forms bulk input requires an explicit input id"));
    }
    let input_id = payload.input_id.as_deref().unwrap_or("setTryValues-single");
    let Some(input) = stage_command_input(operation, lease, "setTryValues", input_id, payload.input_index.unwrap_or(0), input_count, payload.values_json.owner())? else {
        return Ok(TryWindowCommandOutput { emit: Emit::default(), transient: None });
    };
    start_bulk_window(input.source, &input.operation, transient, lease)
}

pub fn handle(_payload: &SetTryValues, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::default())
}
