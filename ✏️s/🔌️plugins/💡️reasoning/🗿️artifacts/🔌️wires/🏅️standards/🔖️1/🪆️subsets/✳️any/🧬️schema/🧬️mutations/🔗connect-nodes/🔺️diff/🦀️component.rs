//! 🔺️ Sparse diff builder for `ConnectNodes` — delegates to the schema diff facet's own
//! `fixtures_after_add_edge` (a real targeted board+wires-fixture rebuild).
use crate::artifacts::wires::diff::{diff_wires_and_board, fixtures_after_add_edge, WiresDiff};
use crate::artifacts::wires::schema::entity_id;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::{find_board_edge, find_board_node};
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectNodes, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    if let Some(id) = entity_id(&payload.edge, "id") {
        if find_board_edge(base, id).is_some() {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", id), [id.to_string()]);
        }
    }
    for key in ["source", "target"] {
        if let Some(endpoint) = payload.edge.get(key).and_then(|value| value.as_str()) {
            if find_board_node(base, endpoint).is_none() {
                return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", endpoint), [endpoint.to_string()]);
            }
        }
    }
    let (wires, board) = fixtures_after_add_edge(base, &payload.edge, &payload.relationship);
    protocol::MutationOutcome::new(diff_wires_and_board(wires, board))
}
//#endregion 🔖️Diff
