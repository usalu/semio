//! 📝️ VCS play app — the editor window: counter/commit/branch/undo/redo actions plus a projection summary.

use crate::VcsSnapshot;
use crate::editor::vcs::terminology::VcsPlayLabels;
use crate::editor::vcs::{ui_fixed_label, ui_node_list, vcs_action};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::{built_text_node, Buildable, BuiltNode, HasBase, HasChildren, Label, LocalizedLabel, PluginAssemblyError, SurfaceKind, Trigger, UiAssemblyResult, UiText, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const VCS_PLAY_WINDOW_EDITOR: &str = "vcs-editor";
pub const VCS_PLAY_BODY_EDITOR: &str = "vcs.play.editor";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::vcs::create_vcs_app`.
pub fn definition() -> WindowKindDefinition {
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
fn ui_error(detail: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", detail)
}

/// 🏷️ Admits one runtime-data string as a fixed-capacity UI label.
fn data_label(value: String) -> UiAssemblyResult<ui::Label> {
    ui::Label::try_from(value).map_err(|_| ui_error("vcs editor data label admission failed"))
}

fn editor_text(label: ui::Label) -> UiAssemblyResult<BuiltNode> {
    ui::text(label).try_build().map_err(|_| ui_error("vcs editor text admission failed"))
}

fn editor_data_text(value: String) -> UiAssemblyResult<BuiltNode> {
    built_text_node(Label::data(value)).map_err(|_| ui_error("vcs editor data text admission failed"))
}

fn editor_row(id: &str, children: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<BuiltNode> {
    let children = ui_node_list(children)?;
    let builder = ui::row().try_id(id).map_err(|_| ui_error("vcs editor row id admission failed"))?;
    builder.try_children(children).map_err(|_| ui_error("vcs editor row children admission failed"))?.try_build().map_err(|_| ui_error("vcs editor row admission failed"))
}

fn editor_button(id: &str, icon_id: &str, label: ui::Label, action: &str) -> UiAssemblyResult<BuiltNode> {
    let icon = UiText::try_from_str(icon_id).ok_or_else(|| ui_error("vcs editor button icon admission failed"))?;
    let (action_id, _) = vcs_action(action, None)?;
    let builder = ui::button(label).icon(icon);
    let builder = builder.try_id(format!("vcs-play-editor.{id}")).map_err(|_| ui_error("vcs editor button id admission failed"))?;
    let builder = builder.try_on(Trigger::Activate, action_id).map_err(|_| ui_error("vcs editor button binding admission failed"))?;
    builder.try_build().map_err(|_| ui_error("vcs editor button admission failed"))
}

/// 🖥️ One button per row where the label is dynamic-width (counter), two per row otherwise: the
/// framework's horizontal stack gives every child an equal flex-1 share and buttons don't shrink below
/// their label width, so a wide/growing label overflows and overlaps its neighbor in the (narrower)
/// Editor panel of the default layout. A leading heading clears the window's Action/Viewport tab chrome,
/// which otherwise overlaps content placed flush at the panel top.
pub fn render(projection: &VcsSnapshot, labels: &VcsPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let heading = editor_text(ui_fixed_label(labels.actions)?)?;
    let increment_row = editor_row("vcs-play-editor.increment-row", [editor_button("increment", "plus", data_label(format!("+ {} ({})", labels.counter.as_str(), projection.counter))?, "incrementCounter")])?;
    let commit_row = editor_row(
        "vcs-play-editor.commit-row",
        [editor_button("commit", "git-commit", ui_fixed_label(labels.commit)?, "commitCheckpoint"), editor_button("new-alternative", "git-branch", ui_fixed_label(labels.branch)?, "createAlternative")],
    )?;
    let history_row = editor_row("vcs-play-editor.history-row", [editor_button("undo", "undo", ui_fixed_label(labels.undo)?, "undo"), editor_button("redo", "redo", ui_fixed_label(labels.redo)?, "redo")])?;
    let summary_children = ui_node_list([
        editor_data_text(format!("{} · {} {}", projection.title, labels.counter.as_str(), projection.counter)),
        editor_data_text(if projection.notes.is_empty() { "—".to_string() } else { projection.notes.clone() }),
    ])?;
    let summary = ui::column().try_id("vcs-play-editor.summary").map_err(|_| ui_error("vcs editor summary id admission failed"))?;
    let summary = summary.try_children(summary_children).map_err(|_| ui_error("vcs editor summary children admission failed"))?.try_build().map_err(|_| ui_error("vcs editor summary admission failed"))?;
    let root_children = ui_node_list([Ok(heading), Ok(increment_row), Ok(commit_row), Ok(history_row), Ok(summary)])?;
    let root = ui::column().try_id("vcs-play-editor").map_err(|_| ui_error("vcs editor root id admission failed"))?;
    root.try_children(root_children).map_err(|_| ui_error("vcs editor root children admission failed"))?.try_build().map_err(|_| ui_error("vcs editor root admission failed"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_editor_scene() {
        let mut instance = app().await;
        let json = render_body(&mut instance, VCS_PLAY_BODY_EDITOR).await;
        assert!(!json.contains("text-editor"), "editor must no longer be a raw JSON editor: {json}");
        for action in ["incrementCounter", "commitCheckpoint", "undo", "redo", "createAlternative"] {
            assert!(json.contains(action), "missing editor button for {action}: {json}");
        }
        assert!(json.contains(" · Counter "), "missing title/counter summary: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn vcs_labels_resolve_native_english_by_default() {
        let mut instance = app().await;
        let json = render_body(&mut instance, VCS_PLAY_BODY_EDITOR).await;
        assert!(json.contains("Actions"));
        assert!(json.contains("Commit"));
        assert!(json.contains("Branch"));
        assert!(json.contains("Undo"));
        assert!(json.contains("Redo"));
        assert!(json.contains("Counter"));
    }
}
//#endregion 🧪️Tests
