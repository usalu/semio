//! 🗑️ Wires play app command — `delete-selection`: graph `delete-edge` leaves for the selected edges, then `delete-node` leaves
//! for the selected nodes (each cascading its incident edges), as ONE edit of the composed board child.

use crate::schema::{board_edge, board_node};
use crate::{GraphEdgeId, GraphNodeId, SemioGraphMutation, WiresMutation, WiresSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::{delete_edge::DeleteEdge, delete_node::DeleteNode};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ The leaves deleting every selected board entity: selected edges first (an edge a deleted node would cascade away is
/// still deleted by name), then the selected nodes. A selection naming no live node or edge is refused by name rather than
/// answered with an empty success.
fn delete_selected(doc: &ArtifactView<'_, WiresSnapshot>, selected: &[String]) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let composed = crate::wires_composed_from_children(doc.snapshot, &doc.children)?;
    let edges = selected.iter().filter(|id| board_edge(&composed.board, id).is_some()).map(|id| SemioGraphMutation::DeleteEdge(DeleteEdge { id: GraphEdgeId::new(id.clone()) }));
    let nodes = selected.iter().filter(|id| board_node(&composed.board, id).is_some()).map(|id| SemioGraphMutation::DeleteNode(DeleteNode { id: GraphNodeId::new(id.clone()) }));
    let leaves: Vec<SemioGraphMutation> = edges.chain(nodes).collect();
    if leaves.is_empty() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("wires.selection.empty"), "deleteSelection needs at least one selected node or relationship on the board"));
    }
    Ok(crate::wires_child_emit(doc.snapshot, &leaves))
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), reachable
/// only through that macro-generated path (`ReasoningWiresPlayApp::handle` always routes this command
/// through `apply` below instead), so it sees an empty selection and refuses by name.
pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    delete_selected(doc, &[])
}

/// 🗑️ Removes the framework graph selection; topology validation prunes deleted identities.
pub fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    delete_selected(doc, &interaction.selection("graph").ids)
}

/// 🧵️ The retained-tool twin of [`apply`]: a bounded tool-job reducer is handed the raw
/// `protocol::InteractionState`, and `InteractionView`'s fields are framework-private, so the domain
/// selection is read straight off the state instead of being wrapped first.
pub fn apply_with_state(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, interaction: &protocol::InteractionState) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let selected = interaction.selection.get("graph").map(|domain| domain.ids.clone()).unwrap_or_default();
    delete_selected(doc, &selected)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
