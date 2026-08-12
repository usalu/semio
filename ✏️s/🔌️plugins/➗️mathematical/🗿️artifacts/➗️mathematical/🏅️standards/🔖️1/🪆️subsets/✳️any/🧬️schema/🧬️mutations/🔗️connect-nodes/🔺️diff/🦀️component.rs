//! 🔺️ `connect-nodes` — sparse diff construction.

use super::mutation::ConnectNodes;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalEdge, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate edge `id` is a no-op, matching `create-node`'s duplicate-id handling.
pub fn diff(payload: &ConnectNodes, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    if !graph.edges.iter().any(|edge| edge.id == payload.id) {
        graph.edges.push(MathematicalEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone() });
    }
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
