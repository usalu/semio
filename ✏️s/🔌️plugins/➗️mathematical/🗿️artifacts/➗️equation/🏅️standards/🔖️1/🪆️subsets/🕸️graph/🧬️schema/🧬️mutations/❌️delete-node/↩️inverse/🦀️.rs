//! ↩️ `delete-node` — undo re-creates the node and re-`connect`s every edge it severed, both
//! captured from BASE state (pre-deletion); missing id ⇒ `Vec::new()`.

use crate::artifacts::equation::mutations::{connect_nodes, create_node};
use crate::artifacts::equation::{equation_graph, EquationMutation, EquationSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::DeleteNode, base: &EquationSnapshot) -> Vec<EquationMutation> {
    let graph = crate::artifacts::equation::equation_graph(base);
    let Some(node) = graph.nodes.iter().find(|node| node.id == payload.id) else {
        return Vec::new();
    };
    let mut steps = vec![EquationMutation::CreateNode(create_node::CreateNode { id: node.id.clone(), label: node.label.clone(), x: node.x, y: node.y })];
    steps.extend(
        graph
            .edges
            .iter()
            .filter(|edge| edge.source == payload.id || edge.target == payload.id)
            .map(|edge| EquationMutation::ConnectNodes(connect_nodes::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone() })),
    );
    steps
}
//#endregion 🔖️Inverse
