//! 🔺️ `disconnect-nodes` — sparse diff construction.

use super::mutation::DisconnectNodes;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DisconnectNodes, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    graph.edges.retain(|edge| edge.id != payload.id);
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
