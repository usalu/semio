//! 🔺️ Sparse diff builder for `DisconnectNodes` — delegates to the schema diff facet's own
//! `fixtures_after_remove_edge`.

use crate::artifacts::wires::diff::{diff_wires_and_board, fixtures_after_remove_edge, WiresDiff};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_edge;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::DisconnectNodes, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    if find_board_edge(base, &payload.edge_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.edge_id), [payload.edge_id.clone()]);
    }
    let (wires, board) = fixtures_after_remove_edge(base, &payload.edge_id);
    protocol::MutationOutcome::new(diff_wires_and_board(wires, board))
}
//#endregion 🔖️Diff
