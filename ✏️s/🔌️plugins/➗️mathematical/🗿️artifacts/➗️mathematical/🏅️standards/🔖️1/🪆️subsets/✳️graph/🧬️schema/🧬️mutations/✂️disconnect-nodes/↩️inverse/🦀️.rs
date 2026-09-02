//! ↩️ `disconnect-nodes` — undo re-`connect`s the exact edge captured from BASE state; missing
//! edge ⇒ `Vec::new()`.

use crate::artifacts::mathematical::standards::v1::subsets::graph::schema::mutations::connect_nodes;
use crate::artifacts::mathematical::{mathematical_graph, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::DisconnectNodes, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let graph = crate::artifacts::mathematical::mathematical_graph(base);
    match graph.edges.iter().find(|edge| edge.id == payload.id) {
        Some(edge) => vec![MathematicalMutation::ConnectNodes(connect_nodes::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
