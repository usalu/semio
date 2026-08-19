//! 📝️ VCS play app — the editor window: counter/commit/branch/undo/redo actions plus a projection summary.

use crate::editor::vcs::terminology::VcsPlayLabels;
use crate::editor::vcs::vcs_action;
use crate::artifacts::vcs::VcsSnapshot;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, SurfaceKind, UiButtonNode, UiNode, UiPresence, UiStackNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const VCS_PLAY_WINDOW_EDITOR: &str = "vcs-editor";
pub const VCS_PLAY_BODY_EDITOR: &str = "vcs.play.editor";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::vcs::create_vcs_app`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: VCS_PLAY_WINDOW_EDITOR.into(),
        label: LocalizedLabel::native("Editor", "Editor"),
        body_key: VCS_PLAY_BODY_EDITOR.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "pen-tool".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        // 🕹️ No checkpoint tree here — the "history" interaction domain (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) is scoped to the history window only.
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn ui_stack_horizontal(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "horizontal".into(), gap: Some("tight".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

async fn editor_button(id: &str, icon_id: &str, label: impl Into<Label>, action: &str) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(format!("vcs-play-editor.{id}")), icon_id: icon_id.into(), label: label.into(), action: vcs_action(action, None), style: None, presence: UiPresence::default(), menu: None })
}

/// 🖥️ One button per row where the label is dynamic-width (counter), two per row otherwise: the
/// framework's horizontal stack gives every child an equal flex-1 share and buttons don't shrink below
/// their label width, so a wide/growing label overflows and overlaps its neighbor in the (narrower)
/// Editor panel of the default layout. A leading heading clears the window's Action/Viewport tab chrome,
/// which otherwise overlaps content placed flush at the panel top.
pub async fn render(projection: &VcsSnapshot, labels: &VcsPlayLabels) -> UiNode {
    let heading = ui_text(labels.actions);
    let increment_row = ui_stack_horizontal(vec![editor_button("increment", "plus", Label::data(format!("+ {} ({})", labels.counter.as_str(), projection.counter)), "incrementCounter")]);
    let commit_row = ui_stack_horizontal(vec![editor_button("commit", "git-commit", labels.commit, "commitCheckpoint"), editor_button("new-alternative", "git-branch", labels.branch, "createAlternative")]);
    let history_row = ui_stack_horizontal(vec![editor_button("undo", "undo", labels.undo, "undo"), editor_button("redo", "redo", labels.redo, "redo")]);
    let summary =
        ui_stack_vertical(vec![ui_text(Label::data(format!("{} · {} {}", projection.title, labels.counter.as_str(), projection.counter))), ui_text(Label::data(if projection.notes.is_empty() { "—".to_string() } else { projection.notes.clone() }))]);
    ui_stack_vertical(vec![heading, increment_row, commit_row, history_row, summary])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::testkit::{app, render as render_body};

    #[test]
    async fn renders_editor_scene() {
        let mut instance = app();
        let json = render_body(&mut instance, VCS_PLAY_BODY_EDITOR);
        assert!(!json.contains("text-editor"), "editor must no longer be a raw JSON editor: {json}");
        for action in ["incrementCounter", "commitCheckpoint", "undo", "redo", "createAlternative"] {
            assert!(json.contains(action), "missing editor button for {action}: {json}");
        }
        assert!(json.contains(" · Counter "), "missing title/counter summary: {json}");
    }

    #[test]
    async fn vcs_labels_resolve_native_english_by_default() {
        let mut instance = app();
        let json = render_body(&mut instance, VCS_PLAY_BODY_EDITOR);
        assert!(json.contains("Actions"));
        assert!(json.contains("Commit"));
        assert!(json.contains("Branch"));
        assert!(json.contains("Undo"));
        assert!(json.contains("Redo"));
        assert!(json.contains("Counter"));
    }
}
//#endregion 🧪️Tests
