//! Norm command — `remove-item`.

use crate::op::En1993Mutation;
use crate::En1993Snapshot;
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
pub fn handle(payload: &RemoveItem, doc: &ArtifactView<'_, En1993Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1993Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_remove_item(doc.snapshot, &payload.path, payload.index as usize, |base, target| En1993Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
