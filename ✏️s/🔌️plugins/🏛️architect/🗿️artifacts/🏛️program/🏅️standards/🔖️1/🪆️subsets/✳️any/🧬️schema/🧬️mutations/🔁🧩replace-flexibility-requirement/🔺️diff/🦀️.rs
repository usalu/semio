//! 🔺️ Sparse diff construction for the `replace-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::ReplaceFlexibilityRequirement;
use crate::artifacts::program::diff::{ProgramFlexibilityDelta, ProgramFlexibilityPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceFlexibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.flexibility.iter().find(|row| row.header.id == payload.flexibility_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No flexibility requirement exists with this id.", [payload.flexibility_requirement.header.id.0.clone()]);
    };
    if existing == &payload.flexibility_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This flexibility requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.flexibility_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff {
        flexibility: Some(ProgramFlexibilityDelta { patched: vec![ProgramFlexibilityPatchEntry { id: payload.flexibility_requirement.header.id.0.clone(), patch }], ..Default::default() }),
        ..Default::default()
    })
}
