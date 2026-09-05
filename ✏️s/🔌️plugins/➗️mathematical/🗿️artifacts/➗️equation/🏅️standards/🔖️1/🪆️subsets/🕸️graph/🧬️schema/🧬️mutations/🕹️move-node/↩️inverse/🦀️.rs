//! ↩️ `move-node` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use crate::artifacts::equation::{equation_graph, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::MoveNode, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let graph = crate::artifacts::equation::equation_graph(base);
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![EquationMutation::MoveNode(super::MoveNode { id: payload.id.clone(), x: node.x, y: node.y })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
