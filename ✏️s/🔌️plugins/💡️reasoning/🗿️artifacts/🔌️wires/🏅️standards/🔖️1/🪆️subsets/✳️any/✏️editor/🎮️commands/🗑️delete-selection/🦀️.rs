//! 🗑️ 🗑️ Wires play app commands command — `delete-selection`.

use crate::op::WiresMutation;
use crate::schema::fixture_edges;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ Deletes every currently-selected node/edge — shared by `handle`/`apply` below.
fn delete_selected(document: &WiresSnapshot, selected: &[String]) -> Emit<WiresMutation, WiresConfigMutation> {
    let board = crate::wires_working_board(document);
    let mut operations = Vec::new();
    for id in selected {
        if find_board_node(document, id).is_some() {
            operations.push(crate::mutations::delete_node(id.clone()));
        } else if fixture_edges(&board).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
            operations.push(crate::mutations::disconnect_nodes(id.clone()));
        }
    }
    Emit { artifact_mutations: operations, ..Default::default() }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), reachable
/// only through that macro-generated path (`ReasoningWiresPlayApp::handle` always routes this command
/// through `apply` below instead), so it degrades to treating the selection as empty.
pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(delete_selected(doc.snapshot, &[]))
}

/// 🕹️ Reads the "graph" domain's live selection instead of the deleted `config.selected_ids` — no
/// `SetSelection` config mutation needed afterwards, the framework auto-prunes the deleted ids out of
/// "graph"'s selection via `interaction_topology`/`validate_state`.
pub fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(delete_selected(doc.snapshot, &interaction.selection("graph").ids))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
