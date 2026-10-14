//! Norm command — `set-field`.

use semio_s_artifact_norm_iso16757_mutations::Iso16757Mutation;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
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
pub fn handle(payload: &SetField, doc: &ArtifactView<'_, Iso16757Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Iso16757Mutation, NoConfigMutation>, Fault> {
    semio_s_plugin_norm_app_surface::dispatch_set_field(doc.snapshot, &payload.path, &payload.value_json, |document, edit| semio_s_artifact_norm_iso16757_mutations::resolve_edit(document, edit))
}
//#endregion 🔖️Handler
