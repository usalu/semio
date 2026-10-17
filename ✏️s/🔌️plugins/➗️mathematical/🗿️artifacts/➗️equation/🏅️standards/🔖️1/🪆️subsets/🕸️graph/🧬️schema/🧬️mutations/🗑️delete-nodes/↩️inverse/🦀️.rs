//! ↩️ `delete-nodes` — re-creates every deleted node then re-`connect`s every severed edge, both
//! captured from BASE state.

use crate::standards::v1::subsets::graph::schema::mutations::{connect_nodes, create_node};
use crate::EquationSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::DeleteNodes, base: &EquationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::graph::schema::mutations::connect_nodes::ConnectNodes> + From<crate::standards::v1::subsets::graph::schema::mutations::create_node::CreateNode>,
{
    Ok((|| {
    let graph = &base.graph;
    let mut steps: Vec<Op> = graph
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| payload.ids.contains(&node.id))
        .map(|(index, node)| Op::from(create_node::CreateNode { id: node.id.clone(), label: node.label.clone(), x: node.x, y: node.y, index: Some(index) }))
        .collect();
    steps.extend(
        graph
            .edges
            .iter()
            .enumerate()
            .filter(|(_, edge)| payload.ids.contains(&edge.source) || payload.ids.contains(&edge.target))
            .map(|(index, edge)| Op::from(connect_nodes::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), index: Some(index) })),
    );
    steps.reverse();
    steps

    })())
}
//#endregion 🔖️Inverse
