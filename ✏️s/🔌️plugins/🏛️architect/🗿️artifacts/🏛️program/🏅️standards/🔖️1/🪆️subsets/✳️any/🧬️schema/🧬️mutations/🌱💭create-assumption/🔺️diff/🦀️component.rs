//! 🔺️ Sparse diff construction for the `create-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::mutation::CreateAssumption;
use crate::artifacts::program::diff::ProgramAssumptionsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateAssumption, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.assumption.header.id.clone();
    if base.assumptions.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An assumption already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { added: vec![payload.assumption.clone()], ..Default::default() }), ..Default::default() })
}
