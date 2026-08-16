//! 🔺️ Sparse diff construction for the `delete-cost-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💰costs` per Wave C.

use super::mutation::DeleteCostRequirement;
use crate::artifacts::program::diff::ProgramCostsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteCostRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.costs.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No cost requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { costs: Some(ProgramCostsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
