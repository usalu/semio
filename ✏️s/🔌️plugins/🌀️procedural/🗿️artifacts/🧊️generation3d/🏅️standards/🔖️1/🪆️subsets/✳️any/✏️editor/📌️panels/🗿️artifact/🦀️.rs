//! 📄️ Generation3d play app panel — the document tree: nodes, ports and wires of the open graph.

use crate::editor::generation3d::config::Generation3dConfig;
use crate::editor::generation3d::modes::edit::windows::flow::graph_outline;
use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::standards::v1::subsets::any::schema::{dag_host_snapshot_to_workflow, with_host};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{flow_backed_node_graph_extras, FlowEvalSession};
use semio_framework_plugin::plugin_app_close_prelude::UiAssemblyResult;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_BODY_ARTIFACT: &str = "procedural.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(GENERATION_3D_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn document_error(scope: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.document", scope)
}

/// 🕸️ The open flow graph as semantic rows on the Artifact panel — the same outline the Flow window
/// used to embed beside its canvas. The window body is the node-graph surface only; this tree is where
/// keyboard and screen-reader users traverse nodes, ports and wires (ticket 26/09/14).
pub fn render(document: &Generation3dSnapshot, config: &Generation3dConfig, session: &FlowEvalSession, labels: &Generation3dLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let host_snapshot = &document.host_snapshot;
    let (nodes, edges) = with_host(host_snapshot, |host| dag_host_snapshot_to_workflow(&host.dag.host_snapshot));
    let flow_extras = flow_backed_node_graph_extras(host_snapshot, &config.lod_mode, 0.0, true, false, semio_framework_ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(session));
    graph_outline(&nodes, &edges, flow_extras.status_json.as_ref(), labels).map_err(|_| document_error("outline"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
