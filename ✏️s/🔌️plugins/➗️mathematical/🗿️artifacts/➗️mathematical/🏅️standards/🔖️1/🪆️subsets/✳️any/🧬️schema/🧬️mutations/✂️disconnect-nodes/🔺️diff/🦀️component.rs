//! 🔺️ `disconnect-nodes` — sparse diff construction.

use super::mutation::DisconnectNodes;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DisconnectNodes, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = mathematical_graph(base);
    graph.edges.retain(|edge| edge.id != payload.id);
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
