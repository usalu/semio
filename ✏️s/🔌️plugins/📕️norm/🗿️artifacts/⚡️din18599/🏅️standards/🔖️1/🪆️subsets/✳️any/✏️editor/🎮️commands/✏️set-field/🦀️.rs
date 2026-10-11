//! Norm command — `set-field`.

use crate::standards::v1::subsets::any::schema::mutations::Din18599Mutation;
use crate::Din18599Snapshot;
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
pub fn handle(payload: &SetField, doc: &ArtifactView<'_, Din18599Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din18599Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_set_field(doc.snapshot, &payload.path, &payload.value_json, |document, edit| crate::mutations::resolve_edit(document, edit))
}
//#endregion 🔖️Handler
