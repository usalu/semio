//! 🔺️ `change-graph-directed` — sparse diff construction: the `directed` flag.
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGraphDirected, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if base.graph.directed == payload.new_directed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Graph is already {}.", if payload.new_directed { "directed" } else { "undirected" }));
    }
    protocol::MutationOutcome::new(crate::equation_state_diff(EquationDiff { directed: Some(payload.new_directed), ..Default::default() }, base))
}
//#endregion 🔖️Diff
