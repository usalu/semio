//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{Din18599Snapshot, Din18599Mutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
/// 🎨️ Replaces the live document with the named example's `PRIMARY_TEXT`, or clears it when the id is empty.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Din18599Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din18599Mutation, NoConfigMutation>, Fault> {
    let text = match payload.example_id.trim() {
        "" => <Din18599Snapshot as store::ArtifactDsl>::print_dsl(&Din18599Snapshot::default()),
        id if id == crate::examples::demo::ID => crate::examples::demo::PRIMARY_TEXT.to_string(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { text }, doc, cfg)
}
//#endregion 🔖️Handler
