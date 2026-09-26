//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{En1995Snapshot, En1995Mutation};
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
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, En1995Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1995Mutation, NoConfigMutation>, Fault> {
    let id = payload.example_id.trim();
    let snapshot = if id.is_empty() {
        En1995Snapshot::default()
    } else {
        let Some(example) = crate::examples().into_iter().find(|example| example.id() == id) else {
            return Ok(Emit::default());
        };
        <En1995Snapshot as store::ArtifactDsl>::parse_dsl(&example.document())
            .map_err(|error| Fault::from(format!("set-active-example: invalid example text: {error:?}")))?
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
//#endregion 🔖️Handler
