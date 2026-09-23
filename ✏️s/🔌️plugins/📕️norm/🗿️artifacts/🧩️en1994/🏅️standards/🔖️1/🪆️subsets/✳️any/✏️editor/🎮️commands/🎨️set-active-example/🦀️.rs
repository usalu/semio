//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1994Snapshot, En1994Mutation};
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
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1994Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1994Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => En1994Snapshot::default(),
        id if id == crate::composite_bridge_girder::ID => <En1994Snapshot as store::ArtifactDsl>::parse_dsl(crate::composite_bridge_girder::PRIMARY_TEXT)
            .map_err(|error| Fault::from(format!("set-active-example: invalid example text: {error:?}")))?,
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
//#endregion 🔖️Handler
