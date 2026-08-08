//! 🎯️ Flow play app commands — batched node-graph edits (`nodeGraphEdit` / `spotlightCommit`).
//!
//! Both commands carry the exact same sub-op vocabulary (they shared one `handle_action` match arm before
//! the typed-command conversion), so they share [`node_graph_edit_result`]. The `DeleteSelection` sub-op
//! only clears `selected_node_ids` on success (leaving `selected_edge_ids`/`selected_handle_ids`
//! untouched) — distinct from the top-level `FlowCommand::DeleteSelection`, which clears all three.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::engine::{host_operations, sync_host_selection};
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️FlowNodeGraphEditOp
/// 🎯️ One batched edit inside a `FlowCommand::NodeGraphEdit`/`SpotlightCommit` — the `"setFixture"`/
/// `"deleteSelection"`/`"connect"` sub-kinds, closed and typed instead of stringly-tagged JSON. Mirrors
/// `dag_protocol::DagNodeGraphEditOp` exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
pub enum FlowNodeGraphEditOp {
    #[dsl(key = "set-fixture")]
    SetSnapshot { snapshot_json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
}
//#endregion 🔖️FlowNodeGraphEditOp

//#region 🔖️SharedDispatch
fn node_graph_edit_result(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession, operations: &[FlowNodeGraphEditOp]) -> Emit<FlowMutation, FlowConfigMutation> {
    let selected = config.selected_node_ids.clone();
    let mut clear_selection = false;
    let document_mutations = host_operations(fixture, config, session, |host| {
        let mut changed = false;
        for sub_operation in operations {
            match sub_operation {
                FlowNodeGraphEditOp::SetSnapshot { snapshot_json } => {
                    if let Ok(parsed) = serde_json::from_str::<FlowSnapshot>(snapshot_json) {
                        host.begin_change();
                        host.set_fixture_preserving_history(parsed.to_fixture());
                        changed = true;
                    }
                }
                FlowNodeGraphEditOp::DeleteSelection => {
                    sync_host_selection(host, &selected);
                    if host.delete_selection().is_ok() {
                        clear_selection = true;
                        changed = true;
                    }
                }
                FlowNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                    if host.connect_ports(source_node_id, source_port_id, target_node_id, target_port_id).is_ok() {
                        changed = true;
                    }
                }
            }
        }
        changed
    });
    let config_mutations = if clear_selection { vec![FlowConfigMutation::SetSelection { node_ids: Vec::new(), edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }] } else { Vec::new() };
    Emit { document_mutations, config_mutations, ..Default::default() }
}
//#endregion 🔖️SharedDispatch

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct NodeGraphEdit {
        #[dsl(statements)]
        pub operations: Vec<FlowNodeGraphEditOp>,
    }

    pub fn handle(payload: &NodeGraphEdit, doc: &DocumentView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(node_graph_edit_result(doc.snapshot, cfg.snapshot, session, &payload.operations))
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🔖️SpotlightCommit
pub mod spotlight_commit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct SpotlightCommit {
        #[dsl(statements)]
        pub operations: Vec<FlowNodeGraphEditOp>,
    }

    pub fn handle(payload: &SpotlightCommit, doc: &DocumentView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(node_graph_edit_result(doc.snapshot, cfg.snapshot, session, &payload.operations))
    }
}
//#endregion 🔖️SpotlightCommit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::commands::selection::set_selection::SetSelection;
    use crate::apps::flow::testkit::{dispatch, flow_app, render};
    use crate::apps::flow::FlowCommand;

    /// 🎯️ The batched `DeleteSelection` sub-op must clear the node selection (visible on the rendered
    /// scene) while leaving the widget count intact when nothing resolves — the behavior that
    /// distinguishes it from the top-level `FlowCommand::DeleteSelection`.
    #[test]
    fn batched_delete_selection_clears_the_node_selection_on_the_scene() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetSelection(SetSelection { ids: vec!["slider".into()], edge_ids: Vec::new(), handle_ids: Vec::new() }));
        assert!(render(&mut app, crate::apps::flow::FLOW_PLAY_BODY_MAIN).contains(r#""selection":["slider"]"#), "selection lands on the scene first");
        dispatch(&mut app, FlowCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations: vec![FlowNodeGraphEditOp::DeleteSelection] }));
        assert!(!render(&mut app, crate::apps::flow::FLOW_PLAY_BODY_MAIN).contains(r#""selection":["slider"]"#), "batched delete clears the node selection");
    }

    #[test]
    fn spotlight_commit_shares_the_node_graph_edit_vocabulary() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit {
                operations: vec![FlowNodeGraphEditOp::Connect { source_node_id: "nope".into(), source_port_id: "out".into(), target_node_id: "gone".into(), target_port_id: "in".into() }] }));
        assert!(result.mutations.is_empty(), "connecting missing nodes is a no-operation");
    }
}
//#endregion 🧪️Tests
