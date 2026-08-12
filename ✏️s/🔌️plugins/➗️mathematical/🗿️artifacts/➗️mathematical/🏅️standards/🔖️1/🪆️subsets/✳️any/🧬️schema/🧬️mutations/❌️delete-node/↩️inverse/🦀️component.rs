//! ↩️ `delete-node` — undo re-creates the node and re-`connect`s every edge it severed, both
//! captured from BASE state (pre-deletion); missing id ⇒ `Vec::new()`.

use crate::artifacts::mathematical::mutations::{connect_nodes, create_node};
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

use super::mutation::DeleteNode;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteNode, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let Some(node) = base.graph.nodes.iter().find(|node| node.id == payload.id) else {
        return Vec::new();
    };
    let mut steps = vec![MathematicalMutation::CreateNode(create_node::mutation::CreateNode {
        id: node.id.clone(),
        label: node.label.clone(),
        x: node.x,
        y: node.y,
    })];
    steps.extend(
        base.graph
            .edges
            .iter()
            .filter(|edge| edge.source == payload.id || edge.target == payload.id)
            .map(|edge| MathematicalMutation::ConnectNodes(connect_nodes::mutation::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone() })),
    );
    steps
}
//#endregion 🔖️Inverse
