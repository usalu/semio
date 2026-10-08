//! ↩️ `change-node-label` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeNodeLabel, base: &EquationSnapshot) -> Result<Vec<EquationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let graph = &base.graph;
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![EquationMutation::ChangeNodeLabel(super::ChangeNodeLabel { id: payload.id.clone(), new_label: node.label.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
