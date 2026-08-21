//! 📄️ Procedural2d play app panel — the document tree: widgets of the current fixture.

use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use crate::editor::procedural2d::config::Procedural2dConfig;
use crate::editor::procedural2d::terminology::Procedural2dLabels;
use semio_framework_plugin::{tree_item, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_BODY_DOCUMENT: &str = "procedural2d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCEDURAL2D_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ Item ids are the RAW widget id (no namespace prefix) — they must equal the `graph` interaction
/// domain's target ids one-for-one so `.interaction_domain("graph")`'s post-render presence stamping
/// (`ui_tree_stamp_presence`) can match them by plain string membership (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Clicks/selection are the framework's now — no
/// per-item action needed, and `_config` is unused (kept for call-site symmetry with `inspection`).
pub async fn render(document: &Procedural2dSnapshot, _config: &Procedural2dConfig, labels: &Procedural2dLabels) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = document.fixture.widgets.iter().map(|widget| tree_item(widget_id(widget).to_string(), Label::data(widget_id(widget).to_string()))).collect();
    PanelTreeBuilder::new("procedural2d-play-document").section_or_placeholder("procedural2d-play-document.widgets", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, widget_items, labels.none).interaction_domain("graph").build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, render as render_body};
    use semio_framework_plugin::PluginApp;

    #[semio_framework_async_macros::async_test]
    async fn document_lists_widgets() {
        let mut app = app();
        let rendered = render_body(&mut app, PROCEDURAL2D_PLAY_BODY_DOCUMENT);
        let fixture_widgets: Vec<String> = app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
        let first = fixture_widgets.first().expect("default fixture has at least one widget");
        assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCEDURAL2D_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
