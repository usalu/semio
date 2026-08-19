//! 🔺️ Sparse diff construction for the `create-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::mutation::CreateSustainabilityRequirement;
use crate::artifacts::program::diff::ProgramSustainabilityDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateSustainabilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.sustainability_requirement.header.id.clone();
    if base.sustainability.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A sustainability requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { added: vec![payload.sustainability_requirement.clone()], ..Default::default() }), ..Default::default() })
}
