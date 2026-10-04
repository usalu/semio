//! 🔺️ `replace-graph` — sparse diff construction.

use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceGraph, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if base.graph == payload.graph {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Graph is already identical to the requested replacement.");
    }
    protocol::MutationOutcome::new(crate::equation_state_diff(payload.graph.clone(), base.geometry.clone()))
}
//#endregion 🔖️Diff
