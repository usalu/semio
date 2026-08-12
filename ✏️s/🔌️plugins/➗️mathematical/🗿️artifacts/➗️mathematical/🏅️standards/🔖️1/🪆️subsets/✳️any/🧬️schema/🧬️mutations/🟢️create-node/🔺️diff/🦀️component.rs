//! 🔺️ `create-node` — sparse diff construction.

use super::mutation::CreateNode;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalNode, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is a no-op (an id-keyed entity that already exists cannot be "created"
/// again) — the clone is returned unchanged rather than pushing a second node with the same id.
pub fn diff(payload: &CreateNode, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = base.graph.clone();
    if !graph.nodes.iter().any(|node| node.id == payload.id) {
        graph.nodes.push(MathematicalNode { id: payload.id.clone(), label: payload.label.clone(), x: payload.x, y: payload.y });
    }
    MathematicalDiff { graph: Some(graph), ..Default::default() }
}
//#endregion 🔖️Diff
