//! 🗑️ 🗑️ Wires play app commands command — `delete-selection`.

use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::schema::fixture_edges;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ Deletes every currently-selected node/edge — shared by `handle`/`apply` below.
async fn delete_selected(document: &WiresSnapshot, selected: &[String]) -> Emit<WiresMutation, WiresConfigMutation> {
    let board = crate::artifacts::wires::wires_working_board(document);
    let mut operations = Vec::new();
    for id in selected {
        if find_board_node(document, id).is_some() {
            operations.push(crate::artifacts::wires::mutations::delete_node(id.clone()));
        } else if fixture_edges(&board).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
            operations.push(crate::artifacts::wires::mutations::disconnect_nodes(id.clone()));
        }
    }
    Emit { artifact_mutations: operations, ..Default::default() }
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape
/// (no `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), reachable
/// only through that macro-generated path (`ReasoningWiresPlayApp::handle` always routes this command
/// through `apply` below instead), so it degrades to treating the selection as empty.
pub async fn handle(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(delete_selected(doc.snapshot, &[]))
}

/// 🕹️ Reads the "graph" domain's live selection instead of the deleted `config.selected_ids` — no
/// `SetSelection` config mutation needed afterwards, the framework auto-prunes the deleted ids out of
/// "graph"'s selection via `interaction_topology`/`validate_state`.
pub async fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>, interaction: &InteractionView<'_>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(delete_selected(doc.snapshot, &interaction.selection("graph").ids))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::schema::fixture_nodes;
    use crate::editor::wires::commands::add_node;
    use crate::editor::wires::testkit::{app_with_registry, dispatch, new_app};
    use crate::editor::wires::WiresCommand;
    use semio_framework_plugin::{testkit::meta, InteractionTarget, PluginApp, INTERACTION_SELECT_ACTION_ID};
    use serde_json::json;

    /// 🕹️ `handle`'s macro-only path treats the selection as empty (no `InteractionView` reachable) —
    /// nothing gets deleted.
    #[semio_framework_async_macros::async_test]
    async fn handle_alone_deletes_nothing_without_a_live_selection() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }));
        dispatch(&mut app, WiresCommand::DeleteSelection(DeleteSelection {}));
        assert_eq!(fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).len(), 1);
    }

    /// 🕹️ End-to-end proof the "graph" domain's live selection actually drives `deleteSelection` —
    /// spawns a node, selects it via the framework's real `interactionSelect` action (the only way a
    /// downstream crate can populate a genuine `InteractionView`), then confirms `deleteSelection`
    /// removes exactly that node.
    #[semio_framework_async_macros::async_test]
    async fn delete_selection_removes_the_live_selected_node() {
        let mut app = app_with_registry();
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }));
        let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "node".into(), id: "node-1".into() }]).expect("targets");
        app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&json!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" })), &meta("local")).expect("interactionSelect");
        dispatch(&mut app, WiresCommand::DeleteSelection(DeleteSelection {}));
        assert!(fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).is_empty());
    }
}
//#endregion 🧪️Tests
