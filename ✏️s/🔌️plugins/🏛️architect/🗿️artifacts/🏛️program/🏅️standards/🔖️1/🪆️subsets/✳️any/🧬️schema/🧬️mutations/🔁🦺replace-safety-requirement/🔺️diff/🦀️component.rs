//! 🔺️ Sparse diff construction for the `replace-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::mutation::ReplaceSafetyRequirement;
use crate::artifacts::program::diff::{ProgramSafetyDelta, ProgramSafetyPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceSafetyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.safety.iter().find(|row| row.header.id == payload.safety_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No safety requirement exists with this id.", [payload.safety_requirement.header.id.0.clone()]);
    };
    if existing == &payload.safety_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This safety requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.safety_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { safety: Some(ProgramSafetyDelta { patched: vec![ProgramSafetyPatchEntry { id: payload.safety_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
