//! 📄️ Flow play app panel — the document tree: widgets and synapses of the current fixture.

use crate::artifacts::flow::schema::{widget_id, widget_kind_label, widget_tree_label};
use crate::artifacts::flow::FlowSnapshot;
use crate::editor::flow::terminology::FlowPlayLabels;
use crate::editor::flow::{flow_graph_edge_target_id, flow_graph_node_target_id, FLOW_INTERACTION_GRAPH};
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

/// 🏷️ Converts document-tree titles into the panel builder's `Label`.
fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<Label> {
    Label::try_from(value.as_ref().to_string()).map_err(|error| PluginAssemblyError::new("ui.document", error))
}

//#region 🔖️Constants
pub const FLOW_PLAY_BODY_DOCUMENT: &str = "flow.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(FLOW_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `flow_graph_node_target_id`/`flow_graph_edge_target_id` targets `FlowPlayApp::interaction_topology`
/// declares for the "graph" domain — the framework stamps this tree's selection/hover presence from
/// that domain (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item
/// click action is declared here anymore (clicks are translated into `interactionSelect` generically)?.
pub fn render(fixture: &FlowSnapshot, labels: &FlowPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let live = fixture.to_fixture();
    let widget_items = crate::editor::flow::ui_node_list(live.widgets.iter().map(|widget| tree_item_desc(flow_graph_node_target_id(widget_id(widget)), widget_tree_label(widget), Some(widget_kind_label(widget).into()))))?;
    let synapse_items = crate::editor::flow::ui_node_list(
        live.synapses.iter().map(|synapse| tree_item_desc(flow_graph_edge_target_id(&synapse.id), format!("{} → {}", synapse.from, synapse.to), Some(format!("{} → {}", synapse.from_port, synapse.to_port)))),
    )?;
    PanelTreeBuilder::new("flow-play-document")?
        .section_or_placeholder("flow-play-document.widgets", Some(ui_label(labels.widgets.as_str())?), true, widget_items, labels.none_placeholder.as_str())?
        .section_or_placeholder("flow-play-document.synapses", Some(ui_label(labels.synapses.as_str())?), false, synapse_items, labels.none_placeholder.as_str())?
        .interaction_domain(FLOW_INTERACTION_GRAPH)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn document_lists_widgets() {
        let mut app = flow_app();
        assert!(render_body(&mut app, FLOW_PLAY_BODY_DOCUMENT).contains("flow-play-document.widgets"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(FLOW_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
