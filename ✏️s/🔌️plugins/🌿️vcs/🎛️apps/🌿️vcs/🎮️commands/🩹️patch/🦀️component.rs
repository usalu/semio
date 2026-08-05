//! 🩹️ VCS play app commands — projection field patches and whole-document JSON edits.

use crate::apps::vcs::config::{VcsDemoConfig, VcsDemoConfigOperation};
use crate::artifacts::vcs::{op::VcsDemoOperation, VcsDemoProjection};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
/// 🩹️ Builds the `VcsDemoOperation` for a `patchProjection` field write — mirrors
/// `shooting_ui::shot_patch_for_field`'s string-keyed field dispatch.
fn vcs_patch_operation_for_field(field: &str, value: &str) -> Option<VcsDemoOperation> {
    match field {
        "title" => Some(VcsDemoOperation::SetTitle { title: value.into() }),
        "counter" => value.parse::<i64>().ok().map(|counter| VcsDemoOperation::SetCounter { counter }),
        "status" => Some(VcsDemoOperation::SetStatus { status: value.into() }),
        "notes" => Some(VcsDemoOperation::SetNotes { notes: value.into() }),
        _ => None,
    }
}

fn vcs_demo_projection_diff_operations(current: &VcsDemoProjection, next: &VcsDemoProjection) -> Vec<VcsDemoOperation> {
    let mut operations = Vec::new();
    if next.title != current.title {
        operations.push(VcsDemoOperation::SetTitle { title: next.title.clone() });
    }
    if next.counter != current.counter {
        operations.push(VcsDemoOperation::SetCounter { counter: next.counter });
    }
    if next.status != current.status {
        operations.push(VcsDemoOperation::SetStatus { status: next.status.clone() });
    }
    if next.notes != current.notes {
        operations.push(VcsDemoOperation::SetNotes { notes: next.notes.clone() });
    }
    for tag in &next.tags {
        if !current.tags.contains(tag) {
            operations.push(VcsDemoOperation::AddTag { tag: tag.clone() });
        }
    }
    for tag in &current.tags {
        if !next.tags.contains(tag) {
            operations.push(VcsDemoOperation::RemoveTag { tag: tag.clone() });
        }
    }
    operations
}
//#endregion 🔖️Helpers

//#region 🔖️PatchProjection
pub mod patch_projection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-projection")]
    pub struct PatchProjection {
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchProjection, _doc: &DocumentView<'_, VcsDemoProjection>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoOperation, VcsDemoConfigOperation>, Fault> {
        match vcs_patch_operation_for_field(&payload.field, &payload.value) {
            Some(operation) => Ok(Emit::operations(vec![operation])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchProjection

//#region 🔖️TextEdit
pub mod text_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "text-edit")]
    pub struct TextEdit {
        pub text: String,
    }

    pub fn handle(payload: &TextEdit, doc: &DocumentView<'_, VcsDemoProjection>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoOperation, VcsDemoConfigOperation>, Fault> {
        Ok(text_edit_operations(&payload.text, doc.projection))
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

    pub fn handle(payload: &Edit, doc: &DocumentView<'_, VcsDemoProjection>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoOperation, VcsDemoConfigOperation>, Fault> {
        Ok(text_edit_operations(&payload.text, doc.projection))
    }
}
//#endregion 🔖️Edit

/// 🧩️ The former `TextEdit`/`Edit` match arm body, shared by both payload modules: parses the given
/// text as a whole `VcsDemoProjection` and emits the diff against the current one.
fn text_edit_operations(text: &str, current: &VcsDemoProjection) -> Emit<VcsDemoOperation, VcsDemoConfigOperation> {
    match serde_json::from_str::<VcsDemoProjection>(text) {
        Ok(next_projection) => {
            let operations = vcs_demo_projection_diff_operations(current, &next_projection);
            if operations.is_empty() {
                Emit::default()
            } else {
                Emit::operations(operations)
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
        store::test_support::assert_op_line_round_trip(&VcsCommand::PatchProjection(patch_projection::PatchProjection { field: "title".into(), value: "Renamed".into() }));
        store::test_support::assert_op_line_round_trip(&VcsCommand::TextEdit(text_edit::TextEdit { text: "{}".into() }));
        store::test_support::assert_op_line_round_trip(&VcsCommand::Edit(edit::Edit { text: "{}".into() }));
    }

    #[test]
    fn vcs_demo_command_op_binary_agrees_with_text() {
        store::test_support::assert_op_text_binary_equivalence(&VcsCommand::PatchProjection(patch_projection::PatchProjection { field: "counter".into(), value: "3".into() }));
    }

    #[test]
    fn text_edit_action_persists_projection_changes() {
        let mut instance = app();
        let before = instance.projection().expect("materialize projection");
        let mut edited = before.clone();
        edited.title = "Edited via JSON".into();
        edited.counter = before.counter + 41;
        edited.tags.push("edited-in-place".into());
        let text = serde_json::to_string_pretty(&edited).unwrap();
        let result = dispatch(&mut instance, VcsCommand::TextEdit(text_edit::TextEdit { text }));
        assert!(!result.operations.is_empty());
        let after = instance.projection().expect("materialize projection");
        assert_eq!(after.title, "Edited via JSON");
        assert_eq!(after.counter, before.counter + 41);
        assert!(after.tags.contains(&"edited-in-place".to_string()));
    }

    #[test]
    fn edit_action_is_alias_for_text_edit() {
        let mut instance = app();
        let before = instance.projection().expect("materialize projection");
        let mut edited = before;
        edited.status = "reviewed".into();
        let text = serde_json::to_string(&edited).unwrap();
        let result = dispatch(&mut instance, VcsCommand::Edit(edit::Edit { text }));
        assert!(!result.operations.is_empty());
        assert_eq!(instance.projection().expect("materialize projection").status, "reviewed");
    }
}
//#endregion 🧪️Tests
