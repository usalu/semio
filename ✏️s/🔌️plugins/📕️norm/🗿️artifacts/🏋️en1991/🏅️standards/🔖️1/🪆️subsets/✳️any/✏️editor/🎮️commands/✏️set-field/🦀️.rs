//! Norm command — `set-field`.

use crate::op::En1991Mutation;
use crate::En1991Snapshot;
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
pub fn handle(payload: &SetField, doc: &ArtifactView<'_, En1991Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1991Mutation, NoConfigMutation>, Fault> {
    let path = crate::field_meta::flatten_editor_path(&payload.path);
    crate::app_surface::dispatch_set_field(doc.snapshot, &path, &payload.value_json, |base, target| En1991Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
