//! ↩️ `delete-nodes` — re-creates every deleted node then re-`connect`s every severed edge, both
//! captured from BASE state.

use crate::artifacts::mathematical::mutations::{connect_nodes, create_node};
use crate::artifacts::mathematical::{mathematical_graph, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &super::DeleteNodes, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    let graph = crate::artifacts::mathematical::mathematical_graph(base);
    let mut steps: Vec<MathematicalMutation> =
        graph.nodes.iter().filter(|node| payload.ids.contains(&node.id)).map(|node| MathematicalMutation::CreateNode(create_node::CreateNode { id: node.id.clone(), label: node.label.clone(), x: node.x, y: node.y })).collect();
    steps.extend(
        graph
            .edges
            .iter()
            .filter(|edge| payload.ids.contains(&edge.source) || payload.ids.contains(&edge.target))
            .map(|edge| MathematicalMutation::ConnectNodes(connect_nodes::ConnectNodes { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone() })),
    );
    steps
}
//#endregion 🔖️Inverse
