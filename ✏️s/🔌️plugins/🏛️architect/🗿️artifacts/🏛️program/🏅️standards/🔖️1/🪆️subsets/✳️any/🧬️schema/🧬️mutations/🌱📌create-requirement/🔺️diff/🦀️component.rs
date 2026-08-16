//! 🔺️ Sparse diff construction for the `create-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::mutation::CreateRequirement;
use crate::artifacts::program::diff::ProgramRequirementsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.requirement.header.id.clone();
    if base.requirements.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { requirements: Some(ProgramRequirementsDelta { added: vec![payload.requirement.clone()], ..Default::default() }), ..Default::default() })
}
