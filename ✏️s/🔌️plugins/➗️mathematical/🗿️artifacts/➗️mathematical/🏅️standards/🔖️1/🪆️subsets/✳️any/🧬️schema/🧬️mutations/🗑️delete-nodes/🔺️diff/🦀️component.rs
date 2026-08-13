//! 🔺️ `delete-nodes` — sparse diff construction, cascading to every incident edge.

use super::mutation::DeleteNodes;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteNodes, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = mathematical_graph(base);
    graph.nodes.retain(|node| !payload.ids.contains(&node.id));
    graph.edges.retain(|edge| !payload.ids.contains(&edge.source) && !payload.ids.contains(&edge.target));
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
