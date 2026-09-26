//! Norm command — `insert-item`.

use crate::op::Din16798Mutation;
use crate::Din16798Snapshot;
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
pub fn handle(payload: &InsertItem, doc: &ArtifactView<'_, Din16798Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din16798Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_insert_item(doc.snapshot, &payload.path, payload.index as usize, payload.value_json.as_deref(), |base, target| Din16798Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
