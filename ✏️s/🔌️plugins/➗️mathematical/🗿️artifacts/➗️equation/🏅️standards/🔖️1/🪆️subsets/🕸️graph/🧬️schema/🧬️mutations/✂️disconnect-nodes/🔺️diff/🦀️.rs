//! 🔺️ `disconnect-nodes` — sparse diff construction: one removed edge id.
use crate::diff::EquationEdgesDelta;
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectNodes, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if !base.graph.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let diff = EquationDiff { edges: Some(EquationEdgesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base))
}
//#endregion 🔖️Diff
