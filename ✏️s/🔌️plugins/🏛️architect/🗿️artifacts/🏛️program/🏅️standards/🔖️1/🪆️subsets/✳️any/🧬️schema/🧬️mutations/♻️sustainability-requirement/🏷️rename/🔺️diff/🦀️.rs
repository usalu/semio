//! 🔺️ Sparse diff construction for the `rename-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::RenameSustainabilityRequirement;
use crate::artifacts::program::diff::{ProgramSustainabilityDelta, ProgramSustainabilityPatchEntry};
use crate::artifacts::program::registers::SustainabilityRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameSustainabilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.sustainability.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No sustainability requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This sustainability requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = SustainabilityRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { patched: vec![ProgramSustainabilityPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
