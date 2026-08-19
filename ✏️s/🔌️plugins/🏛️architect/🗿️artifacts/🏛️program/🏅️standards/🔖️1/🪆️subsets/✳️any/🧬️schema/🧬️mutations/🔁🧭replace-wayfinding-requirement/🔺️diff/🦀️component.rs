//! 🔺️ Sparse diff construction for the `replace-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::mutation::ReplaceWayfindingRequirement;
use crate::artifacts::program::diff::{ProgramWayfindingDelta, ProgramWayfindingPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceWayfindingRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.wayfinding.iter().find(|row| row.header.id == payload.wayfinding_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No wayfinding requirement exists with this id.", [payload.wayfinding_requirement.header.id.0.clone()]);
    };
    if existing == &payload.wayfinding_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This wayfinding requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.wayfinding_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { patched: vec![ProgramWayfindingPatchEntry { id: payload.wayfinding_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
