//! ↩️ `move-node` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use crate::artifacts::mathematical::{mathematical_graph, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::MoveNode, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let graph = crate::artifacts::mathematical::mathematical_graph(base);
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![MathematicalMutation::MoveNode(super::MoveNode { id: payload.id.clone(), x: node.x, y: node.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
