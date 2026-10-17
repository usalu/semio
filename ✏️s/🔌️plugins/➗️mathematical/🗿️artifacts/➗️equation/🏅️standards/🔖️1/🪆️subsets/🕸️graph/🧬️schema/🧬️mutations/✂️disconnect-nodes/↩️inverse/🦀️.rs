//! ↩️ `disconnect-nodes` — undo re-`connect`s the exact edge captured from BASE state; missing
//! edge ⇒ `Vec::new()`.

use crate::standards::v1::subsets::graph::schema::mutations::connect_nodes;
use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::DisconnectNodes, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::connect_nodes::ConnectNodes>,
{
    Ok((|| {
    let graph = &base.graph;
    match graph.edges.iter().position(|edge| edge.id == payload.id) {
        Some(index) => {
            let edge = &graph.edges[index];
            vec![Op::from(connect_nodes::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), index: Some(index) })]
        }
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
