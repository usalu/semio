//! 🔺️ `change-graph-directed` — sparse diff construction.

use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ Clones the current graph and flips only the `directed` field, then re-derives all three
/// composed children from the patched `(graph, geometry)` pair — every graph-scoped mutation shares
/// this "clone + patch one field + re-derive" shape.
pub fn diff(payload: &super::ChangeGraphDirected, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = base.graph.clone();
    if graph.directed == payload.new_directed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Graph is already {}.", if payload.new_directed { "directed" } else { "undirected" }));
    }
    graph.directed = payload.new_directed;
    protocol::MutationOutcome::new(crate::equation_state_diff(graph, base.geometry.clone()))
}
//#endregion 🔖️Diff
