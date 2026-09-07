//! 🔍️ VCS play app panel — the inspector: title/counter/status/notes/tags fields for the document.

use crate::artifacts::vcs::VcsSnapshot;
use crate::editor::vcs::terminology::VcsPlayLabels;
use crate::editor::vcs::{ui_fixed_label, ui_node_list, ui_value_map, ui_value_text, vcs_action};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::{
    tree_item_desc, Buildable, BuiltNode, HasBase, HasChildren, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, Trigger, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const VCS_PLAY_BODY_INSPECTION: &str = "vcs.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(VCS_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn ui_error(detail: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", detail)
}

/// ✍️ One inspector row: a labelled tree item carrying the field's own live input control, whose
/// change dispatches `patchSnapshot` for exactly that field.
fn field_row(field: &'static str, label: LabelText, kind: ui::InputKind, value: String) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = vcs_action("patchSnapshot", Some(ui_value_map([("field", ui_value_text(field)?)])?))?;
    let args = args.ok_or_else(|| ui_error("vcs inspector action arguments missing"))?;
    let control = ui::input(kind).value(UiText::try_from_string(value).map_err(|_| ui_error("vcs inspector value admission failed"))?).commit(UiText::try_from_str("blur").ok_or_else(|| ui_error("vcs inspector commit admission failed"))?);
    let control = control.try_id(format!("vcs-play-inspector.{field}.input")).map_err(|_| ui_error("vcs inspector input id admission failed"))?;
    let control = control.try_on_with(Trigger::Change, action, args).map_err(|_| ui_error("vcs inspector input binding admission failed"))?;
    let control = control.try_build().map_err(|_| ui_error("vcs inspector input admission failed"))?;
    let row = semio_framework_ui_contract::tree_item(ui_fixed_label(label)?).try_id(format!("vcs-play-inspector.{field}")).map_err(|_| ui_error("vcs inspector row id admission failed"))?;
    row.try_child(control).map_err(|_| ui_error("vcs inspector row child admission failed"))?.try_build().map_err(|_| ui_error("vcs inspector row admission failed"))
}

pub fn render(projection: &VcsSnapshot, labels: &VcsPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let items = ui_node_list([
        field_row("title", labels.title, ui::InputKind::Text, projection.title.clone()),
        field_row("counter", labels.counter, ui::InputKind::Number, projection.counter.to_string()),
        field_row("status", labels.status, ui::InputKind::Text, projection.status.clone()),
        field_row("notes", labels.notes, ui::InputKind::Text, projection.notes.clone()),
        tree_item_desc("vcs-play-inspector.tags", labels.tags.as_str(), Some(projection.tags.join(", "))),
    ])?;
    PanelTreeBuilder::new("vcs-play-inspector")?.section("vcs-play-inspector", Some(ui_fixed_label(labels.title)?), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn vcs_labels_resolve_native_english_by_default() {
        let mut instance = app().await;
        let json = render_body(&mut instance, VCS_PLAY_BODY_INSPECTION).await;
        assert!(json.contains("Title"));
        assert!(json.contains("Status"));
        assert!(json.contains("Notes"));
        assert!(json.contains("Tags"));
    }
}
//#endregion 🧪️Tests
