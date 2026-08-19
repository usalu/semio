//! 🔺️ Sparse diff construction for the `rename-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::mutation::RenameEnvironmentalRequirement;
use crate::artifacts::program::diff::{ProgramEnvironmentalDelta, ProgramEnvironmentalPatchEntry};
use crate::artifacts::program::registers::EnvironmentalRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameEnvironmentalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.environmental.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No environmental requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This environmental requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = EnvironmentalRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { patched: vec![ProgramEnvironmentalPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
