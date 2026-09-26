//! Norm command — `remove-item`.

use crate::op::Iso16757Mutation;
use crate::Iso16757Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-item")]
pub struct RemoveItem {
    pub path: String,
    pub index: u32,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &RemoveItem, doc: &ArtifactView<'_, Iso16757Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Iso16757Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_remove_item(doc.snapshot, &payload.path, payload.index as usize, |base, target| Iso16757Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
