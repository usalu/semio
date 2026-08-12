//! 🔺️ `change-node-label` — sparse diff construction.

use super::mutation::ChangeNodeLabel;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeNodeLabel, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.label = payload.new_label.clone();
    }
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
