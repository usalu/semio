//! 🔺️ Sparse diff construction for the `replace-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::mutation::ReplaceSustainabilityRequirement;
use crate::artifacts::program::diff::{ProgramSustainabilityDelta, ProgramSustainabilityPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceSustainabilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.sustainability.iter().find(|row| row.header.id == payload.sustainability_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No sustainability requirement exists with this id.", [payload.sustainability_requirement.header.id.0.clone()]);
    };
    if existing == &payload.sustainability_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This sustainability requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.sustainability_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { sustainability: Some(ProgramSustainabilityDelta { patched: vec![ProgramSustainabilityPatchEntry { id: payload.sustainability_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
