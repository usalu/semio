//! ↩️ `change-node-label` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ChangeNodeLabel, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::change_node_label::ChangeNodeLabel>,
{
    Ok((|| {
    let graph = &base.graph;
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![Op::from(super::ChangeNodeLabel { id: payload.id.clone(), new_label: node.label.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
