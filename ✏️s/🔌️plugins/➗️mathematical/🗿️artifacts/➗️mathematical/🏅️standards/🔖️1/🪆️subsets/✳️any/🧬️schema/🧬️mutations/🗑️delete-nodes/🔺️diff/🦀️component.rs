//! 🔺️ `delete-nodes` — sparse diff construction, cascading to every incident edge.

use super::mutation::DeleteNodes;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteNodes, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    graph.nodes.retain(|node| !payload.ids.contains(&node.id));
    graph.edges.retain(|edge| !payload.ids.contains(&edge.source) && !payload.ids.contains(&edge.target));
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
