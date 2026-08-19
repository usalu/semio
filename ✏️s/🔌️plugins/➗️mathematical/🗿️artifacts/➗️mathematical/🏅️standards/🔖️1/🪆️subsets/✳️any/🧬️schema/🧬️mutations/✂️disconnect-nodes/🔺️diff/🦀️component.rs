//! 🔺️ `disconnect-nodes` — sparse diff construction.

use super::mutation::DisconnectNodes;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &DisconnectNodes, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut graph = mathematical_graph(base);
    if !graph.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    graph.edges.retain(|edge| edge.id != payload.id);
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
