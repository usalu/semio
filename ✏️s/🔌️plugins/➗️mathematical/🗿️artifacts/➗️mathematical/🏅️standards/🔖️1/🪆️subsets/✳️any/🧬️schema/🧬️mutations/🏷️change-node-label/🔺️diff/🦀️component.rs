//! 🔺️ `change-node-label` — sparse diff construction.

use super::mutation::ChangeNodeLabel;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeNodeLabel, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = mathematical_graph(base);
    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.label = payload.new_label.clone();
    }
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
