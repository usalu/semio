//! ↩️ `connect-nodes` — undo is `disconnect-nodes`, unless `base` already had this edge id (then
//! `connect` was a no-op, matching `create-node`'s duplicate-id handling).

use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::disconnect_nodes;
use crate::artifacts::equation::{equation_graph, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ConnectNodes, base: &EquationSnapshot) -> Vec<EquationMutation> {
    if crate::artifacts::equation::equation_graph(base).edges.iter().any(|edge| edge.id == payload.id) {
        return Vec::new();
    }
    vec![EquationMutation::DisconnectNodes(disconnect_nodes::DisconnectNodes { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
