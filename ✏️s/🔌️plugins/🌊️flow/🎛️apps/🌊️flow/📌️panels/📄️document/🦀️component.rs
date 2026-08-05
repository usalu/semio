//! 📄️ Flow play app panel — the document tree: widgets and synapses of the current fixture.

use crate::apps::flow::flow_action;
use crate::apps::flow::terminology::FlowPlayLabels;
use crate::artifacts::flow::engine::{widget_id, widget_kind_label, widget_tree_label};
use crate::artifacts::flow::FlowFixture;
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const FLOW_PLAY_BODY_DOCUMENT: &str = "flow.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(FLOW_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &FlowFixture, selected: &[String], labels: &FlowPlayLabels) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            tree_item_with_action(format!("flow-play-document.widget.{}", widget_id(widget)), Label::data(widget_tree_label(widget)), Some(widget_kind_label(widget).into()), flow_action("setSelection", Some(json!({ "ids": [widget_id(widget)] }))))
        })
        .collect();
    let synapse_items: Vec<UiTreeItemNode> =
        fixture.synapses.iter().map(|synapse| tree_item_desc(format!("flow-play-document.synapse.{}", synapse.id), Label::data(format!("{} → {}", synapse.from, synapse.to)), Some(format!("{} → {}", synapse.from_port, synapse.to_port)))).collect();
    PanelTreeBuilder::new("flow-play-document")
        .section_or_placeholder("flow-play-document.widgets", Some(labels.widgets.into()), true, widget_items, labels.none_placeholder)
        .section_or_placeholder("flow-play-document.synapses", Some(labels.synapses.into()), false, synapse_items, labels.none_placeholder)
        .selected(selected.iter().map(|id| format!("flow-play-document.widget.{id}")).collect())
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{flow_app, render as render_body};

    #[test]
    fn document_lists_widgets() {
        let mut app = flow_app();
        assert!(render_body(&mut app, FLOW_PLAY_BODY_DOCUMENT).contains("flow-play-document.widgets"));
    }

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(FLOW_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
