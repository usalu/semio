//! 🔺️ `delete-node` — sparse diff construction, cascading to incident edges.

use super::mutation::DeleteNode;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    graph.nodes.retain(|node| node.id != payload.id);
    graph.edges.retain(|edge| edge.source != payload.id && edge.target != payload.id);
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
