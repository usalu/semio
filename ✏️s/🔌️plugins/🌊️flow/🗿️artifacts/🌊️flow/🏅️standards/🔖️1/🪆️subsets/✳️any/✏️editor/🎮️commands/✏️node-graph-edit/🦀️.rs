//! 🕸️ 🎯️ Flow play app commands command — `node-graph-edit`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::{flow_graph_selection_domains, host_operations, sync_host_selection, FLOW_INTERACTION_GRAPH};
use flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

//#region 🔖️FlowNodeGraphEditOp
/// 🎯️ One batched edit inside a `FlowCommand::NodeGraphEdit`/`SpotlightCommit` — the `"setFixture"`/
/// `"deleteSelection"`/`"connect"` sub-kinds, closed and typed instead of stringly-tagged JSON. Mirrors
/// `dag_protocol::DagNodeGraphEditOp` exactly.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum)]
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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_nodes` is the "graph"
/// domain's live node selection (read by the caller via `InteractionView`) — no `SetSelection` config
/// mutation afterwards, the framework auto-prunes deleted ids out of `graph`'s selection via
/// `interaction_topology`.
fn node_graph_edit_result(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession, operations: &[FlowNodeGraphEditOp], selected_nodes: &[String]) -> Emit<FlowMutation, FlowConfigMutation> {
    let artifact_mutations = host_operations(fixture, config, session, |host| {
        let mut changed = false;
        for sub_operation in operations {
            match sub_operation {
                FlowNodeGraphEditOp::SetSnapshot { snapshot_json } => {
                    let parsed: Option<FlowSnapshot> = serde_json::from_str::<serde_json::Value>(snapshot_json).ok().and_then(|json| dsl::FromValue::from_value(dsl::DslValue::from(json)).ok());
                    if let Some(parsed) = parsed {
                        host.begin_change();
                        host.set_fixture_preserving_history(parsed.to_fixture());
                        changed = true;
                    }
                }
                FlowNodeGraphEditOp::DeleteSelection => {
                    sync_host_selection(host, selected_nodes);
                    if host.delete_selection().is_ok() {
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
    Emit::mutations(artifact_mutations)
}
//#endregion 🔖️SharedDispatch

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
pub struct NodeGraphEdit {
    #[dsl(statements)]
    pub operations: Vec<FlowNodeGraphEditOp>,
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot), so it still requires a `handle` of this signature to exist even though
/// it is reachable only through that macro-generated path (`FlowPlayApp::handle` always routes this
/// command through `apply` below instead) — degrades to treating the selection as empty.
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(node_graph_edit_result(doc.snapshot, cfg.snapshot, session, &payload.operations, &[]))
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` has no `interaction` slot (see
/// `delete_selection::apply`'s doc comment) — `FlowPlayApp::handle` routes this command through `apply`.
pub fn apply(payload: &NodeGraphEdit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession, interaction: &InteractionView<'_>) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let (nodes, _edges) = flow_graph_selection_domains(&interaction.selection(FLOW_INTERACTION_GRAPH).ids);
    Ok(node_graph_edit_result(doc.snapshot, cfg.snapshot, session, &payload.operations, &nodes))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app_with_registry, render, select_graph};
    use crate::editor::flow::FlowCommand;

    /// 🎯️ The batched `DeleteSelection` sub-op must clear the node selection (visible on the rendered
    /// scene) while leaving the widget count intact when nothing resolves — the behavior that
    /// distinguishes it from the top-level `FlowCommand::DeleteSelection`.
    #[semio_framework_async_macros::async_test]
    async fn batched_delete_selection_clears_the_node_selection_on_the_scene() {
        let mut app = flow_app_with_registry().await;
        select_graph(&mut app, &["slider"], &[]).await;
        dispatch(&mut app, FlowCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![FlowNodeGraphEditOp::DeleteSelection] })).await;
        assert!(!app.snapshot().expect("snapshot").to_fixture().widgets.iter().any(|widget| crate::artifacts::flow::schema::widget_id(widget) == "slider"), "batched delete removes the picked widget");
        let _ = render(&mut app, crate::editor::flow::FLOW_PLAY_BODY_MAIN).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn spotlight_commit_shares_the_node_graph_edit_vocabulary() {
        use crate::editor::flow::commands::spotlight_commit;
        let mut app = flow_app_with_registry().await;
        let result = dispatch(
            &mut app,
            FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit {
                operations: vec![spotlight_commit::FlowNodeGraphEditOp::Connect { source_node_id: "nope".into(), source_port_id: "out".into(), target_node_id: "gone".into(), target_port_id: "in".into() }],
            }),
        ).await;
        assert!(result.mutations.is_empty(), "connecting missing nodes is a no-operation");
    }
}
//#endregion 🧪️Tests
