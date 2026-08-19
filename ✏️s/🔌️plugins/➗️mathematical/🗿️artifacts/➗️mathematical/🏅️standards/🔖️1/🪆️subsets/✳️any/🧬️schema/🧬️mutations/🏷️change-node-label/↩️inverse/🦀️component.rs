//! ↩️ `change-node-label` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use super::mutation::ChangeNodeLabel;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeNodeLabel, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let graph = crate::artifacts::mathematical::mathematical_graph(base);
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![MathematicalMutation::ChangeNodeLabel(ChangeNodeLabel { id: payload.id.clone(), new_label: node.label.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
