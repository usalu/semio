//! ↩️ `delete-node` — undo re-creates the node and re-`connect`s every edge it severed, both
//! captured from BASE state (pre-deletion); missing id ⇒ `Vec::new()`.

use crate::standards::v1::subsets::graph::schema::mutations::{connect_nodes, create_node};
use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::DeleteNode, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::connect_nodes::ConnectNodes> + From<crate::standards::v1::subsets::graph::schema::mutations::create_node::CreateNode>,
{
    Ok((|| {
    let graph = &base.graph;
    let Some(position) = graph.nodes.iter().position(|node| node.id == payload.id) else {
        return Vec::new();
    };
    let node = &graph.nodes[position];
    let mut steps = vec![Op::from(create_node::CreateNode { id: node.id.clone(), label: node.label.clone(), x: node.x, y: node.y, index: Some(position) })];
    steps.extend(
        graph
            .edges
            .iter()
            .enumerate()
            .filter(|(_, edge)| edge.source == payload.id || edge.target == payload.id)
            .map(|(index, edge)| Op::from(connect_nodes::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), index: Some(index) })),
    );
    steps.reverse();
    steps

    })())
}
//#endregion 🔖️Inverse
