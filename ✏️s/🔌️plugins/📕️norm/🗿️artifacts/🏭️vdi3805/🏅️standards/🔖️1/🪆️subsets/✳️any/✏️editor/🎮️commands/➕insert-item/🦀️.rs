//! Norm command — `insert-item`.

use crate::op::Vdi3805Mutation;
use crate::Vdi3805Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "insert-item")]
pub struct InsertItem {
    pub path: String,
    pub index: u32,
    pub value_json: Option<String>,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &InsertItem, doc: &ArtifactView<'_, Vdi3805Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Vdi3805Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_insert_item(doc.snapshot, &payload.path, payload.index as usize, payload.value_json.as_deref(), |base, target| Vdi3805Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
