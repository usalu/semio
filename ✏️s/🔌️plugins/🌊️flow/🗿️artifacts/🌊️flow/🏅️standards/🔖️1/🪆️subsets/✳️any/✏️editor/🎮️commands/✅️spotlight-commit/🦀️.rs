//! 🕸️ 🎯️ Flow play app commands command — `spotlight-commit`.

use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::flow::{flow_graph_selection_domains, FLOW_INTERACTION_GRAPH};
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

pub use crate::editor::flow::commands::node_graph_edit::FlowNodeGraphEditOp;

//#region 🔖️SharedDispatch
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_nodes` is the "graph"
/// domain's live node selection (read by the caller via `InteractionView`) — no `SetSelection` config
/// mutation afterwards, the framework auto-prunes deleted ids out of `graph`'s selection via
/// `interaction_topology`.
pub fn node_graph_edit_result(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, operations: &[FlowNodeGraphEditOp], selected_nodes: &[String]) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    crate::editor::flow::commands::node_graph_edit::node_graph_edit_result(snapshot, config, session, operations, selected_nodes)
}
//#endregion 🔖️SharedDispatch

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
pub struct SpotlightCommit {
    #[dsl(statements)]
    pub operations: Vec<FlowNodeGraphEditOp>,
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot), so it still requires a `handle` of this signature to exist even though
/// it is reachable only through that macro-generated path (`FlowPlayApp::handle` always routes this
/// command through `apply` below instead) — degrades to treating the selection as empty.
pub fn handle(payload: &SpotlightCommit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    node_graph_edit_result(doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, &payload.operations, &[])
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` has no `interaction` slot (see
/// `delete_selection::apply`'s doc comment) — `FlowPlayApp::handle` routes this command through `apply`.
pub fn apply(payload: &SpotlightCommit, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut FlowEvalSession, interaction: &InteractionView<'_>) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let (nodes, _edges) = flow_graph_selection_domains(&interaction.selection(FLOW_INTERACTION_GRAPH).ids);
    node_graph_edit_result(doc.snapshot, &crate::editor::flow::modes::edit::windows::main::config::current(cfg), session, &payload.operations, &nodes)
}
