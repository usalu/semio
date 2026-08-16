//! 🔺️ `change-graph-directed` — sparse diff construction.

use super::mutation::ChangeGraphDirected;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ Clones the current graph and flips only the `directed` field, then re-derives all three
/// composed children from the patched `(graph, geometry)` pair — every graph-scoped mutation shares
/// this "clone + patch one field + re-derive" shape.
pub fn diff(payload: &ChangeGraphDirected, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut graph = mathematical_graph(base);
    if graph.directed == payload.new_directed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Graph is already {}.", if payload.new_directed { "directed" } else { "undirected" }));
    }
    graph.directed = payload.new_directed;
    let (notation, results, computed) = mathematical_children_from_state(&graph, &mathematical_geometry(base));
    protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
