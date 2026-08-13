//! 🩹️ 🩹️ VCS play app commands command — `text-edit`.

use crate::apps::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
/// 🩹️ Builds the `VcsDemoMutation` for a `patchSnapshot` field write — mirrors
/// `shooting_ui::shot_patch_for_field`'s string-keyed field dispatch.
fn vcs_patch_operation_for_field(field: &str, value: &str) -> Option<VcsDemoMutation> {
    use crate::artifacts::vcs::mutations::{change_counter, change_notes, change_status, rename_vcs};
    match field {
        "title" => Some(rename_vcs(value.into())),
        "counter" => value.parse::<i64>().ok().map(change_counter),
        "status" => Some(change_status(value.into())),
        "notes" => Some(change_notes(value.into())),
        _ => None,
    }
}

fn vcs_demo_projection_diff_operations(current: &VcsSnapshot, next: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    use crate::artifacts::vcs::mutations::{add_tag, change_counter, change_notes, change_status, remove_tag, rename_vcs};
    let mut operations = Vec::new();
    if next.title != current.title {
        operations.push(rename_vcs(next.title.clone()));
    }
    if next.counter != current.counter {
        operations.push(change_counter(next.counter));
    }
    if next.status != current.status {
        operations.push(change_status(next.status.clone()));
    }
    if next.notes != current.notes {
        operations.push(change_notes(next.notes.clone()));
    }
    for tag in &next.tags {
        if !current.tags.contains(tag) {
            operations.push(add_tag(tag.clone()));
        }
    }
    for tag in &current.tags {
        if !next.tags.contains(tag) {
            operations.push(remove_tag(tag.clone()));
        }
    }
    operations
}
//#endregion 🔖️Helpers

//#region 🔖️PatchSnapshot
//#endregion 🔖️PatchSnapshot

//#region 🔖️TextEdit
//#endregion 🔖️TextEdit

//#region 🔖️Edit
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "text-edit")]
pub struct TextEdit {
    pub text: String,
}

pub fn handle(payload: &TextEdit, doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(text_edit_operations(&payload.text, doc.snapshot))
}
