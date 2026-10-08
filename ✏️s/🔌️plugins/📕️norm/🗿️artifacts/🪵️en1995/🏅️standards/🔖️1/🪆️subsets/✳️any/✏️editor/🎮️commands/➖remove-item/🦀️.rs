//! Norm command — `remove-item`.

use crate::standards::v1::subsets::any::schema::mutations::En1995Mutation;
use crate::En1995Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "remove-item")]
pub struct RemoveItem {
    pub path: String,
    pub index: u32,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &RemoveItem, doc: &ArtifactView<'_, En1995Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1995Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_remove_item(doc.snapshot, &payload.path, payload.index as usize, |document, edit| crate::mutations::EDIT_RULES.resolve(document, edit))
}
//#endregion 🔖️Handler
