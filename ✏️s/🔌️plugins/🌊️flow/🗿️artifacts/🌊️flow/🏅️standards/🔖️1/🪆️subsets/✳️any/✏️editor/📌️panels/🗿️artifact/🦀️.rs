//! 📄️ Flow play app panel — the document tree: widgets and synapses of the current snapshot.

use crate::editor::flow::terminology::FlowPlayLabels;
use crate::editor::flow::{flow_graph_edge_target_id, flow_graph_node_target_id, pick_item, FLOW_INTERACTION_GRAPH, FLOW_PLAY_APP_ID};
use crate::schema::{widget_id, widget_kind_label, widget_tree_label};
use crate::FlowSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

/// 🏷️ Converts document-tree titles into the panel builder's `Label`.
fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<Label> {
    Label::try_from(value.as_ref().to_string()).map_err(|error| PluginAssemblyError::new("ui.document", error))
}

//#region 🔖️Constants
pub const FLOW_PLAY_BODY_ARTIFACT: &str = "flow.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(FLOW_PLAY_BODY_ARTIFACT.into()),
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
///
/// 🪟️ Both sections are windowed: the host's `TreeWindows` decide which slice of an unbounded widget
/// or synapse list this render materialises, and every section stamps its full `total`.
pub fn render(snapshot: &FlowSnapshot, labels: &FlowPlayLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let live = snapshot.to_host_snapshot();
    PanelTreeBuilder::new("flow-play-document")?
        .window_section_or_placeholder(
            windows,
            "flow-play-document.widgets",
            Some(ui_label(labels.widgets.as_str())?),
            true,
            &live.widgets,
            |widget| pick_item(flow_graph_node_target_id(widget_id(widget)), widget_tree_label(widget), Some(widget_kind_label(widget).into()), "node"),
            labels.none_placeholder.as_str(),
        )?
        .window_section_or_placeholder(
            windows,
            "flow-play-document.synapses",
            Some(ui_label(labels.synapses.as_str())?),
            false,
            &live.synapses,
            |synapse| pick_item(flow_graph_edge_target_id(&synapse.id), format!("{} → {}", synapse.from, synapse.to), Some(format!("{} → {}", synapse.from_port, synapse.to_port)), "edge"),
            labels.none_placeholder.as_str(),
        )?
        .interaction_domain(FLOW_PLAY_APP_ID, FLOW_INTERACTION_GRAPH)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
