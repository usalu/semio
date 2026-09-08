//! ↩️ Inverse for `DisconnectNodes` — reconstructs `ConnectNodes` from the edge (and its
//! relationship, if any) captured from BASE. Missing edge ⇒ `Vec::new()`.

use crate::mutations::WiresMutation;
use crate::standards::v1::subsets::any::schema::inferences::{find_board_edge, find_relationship};
use crate::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DisconnectNodes, base: &WiresSnapshot) -> Vec<WiresMutation> {
    find_board_edge(base, &payload.edge_id)
        .map(|edge| {
            let relationship = find_relationship(base, &payload.edge_id).cloned().unwrap_or(DslValue::Null);
            crate::mutations::connect_nodes::connect_nodes(edge, relationship)
        })
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
