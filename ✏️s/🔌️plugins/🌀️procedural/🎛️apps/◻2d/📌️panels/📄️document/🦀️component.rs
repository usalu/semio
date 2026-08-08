//! 📄️ Procedural2d play app panel — the document tree: widgets of the current fixture.

use crate::apps::procedural2d::config::Procedural2dConfig;
use crate::apps::procedural2d::terminology::Procedural2dLabels;
use crate::apps::procedural2d::procedural2d_action;
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_BODY_DOCUMENT: &str = "procedural2d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(PROCEDURAL2D_PLAY_BODY_DOCUMENT.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Procedural2dSnapshot, config: &Procedural2dConfig, labels: &Procedural2dLabels) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = document
        .fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            semio_framework_plugin::tree_item_with_action(format!("procedural2d-play-document.widget.{id}"), Label::data(id.clone()), None, procedural2d_action("setSelection", Some(json!({ "ids": [id] }))))
        })
        .collect();
    PanelTreeBuilder::new("procedural2d-play-document")
        .section_or_placeholder("procedural2d-play-document.widgets", Some(Label::data(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL)), true, widget_items, labels.none)
        .selected(config.selected_ids.iter().map(|id| format!("procedural2d-play-document.widget.{id}")).collect())
        .selection_change(procedural2d_action("setSelection", None))
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, render as render_body};

    #[test]
    fn document_lists_widgets() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL2D_PLAY_BODY_DOCUMENT).contains("procedural2d-play-document.widget.rect"));
    }

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCEDURAL2D_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
