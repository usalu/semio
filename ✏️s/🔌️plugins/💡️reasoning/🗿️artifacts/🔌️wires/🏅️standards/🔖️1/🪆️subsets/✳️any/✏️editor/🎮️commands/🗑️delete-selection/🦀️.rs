//! 🗑️ 🗑️ Wires play app commands command — `delete-selection`.

use crate::op::WiresMutation;
use crate::schema::fixture_edges;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ Deletes every currently-selected node/edge — shared by `handle`/`apply` below.
fn delete_selected(document: &WiresSnapshot, selected: &[String]) -> Emit<WiresMutation, NoConfigMutation> {
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
pub fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    Ok(delete_selected(doc.snapshot, &[]))
}

/// 🗑️ Removes the framework graph selection; topology validation prunes deleted identities.
pub fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    Ok(delete_selected(doc.snapshot, &interaction.selection("graph").ids))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
