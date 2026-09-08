//! 🔺️ Sparse diff construction for the `create-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::CreateSustainabilityRequirement;
use crate::diff::ProgramSustainabilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateSustainabilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.sustainability_requirement.header.id.clone();
    if base.sustainability.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A sustainability requirement already exists with this id.", [id.0]);
    }
    protocol::MutationOutcome::new(ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { added: vec![payload.sustainability_requirement.clone()], ..Default::default() }), ..Default::default() })
}
