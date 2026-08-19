//! 🔺️ Sparse diff construction for the `rename-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::mutation::RenameRequirement;
use crate::artifacts::program::diff::{ProgramRequirementsDelta, ProgramRequirementsPatchEntry};
use crate::artifacts::program::registers::RequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.requirements.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = RequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { requirements: Some(ProgramRequirementsDelta { patched: vec![ProgramRequirementsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
