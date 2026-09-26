//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1999Snapshot, En1999Mutation};
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
/// 🎨️ Replaces the live document with the named example's snapshot factory (SI subject), or clears it when the id is empty.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1999Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1999Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => En1999Snapshot::default(),
        id if id == crate::aluminium_roof_purlin::ID => crate::aluminium_roof_purlin::snapshot(),
        id if id == crate::noncompliant_multi_fail::ID => crate::noncompliant_multi_fail::snapshot(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
//#endregion 🔖️Handler
