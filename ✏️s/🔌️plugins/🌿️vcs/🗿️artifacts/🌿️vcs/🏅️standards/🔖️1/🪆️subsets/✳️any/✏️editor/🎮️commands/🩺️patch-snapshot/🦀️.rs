//! 🩹️ 🩹️ VCS play app commands command — `patch-snapshot`.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Helpers
/// 🩹️ Builds the `VcsDemoMutation` for a `patchSnapshot` field write — mirrors
/// `shooting_ui::shot_patch_for_field`'s string-keyed field dispatch.
fn vcs_patch_operation_for_field(field: &str, value: &str) -> Option<VcsDemoMutation> {
    use crate::mutations::{change_counter, change_notes, change_status, rename_vcs};
    match field {
        "title" => Some(rename_vcs(value.into())),
        "counter" => value.parse::<i64>().ok().map(change_counter),
        "status" => Some(change_status(value.into())),
        "notes" => Some(change_notes(value.into())),
        _ => None,
    }
}

//#endregion 🔖️Helpers

//#region 🔖️PatchSnapshot
//#endregion 🔖️PatchSnapshot

//#region 🔖️TextEdit
//#endregion 🔖️TextEdit

//#region 🔖️Edit
//#endregion 🔖️Edit

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-snapshot")]
pub struct PatchSnapshot {
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchSnapshot, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    match vcs_patch_operation_for_field(&payload.field, &payload.value) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
