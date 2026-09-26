//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1991Snapshot, En1991Mutation};
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
/// 🎨️ Replaces the live document with the named example subject.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1991Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1991Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => En1991Snapshot::default(),
        id if id == crate::de_office_compliant::ID => crate::example_subjects::de_office_compliant(),
        id if id == crate::multi_fail_noncompliant::ID => crate::example_subjects::multi_fail_noncompliant(),
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
//#endregion 🔖️Handler
