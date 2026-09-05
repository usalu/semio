//! 🔺️ `change-graph-directed` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationSnapshot};

//#region 🔖️Diff
/// 🔺️ Clones the current graph and flips only the `directed` field, then re-derives all three
/// composed children from the patched `(graph, geometry)` pair — every graph-scoped mutation shares
/// this "clone + patch one field + re-derive" shape.
pub async fn diff(payload: &super::ChangeGraphDirected, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let mut graph = equation_graph(base);
    if graph.directed == payload.new_directed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Graph is already {}.", if payload.new_directed { "directed" } else { "undirected" }));
    }
    graph.directed = payload.new_directed;
    let (notation, results, computed) = equation_children_from_state(&graph, &equation_geometry(base));
    protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
