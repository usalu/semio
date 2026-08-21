//! 📄️ Procedural3d play app panel — the document tree: widgets of the current fixture.

use crate::artifacts::procedural3d::widget_id;
use crate::editor::procedural3d::terminology::Procedural3dLabels;
use flow::FlowFixture;
use semio_framework_plugin::{tree_item, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_BODY_DOCUMENT: &str = "procedural.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCEDURAL_3D_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🌳️ `tree_item` plus an icon id — this app's document tree carries icons per item.
async fn tree_item_with_icon(id: impl Into<String>, label: impl Into<Label>, icon_id: Option<&str>) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: icon_id.map(Into::into), menu: None, ..tree_item(id, label) }
}

/// 🕹️ Item ids are the RAW widget id (no namespace prefix) — they must equal the `graph` interaction
/// domain's target ids one-for-one so `.interaction_domain("graph")`'s post-render presence stamping
/// (`ui_tree_stamp_presence`) can match them by plain string membership (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Clicks/selection are the framework's now — no
/// per-item action needed.
pub async fn render(fixture: &FlowFixture, labels: &Procedural3dLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = fixture.widgets.iter().map(|widget| tree_item_with_icon(widget_id(widget).to_string(), Label::data(widget_id(widget).to_string()), Some("cpu"))).collect();
    PanelTreeBuilder::new("procedural-play-document").section("procedural-play-document.widgets", Some(labels.widgets.into()), true, items).interaction_domain("graph").build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, render as render_body};
    use semio_framework_plugin::PluginApp;

    #[semio_framework_async_macros::async_test]
    async fn document_lists_widgets() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        let rendered = render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_DOCUMENT);
        let fixture_widgets: Vec<String> = app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
        let first = fixture_widgets.first().expect("default fixture has at least one widget");
        assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
    }
}
//#endregion 🧪️Tests
