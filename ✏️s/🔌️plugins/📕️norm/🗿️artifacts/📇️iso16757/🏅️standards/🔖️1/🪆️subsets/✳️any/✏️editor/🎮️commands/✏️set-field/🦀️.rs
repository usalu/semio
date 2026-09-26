//! Norm command — `set-field`.

use crate::op::Iso16757Mutation;
use crate::Iso16757Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-field")]
pub struct SetField {
    pub path: String,
    pub value_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetField, doc: &ArtifactView<'_, Iso16757Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Iso16757Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_set_field(doc.snapshot, &payload.path, &payload.value_json, |base, target| Iso16757Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
