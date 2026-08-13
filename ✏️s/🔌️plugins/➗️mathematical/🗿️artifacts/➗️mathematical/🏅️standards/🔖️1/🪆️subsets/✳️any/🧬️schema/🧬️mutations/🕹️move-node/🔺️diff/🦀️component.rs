//! 🔺️ `move-node` — sparse diff construction.

use super::mutation::MoveNode;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MoveNode, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = mathematical_graph(base);
    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.x = payload.x;
        node.y = payload.y;
    }
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
