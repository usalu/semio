//! ↩️ `connect-nodes` — undo is `disconnect-nodes`, unless `base` already had this edge id (then
//! `connect` was a no-op, matching `create-node`'s duplicate-id handling).

use crate::artifacts::mathematical::mutations::disconnect_nodes;
use crate::artifacts::mathematical::{mathematical_graph, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ConnectNodes, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    if crate::artifacts::mathematical::mathematical_graph(base).edges.iter().any(|edge| edge.id == payload.id) {
        return Vec::new();
    }
    vec![MathematicalMutation::DisconnectNodes(disconnect_nodes::DisconnectNodes { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
