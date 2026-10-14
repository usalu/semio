//! Norm command — `set-field`.

use semio_s_artifact_norm_vdi3805_mutations::Vdi3805Mutation;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "set-field")]
pub struct SetField {
    pub path: String,
    pub value_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetField, doc: &ArtifactView<'_, Vdi3805Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Vdi3805Mutation, NoConfigMutation>, Fault> {
    semio_s_plugin_norm_app_surface::dispatch_set_field(doc.snapshot, &payload.path, &payload.value_json, |document, edit| semio_s_artifact_norm_vdi3805_mutations::resolve_edit(document, edit))
}
//#endregion 🔖️Handler
