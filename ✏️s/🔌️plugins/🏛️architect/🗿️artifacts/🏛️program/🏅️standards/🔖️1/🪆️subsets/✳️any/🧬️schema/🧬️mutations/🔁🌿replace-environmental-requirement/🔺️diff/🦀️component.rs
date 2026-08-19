//! 🔺️ Sparse diff construction for the `replace-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::mutation::ReplaceEnvironmentalRequirement;
use crate::artifacts::program::diff::{ProgramEnvironmentalDelta, ProgramEnvironmentalPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceEnvironmentalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.environmental.iter().find(|row| row.header.id == payload.environmental_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No environmental requirement exists with this id.", [payload.environmental_requirement.header.id.0.clone()]);
    };
    if existing == &payload.environmental_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This environmental requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.environmental_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { patched: vec![ProgramEnvironmentalPatchEntry { id: payload.environmental_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
