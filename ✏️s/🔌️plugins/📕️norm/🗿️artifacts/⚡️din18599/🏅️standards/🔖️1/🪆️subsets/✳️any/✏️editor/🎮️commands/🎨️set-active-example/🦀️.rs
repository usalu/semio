//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{Din18599Mutation, Din18599Snapshot};
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
/// 🎨️ Replaces the live document with a named example's `PRIMARY_TEXT` DSL asset.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Din18599Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din18599Mutation, NoConfigMutation>, Fault> {
    let text = match payload.example_id.trim() {
        "" | "demo" => crate::examples::demo::PRIMARY_TEXT,
        "compliant-detached" => crate::examples::compliant_detached::PRIMARY_TEXT,
        "noncompliant-detached" => crate::examples::noncompliant_detached::PRIMARY_TEXT,
        "compliant-two-zone" => crate::examples::compliant_two_zone::PRIMARY_TEXT,
        "cooled-office" => crate::examples::cooled_office::PRIMARY_TEXT,
        _ => return Ok(Emit::default()),
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { text: crate::document::escape_op_text_field(text) }, doc, cfg)
}
//#endregion 🔖️Handler
