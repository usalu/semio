//! 🔺️ Sparse diff construction for the `rename-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::RenameSafetyRequirement;
use crate::artifacts::program::diff::{ProgramSafetyDelta, ProgramSafetyPatchEntry};
use crate::artifacts::program::registers::SafetyRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameSafetyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.safety.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No safety requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This safety requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = SafetyRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { safety: Some(ProgramSafetyDelta { patched: vec![ProgramSafetyPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
