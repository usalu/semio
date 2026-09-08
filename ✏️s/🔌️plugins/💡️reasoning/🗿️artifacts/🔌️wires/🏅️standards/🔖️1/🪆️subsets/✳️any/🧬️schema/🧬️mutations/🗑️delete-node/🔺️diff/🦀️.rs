//! 🔺️ Sparse diff builder for `DeleteNode` — delegates to the schema diff facet's own
//! `board_after_remove_node` (a targeted `retain`, never apply-then-capture).

use crate::diff::{board_after_remove_node, diff_board_fixture, WiresDiff};
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteNode, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    if find_board_node(base, &payload.node_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id), [payload.node_id.clone()]);
    }
    protocol::MutationOutcome::new(diff_board_fixture(&board_after_remove_node(base, &payload.node_id)))
}
//#endregion 🔖️Diff
