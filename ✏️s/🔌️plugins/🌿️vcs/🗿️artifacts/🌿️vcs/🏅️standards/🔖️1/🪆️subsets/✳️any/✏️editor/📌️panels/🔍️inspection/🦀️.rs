//! 🔍️ VCS play app panel — the inspector: title/counter/status/notes/tags fields for the document.

use crate::editor::vcs::terminology::VcsPlayLabels;
use crate::editor::vcs::{ui_fixed_label, ui_node_list, vcs_action};
use crate::VcsSnapshot;
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::Buildable;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::HasBase;
use semio_framework_plugin::HasChildren;
use semio_framework_ui_locale::LabelText;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::Trigger;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiText;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;

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
/// change dispatches the concrete `action_id` command of exactly that field.
fn field_row(field: &'static str, action_id: &'static str, label: LabelText, kind: ui::InputKind, value: String) -> UiAssemblyResult<BuiltNode> {
    let (action, _) = vcs_action(action_id, None)?;
    let control = ui::input(kind).value(UiText::try_from_string(value).map_err(|_| ui_error("vcs inspector value admission failed"))?).commit(UiText::try_from_str("blur").ok_or_else(|| ui_error("vcs inspector commit admission failed"))?);
    let control = control.try_id(format!("vcs-play-inspector.{field}.input")).map_err(|_| ui_error("vcs inspector input id admission failed"))?;
    let control = control.try_on(Trigger::Change, action).map_err(|_| ui_error("vcs inspector input binding admission failed"))?;
    let control = control.try_build().map_err(|_| ui_error("vcs inspector input admission failed"))?;
    let row = semio_framework_ui_contract::tree_item(ui_fixed_label(label)?).try_id(format!("vcs-play-inspector.{field}")).map_err(|_| ui_error("vcs inspector row id admission failed"))?;
    row.try_child(control).map_err(|_| ui_error("vcs inspector row child admission failed"))?.try_build().map_err(|_| ui_error("vcs inspector row admission failed"))
}

pub fn render(projection: &VcsSnapshot, labels: &VcsPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let items = ui_node_list([
        field_row("title", "renameVcs", labels.title, ui::InputKind::Text, projection.title.clone()),
        field_row("counter", "changeCounter", labels.counter, ui::InputKind::Number, projection.counter.to_string()),
        field_row("status", "changeStatus", labels.status, ui::InputKind::Text, projection.status.clone()),
        field_row("notes", "changeNotes", labels.notes, ui::InputKind::Text, projection.notes.clone()),
        tree_item_desc("vcs-play-inspector.tags", labels.tags.as_str(), Some(projection.tags.join(", "))),
    ])?;
    PanelTreeBuilder::new("vcs-play-inspector")?.section("vcs-play-inspector", Some(ui_fixed_label(labels.title)?), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
