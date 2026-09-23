//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{Vdi3805Snapshot, Vdi3805Mutation};
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
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Vdi3805Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Vdi3805Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => Vdi3805Snapshot::default(),
        id if id == crate::examples::demo::ID => <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT)
            .map_err(|error| Fault::from(format!("set-active-example: invalid example text: {error:?}")))?,
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
//#endregion 🔖️Handler
