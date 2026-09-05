//! ↩️ `disconnect-nodes` — undo re-`connect`s the exact edge captured from BASE state; missing
//! edge ⇒ `Vec::new()`.

use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::connect_nodes;
use crate::artifacts::equation::{equation_graph, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::DisconnectNodes, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let graph = crate::artifacts::equation::equation_graph(base);
    match graph.edges.iter().find(|edge| edge.id == payload.id) {
        Some(edge) => vec![EquationMutation::ConnectNodes(connect_nodes::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
