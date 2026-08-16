//! 🔺️ Sparse diff construction for the `delete-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::mutation::DeleteRequirement;
use crate::artifacts::program::diff::ProgramRequirementsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeleteRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.requirements.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { requirements: Some(ProgramRequirementsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
