//! ↩️ `move-node` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`.

use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::MoveNode, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::move_node::MoveNode>,
{
    Ok((|| {
    let graph = &base.graph;
    match graph.nodes.iter().find(|node| node.id == payload.id) {
        Some(node) => vec![Op::from(super::MoveNode { id: payload.id.clone(), x: node.x, y: node.y })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
