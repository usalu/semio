//! 🔺️ `replace-graph` — sparse diff construction.

use super::mutation::ReplaceGraph;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ReplaceGraph, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    if mathematical_graph(base) == payload.graph {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Graph is already identical to the requested replacement.");
    }
    let (notation, results, computed) = mathematical_children_from_state(&payload.graph, &mathematical_geometry(base));
    protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
