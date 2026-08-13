//! ↩️ `move-node` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use super::mutation::MoveNode;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &MoveNode, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let graph = crate::artifacts::mathematical::mathematical_graph(base);
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![MathematicalMutation::MoveNode(MoveNode { id: payload.id.clone(), x: node.x, y: node.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
