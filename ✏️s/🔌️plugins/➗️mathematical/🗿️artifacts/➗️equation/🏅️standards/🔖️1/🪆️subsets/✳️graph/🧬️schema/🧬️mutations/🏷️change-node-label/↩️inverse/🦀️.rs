//! ↩️ `change-node-label` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use crate::artifacts::equation::{equation_graph, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeNodeLabel, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let graph = crate::artifacts::equation::equation_graph(base);
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![EquationMutation::ChangeNodeLabel(super::ChangeNodeLabel { id: payload.id.clone(), new_label: node.label.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
