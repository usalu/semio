//! ↩️ Inverse for `DisconnectNodes` — reconstructs `ConnectNodes` from the edge (and its
//! relationship, if any) captured from BASE. Missing edge ⇒ `Vec::new()`.
use crate::artifacts::wires::engine::{find_board_edge, find_relationship};
use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DisconnectNodes, base: &WiresSnapshot) -> Vec<WiresMutation> {
    find_board_edge(base, &payload.edge_id)
        .map(|edge| {
            let relationship = find_relationship(base, &payload.edge_id).cloned().unwrap_or(DslValue::Null);
            crate::artifacts::wires::mutations::connect_nodes::mutation::connect_nodes(edge.clone(), relationship)
        })
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
