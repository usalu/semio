//! 🔺️ Sparse diff construction for the `delete-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::DeleteAssumption;
use crate::artifacts::program::diff::ProgramAssumptionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteAssumption, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.assumptions.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No assumption exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
