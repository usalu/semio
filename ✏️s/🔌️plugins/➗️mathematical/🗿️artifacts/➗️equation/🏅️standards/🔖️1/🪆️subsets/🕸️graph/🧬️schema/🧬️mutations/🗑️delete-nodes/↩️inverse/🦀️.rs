//! ↩️ `delete-nodes` — re-creates every deleted node then re-`connect`s every severed edge, both
//! captured from BASE state.

use crate::standards::v1::subsets::graph::schema::mutations::{connect_nodes, create_node};
use crate::{EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteNodes, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let graph = crate::equation_graph(base);
    let mut steps: Vec<EquationMutation> = graph
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| payload.ids.contains(&node.id))
        .map(|(index, node)| EquationMutation::CreateNode(create_node::CreateNode { id: node.id.clone(), label: node.label.clone(), x: node.x, y: node.y, index: Some(index) }))
        .collect();
    steps.extend(
        graph
            .edges
            .iter()
            .enumerate()
            .filter(|(_, edge)| payload.ids.contains(&edge.source) || payload.ids.contains(&edge.target))
            .map(|(index, edge)| EquationMutation::ConnectNodes(connect_nodes::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), index: Some(index) })),
    );
    steps.reverse();
    steps
}
//#endregion 🔖️Inverse
