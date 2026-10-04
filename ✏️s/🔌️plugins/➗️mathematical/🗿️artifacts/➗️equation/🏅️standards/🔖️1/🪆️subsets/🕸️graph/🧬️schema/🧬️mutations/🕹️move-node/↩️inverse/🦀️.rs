//! ↩️ `move-node` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveNode, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let graph = base.graph.clone();
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![EquationMutation::MoveNode(super::MoveNode { id: payload.id.clone(), x: node.x, y: node.y })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
