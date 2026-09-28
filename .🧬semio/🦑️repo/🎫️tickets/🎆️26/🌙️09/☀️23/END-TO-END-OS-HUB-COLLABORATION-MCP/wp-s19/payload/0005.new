//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{Iso16757Snapshot, Iso16757Mutation};
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
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
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Iso16757Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Iso16757Mutation, NoConfigMutation>, Fault> {
    let Some(snapshot) = crate::app_surface::roster_example_snapshot(<crate::editor::iso16757::Iso16757PlayApp as ArtifactEditor>::examples(), &payload.example_id)? else {
        return Ok(Emit::default());
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
//#endregion 🔖️Handler
