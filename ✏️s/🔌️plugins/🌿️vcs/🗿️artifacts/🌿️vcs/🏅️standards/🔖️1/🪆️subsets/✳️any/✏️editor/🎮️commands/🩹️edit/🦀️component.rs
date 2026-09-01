//! 🩹️ 🩹️ VCS play app commands command — `edit`.

use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Helpers
fn vcs_demo_projection_diff_operations(current: &VcsSnapshot, next: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    use crate::artifacts::vcs::mutations::{add_tag, change_counter, change_notes, change_status, remove_tag, rename_vcs};
    use std::collections::BTreeSet;
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
    let current_tags = current.tags.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let next_tags = next.tags.iter().map(String::as_str).collect::<BTreeSet<_>>();
    for tag in &next.tags {
        if !current_tags.contains(tag.as_str()) {
            operations.push(add_tag(tag.clone()));
        }
    }
    for tag in &current.tags {
        if !next_tags.contains(tag.as_str()) {
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
pub(crate) fn text_edit_operations(text: &str, current: &VcsSnapshot) -> Emit<VcsDemoMutation, VcsDemoConfigMutation> {
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

/// 🩹️ Alias for [`text_edit::TextEdit`] — same payload shape, same handler body.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "edit")]
pub struct Edit {
    pub text: String,
}

pub fn handle(payload: &Edit, doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(text_edit_operations(&payload.text, doc.snapshot))
}
