//! 🔺️ `delete-node` — sparse diff construction, cascading to incident edges.

use super::mutation::DeleteNode;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteNode, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = mathematical_graph(base);
    graph.nodes.retain(|node| node.id != payload.id);
    graph.edges.retain(|edge| edge.source != payload.id && edge.target != payload.id);
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
