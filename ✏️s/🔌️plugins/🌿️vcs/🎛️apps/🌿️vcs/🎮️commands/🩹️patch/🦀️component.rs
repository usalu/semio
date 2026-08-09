//! 🩹️ VCS play app commands — projection field patches and whole-document JSON edits.

use crate::apps::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
/// 🩹️ Builds the `VcsDemoMutation` for a `patchSnapshot` field write — mirrors
/// `shooting_ui::shot_patch_for_field`'s string-keyed field dispatch.
fn vcs_patch_operation_for_field(field: &str, value: &str) -> Option<VcsDemoMutation> {
    match field {
        "title" => Some(VcsDemoMutation::SetTitle { title: value.into() }),
        "counter" => value.parse::<i64>().ok().map(|counter| VcsDemoMutation::SetCounter { counter }),
        "status" => Some(VcsDemoMutation::SetStatus { status: value.into() }),
        "notes" => Some(VcsDemoMutation::SetNotes { notes: value.into() }),
        _ => None,
    }
}

fn vcs_demo_projection_diff_operations(current: &VcsSnapshot, next: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    let mut operations = Vec::new();
    if next.title != current.title {
        operations.push(VcsDemoMutation::SetTitle { title: next.title.clone() });
    }
    if next.counter != current.counter {
        operations.push(VcsDemoMutation::SetCounter { counter: next.counter });
    }
    if next.status != current.status {
        operations.push(VcsDemoMutation::SetStatus { status: next.status.clone() });
    }
    if next.notes != current.notes {
        operations.push(VcsDemoMutation::SetNotes { notes: next.notes.clone() });
    }
    for tag in &next.tags {
        if !current.tags.contains(tag) {
            operations.push(VcsDemoMutation::AddTag { tag: tag.clone() });
        }
    }
    for tag in &current.tags {
        if !next.tags.contains(tag) {
            operations.push(VcsDemoMutation::RemoveTag { tag: tag.clone() });
        }
    }
    operations
}
//#endregion 🔖️Helpers

//#region 🔖️PatchSnapshot
pub mod patch_snapshot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-snapshot")]
    pub struct PatchSnapshot {
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchSnapshot, _doc: &DocumentView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        match vcs_patch_operation_for_field(&payload.field, &payload.value) {
            Some(operation) => Ok(Emit::mutations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchSnapshot

//#region 🔖️TextEdit
pub mod text_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "text-edit")]
    pub struct TextEdit {
        pub text: String,
    }

    pub fn handle(payload: &TextEdit, doc: &DocumentView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        Ok(text_edit_operations(&payload.text, doc.snapshot))
    }
}
//#endregion 🔖️TextEdit

//#region 🔖️Edit
pub mod edit {
    use super::*;

    /// 🩹️ Alias for [`text_edit::TextEdit`] — same payload shape, same handler body.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "edit")]
    pub struct Edit {
        pub text: String,
    }

    pub fn handle(payload: &Edit, doc: &DocumentView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        Ok(text_edit_operations(&payload.text, doc.snapshot))
    }
}
//#endregion 🔖️Edit

/// 🧩️ The former `TextEdit`/`Edit` match arm body, shared by both payload modules: parses the given
/// text as a whole `VcsSnapshot` and emits the diff against the current one.
fn text_edit_operations(text: &str, current: &VcsSnapshot) -> Emit<VcsDemoMutation, VcsDemoConfigMutation> {
    match serde_json::from_str::<VcsSnapshot>(text) {
        Ok(next_projection) => {
            let operations = vcs_demo_projection_diff_operations(current, &next_projection);
            if operations.is_empty() {
                Emit::default()
            } else {
                Emit::mutations(operations)
            }
        }
        Err(_) => Emit::default(),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vcs::testkit::{app, dispatch};
    use crate::apps::vcs::VcsCommand;

    #[test]
    fn vcs_demo_command_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field: "title".into(), value: "Renamed".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::TextEdit(text_edit::TextEdit { text: "{}".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::Edit(edit::Edit { text: "{}".into() }));
    }

    #[test]
    fn vcs_demo_command_op_binary_agrees_with_text() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field: "counter".into(), value: "3".into() }));
    }

    #[test]
    fn text_edit_action_persists_projection_changes() {
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
    fn edit_action_is_alias_for_text_edit() {
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
