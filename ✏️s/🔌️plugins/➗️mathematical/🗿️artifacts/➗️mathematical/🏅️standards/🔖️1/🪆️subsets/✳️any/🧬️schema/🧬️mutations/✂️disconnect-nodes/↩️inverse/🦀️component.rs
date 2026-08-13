//! ↩️ `disconnect-nodes` — undo re-`connect`s the exact edge captured from BASE state; missing
//! edge ⇒ `Vec::new()`.

use crate::artifacts::mathematical::mutations::connect_nodes;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

use super::mutation::DisconnectNodes;

//#region 🔖️Inverse
pub fn inverse(payload: &DisconnectNodes, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let graph = crate::artifacts::mathematical::mathematical_graph(base);
    match graph.edges.iter().find(|edge| edge.id == payload.id) {
        Some(edge) => vec![MathematicalMutation::ConnectNodes(connect_nodes::mutation::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
