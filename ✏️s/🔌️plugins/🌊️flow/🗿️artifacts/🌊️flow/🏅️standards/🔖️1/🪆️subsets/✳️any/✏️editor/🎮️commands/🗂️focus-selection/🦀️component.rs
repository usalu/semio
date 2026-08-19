//! 🗂️ 🗂️ Flow play app commands command — `focus-selection`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::{flow_graph_selection_domains, focus_selection_camera, FLOW_INTERACTION_GRAPH};
use flow::FlowEvalSession;
use semio_framework_plugin::{app::InteractionView, ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct FocusSelection {}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg, session)` is framework-fixed at this exact 4-arg
/// shape (no `interaction` slot), so it still requires a `handle` of this signature to exist even though
/// it is reachable only through that macro-generated path (`FlowPlayApp::handle` always routes this
/// command through `apply` below instead) — degrades to a no-op, mirroring `delete_selection::handle`.
pub async fn handle(_payload: &FocusSelection, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::default())
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: reads the "graph" domain's live node
/// selection instead of the deleted `FlowConfig.selected_node_ids` — see `delete_selection::apply`'s
/// doc comment for why this is dispatched through `apply` directly rather than the macro-generated path.
pub async fn apply(_payload: &FocusSelection, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession, interaction: &InteractionView<'_>) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let (nodes, _edges) = flow_graph_selection_domains(&interaction.selection(FLOW_INTERACTION_GRAPH).ids);
    match focus_selection_camera(doc.snapshot, cfg.snapshot, session, &nodes) {
        Some(camera) => Ok(Emit::config(vec![FlowConfigMutation::SetCamera { camera }])),
        None => Ok(Emit::default()),
    }
}
