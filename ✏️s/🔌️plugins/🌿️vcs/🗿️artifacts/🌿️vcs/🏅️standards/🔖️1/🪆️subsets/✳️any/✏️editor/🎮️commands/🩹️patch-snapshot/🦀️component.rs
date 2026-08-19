//! 🩹️ 🩹️ VCS play app commands command — `patch-snapshot`.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
/// 🩹️ Builds the `VcsDemoMutation` for a `patchSnapshot` field write — mirrors
/// `shooting_ui::shot_patch_for_field`'s string-keyed field dispatch.
async fn vcs_patch_operation_for_field(field: &str, value: &str) -> Option<VcsDemoMutation> {
    use crate::artifacts::vcs::mutations::{change_counter, change_notes, change_status, rename_vcs};
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-snapshot")]
pub struct PatchSnapshot {
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchSnapshot, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    match vcs_patch_operation_for_field(&payload.field, &payload.value) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::commands::{edit, text_edit};
    use crate::editor::vcs::testkit::{app, dispatch};
    use crate::editor::vcs::VcsCommand;

    #[test]
    async fn vcs_demo_command_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::PatchSnapshot(PatchSnapshot { field: "title".into(), value: "Renamed".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::TextEdit(text_edit::TextEdit { text: "{}".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::Edit(edit::Edit { text: "{}".into() }));
    }

    #[test]
    async fn vcs_demo_command_op_binary_agrees_with_text() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&VcsCommand::PatchSnapshot(PatchSnapshot { field: "counter".into(), value: "3".into() }));
    }

    #[test]
    async fn text_edit_action_persists_projection_changes() {
        let mut instance = app();
        let before = instance.snapshot().expect("materialize snapshot");
        let mut edited = before.clone();
        edited.title = "Edited via JSON".into();
        edited.counter = before.counter + 41;
        edited.tags.push("edited-in-place".into());
        let text = serde_json::to_string_pretty(&edited).unwrap();
        let result = dispatch(&mut instance, VcsCommand::TextEdit(text_edit::TextEdit { text }));
        assert!(!result.mutations.is_empty());
        let after = instance.snapshot().expect("materialize snapshot");
        assert_eq!(after.title, "Edited via JSON");
        assert_eq!(after.counter, before.counter + 41);
        assert!(after.tags.contains(&"edited-in-place".to_string()));
    }

    #[test]
    async fn edit_action_is_alias_for_text_edit() {
        let mut instance = app();
        let before = instance.snapshot().expect("materialize snapshot");
        let mut edited = before;
        edited.status = "reviewed".into();
        let text = serde_json::to_string(&edited).unwrap();
        let result = dispatch(&mut instance, VcsCommand::Edit(edit::Edit { text }));
        assert!(!result.mutations.is_empty());
        assert_eq!(instance.snapshot().expect("materialize snapshot").status, "reviewed");
    }
}
//#endregion 🧪️Tests
