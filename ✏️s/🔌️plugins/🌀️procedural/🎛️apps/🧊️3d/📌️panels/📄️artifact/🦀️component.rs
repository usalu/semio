//! 📄️ Procedural3d play app panel — the document tree: widgets of the current fixture.

use crate::apps::procedural3d::procedural3d_action;
use crate::apps::procedural3d::terminology::Procedural3dLabels;
use crate::artifacts::procedural3d::widget_id;
use flow::FlowFixture;
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_BODY_DOCUMENT: &str = "procedural.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(PROCEDURAL_3D_PLAY_BODY_DOCUMENT.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🌳️ SDK's `tree_item_with_action` plus an icon id — this app's document tree carries icons per item.
fn tree_item_with_icon(id: impl Into<String>, label: impl Into<Label>, icon_id: Option<&str>, action: semio_framework_plugin::ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: icon_id.map(Into::into), menu: None, ..tree_item_with_action(id, label, None, action) }
}

pub fn render(fixture: &FlowFixture, selected_node_ids: &[String], labels: &Procedural3dLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            tree_item_with_icon(format!("procedural-widget:{id}"), Label::data(id.clone()), Some("cpu"), procedural3d_action("setSelection", Some(json!({ "ids": [id] }))))
        })
        .collect();
    PanelTreeBuilder::new("procedural-play-document").section("procedural-play-document.widgets", Some(labels.widgets.into()), true, items).selected(selected_node_ids.iter().map(|id| format!("procedural-widget:{id}")).collect()).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn document_lists_widgets() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_DOCUMENT).contains("procedural-widget:"));
    }
}
//#endregion 🧪️Tests
