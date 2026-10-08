//! 🔺️ `replace-graph` — sparse diff construction: the scalars and node and edge rows that differ from the replacement graph.
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceGraph, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if base.graph == payload.graph {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Graph is already identical to the requested replacement.");
    }
    protocol::MutationOutcome::new(crate::equation_state_diff(crate::diff::graph_replacing(&base.graph, &payload.graph), base))
}
//#endregion 🔖️Diff
