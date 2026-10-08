//! 🔺️ `disconnect-nodes` — sparse diff construction: one removed edge id.
use crate::diff::EquationEdgesDelta;
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectNodes, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let Some(index) = base.graph.edges.iter().position(|edge| edge.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let diff = EquationDiff { edges: Some(EquationEdgesDelta::removal(&base.graph.edges, index)), ..Default::default() };
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
