//! 🔺️ `connect-nodes` — sparse diff construction.

use super::mutation::ConnectNodes;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalEdge, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate edge `id` is a no-op, matching `create-node`'s duplicate-id handling.
pub fn diff(payload: &ConnectNodes, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut graph = mathematical_graph(base);
    if !graph.edges.iter().any(|edge| edge.id == payload.id) {
        graph.edges.push(MathematicalEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone() });
    }
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() }
}
//#endregion 🔖️Diff
