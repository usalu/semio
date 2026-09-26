//! Norm command — `set-field`.

use crate::op::En1995Mutation;
use crate::En1995Snapshot;
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
pub fn handle(payload: &SetField, doc: &ArtifactView<'_, En1995Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1995Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_set_field(doc.snapshot, &payload.path, &payload.value_json, |base, target| En1995Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
