//! 🔺️ `replace-graph` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::ReplaceGraph, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if equation_graph(base) == payload.graph {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Graph is already identical to the requested replacement.");
    }
    let (notation, results, computed) = equation_children_from_state(&payload.graph, &equation_geometry(base));
    protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
