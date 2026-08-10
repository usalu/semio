//! 🗂️ VCS play app commands — the document-tree checkpoint selection (config-only).

use crate::apps::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        Ok(Emit::config(vec![VcsDemoConfigMutation::SetSelection { checkpoint_ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vcs::testkit::{app, dispatch, render};
    use crate::apps::vcs::{VcsCommand, VCS_PLAY_BODY_DOCUMENT};
    use semio_framework_plugin::PluginApp;

    #[test]
    fn vcs_demo_command_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::SetSelection(set_selection::SetSelection { ids: vec!["checkpoint-1".into()] }));
    }

    /// 👁️ `setSelection` is config-only: it must drive `cfg.selected_checkpoint_ids` (rendered into the
    /// document tree's `selected` ids) without ever touching the document store.
    #[test]
    fn set_selection_drives_config_and_emits_no_mutations() {
        let mut instance = app();
        let checkpoint_id = crate::apps::vcs::testkit::seeded_envelope(&instance).vcs.checkpoints[0].id.clone();
        let result = dispatch(&mut instance, VcsCommand::SetSelection(set_selection::SetSelection { ids: vec![checkpoint_id.clone()] }));
        assert!(result.mutations.is_empty(), "setSelection mutates only ephemeral config, never the document");
        let json = render(&mut instance, VCS_PLAY_BODY_DOCUMENT);
        assert!(json.contains(&checkpoint_id));
    }

    /// 🎥️ Config-only commands (selection/locale) never create a document-store undo step — mirrors
    /// `shooting_ui`'s `camera_drag_never_creates_a_document_undo_step`.
    #[test]
    fn set_selection_never_creates_a_document_undo_step() {
        use semio_framework_plugin::testkit::meta;
        let mut instance = app();
        let before = instance.snapshot().expect("materialize snapshot").counter;
        dispatch(&mut instance, VcsCommand::SetSelection(set_selection::SetSelection { ids: vec!["checkpoint-1".into()] }));
        instance.handle_action("undo", None, &meta("local")).expect("undo (no-op: nothing on the document store to undo)");
        assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before, "document undo has nothing to revert — selection never touched the document");
    }
}
//#endregion 🧪️Tests
