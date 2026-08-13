//! 🕸️ 🎯️ Flow play app commands command — `spotlight-commit`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::{host_operations, sync_host_selection};
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
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
    let artifact_mutations = host_operations(fixture, config, session, |host| {
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
    Emit { artifact_mutations, config_mutations, ..Default::default() }
}
//#endregion 🔖️SharedDispatch

//#region 🔖️NodeGraphEdit
//#endregion 🔖️NodeGraphEdit

//#region 🔖️SpotlightCommit
//#endregion 🔖️SpotlightCommit

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SpotlightCommit {
    #[dsl(statements)]
    pub operations: Vec<FlowNodeGraphEditOp>,
}

pub fn handle(payload: &SpotlightCommit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(node_graph_edit_result(doc.snapshot, cfg.snapshot, session, &payload.operations))
}
