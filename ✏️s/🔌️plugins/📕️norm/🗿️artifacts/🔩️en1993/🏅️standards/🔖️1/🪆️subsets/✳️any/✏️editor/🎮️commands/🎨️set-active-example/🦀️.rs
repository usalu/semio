//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1993Mutation, En1993Snapshot};
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
/// 🎨️ Replaces the live document with the named example snapshot constructors.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1993Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1993Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => En1993Snapshot::default(),
        id if id == crate::heb240_compliant::ID => En1993Snapshot::compliant_heb240_frame(),
        id if id == crate::high_strength_connection::ID => En1993Snapshot::noncompliant_overloaded_frame(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
//#endregion 🔖️Handler
