//! 🩹️ 🩹️ VCS play app commands command — `text-edit`.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
async fn vcs_demo_projection_diff_operations(current: &VcsSnapshot, next: &VcsSnapshot) -> Vec<VcsDemoMutation> {
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
async fn text_edit_operations(text: &str, current: &VcsSnapshot) -> Emit<VcsDemoMutation, VcsDemoConfigMutation> {
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

pub async fn handle(payload: &TextEdit, doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(text_edit_operations(&payload.text, doc.snapshot))
}
