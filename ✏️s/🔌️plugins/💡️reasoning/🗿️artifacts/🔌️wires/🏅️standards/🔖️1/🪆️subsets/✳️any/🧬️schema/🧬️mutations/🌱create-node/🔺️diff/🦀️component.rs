//! 🔺️ Sparse diff builder for `CreateNode` — delegates to the schema diff facet's own
//! `board_after_add_node` (a real targeted board-fixture rebuild, never apply-then-capture).
use crate::artifacts::wires::diff::{board_after_add_node, diff_board_fixture, WiresDiff};
use crate::artifacts::wires::schema::entity_id;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::CreateNode, base: &WiresSnapshot) -> protocol::MutationOutcome<WiresDiff> {
    if let Some(id) = entity_id(&payload.node, "id") {
        if find_board_node(base, id).is_some() {
            return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", id), [id.to_string()]);
        }
    }
    protocol::MutationOutcome::new(diff_board_fixture(board_after_add_node(base, &payload.node)))
}
//#endregion 🔖️Diff
