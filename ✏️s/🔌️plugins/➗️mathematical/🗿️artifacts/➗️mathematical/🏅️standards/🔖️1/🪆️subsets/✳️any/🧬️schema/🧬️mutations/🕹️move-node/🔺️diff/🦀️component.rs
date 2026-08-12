//! 🔺️ `move-node` — sparse diff construction.

use super::mutation::MoveNode;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MoveNode, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.x = payload.x;
        node.y = payload.y;
    }
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
