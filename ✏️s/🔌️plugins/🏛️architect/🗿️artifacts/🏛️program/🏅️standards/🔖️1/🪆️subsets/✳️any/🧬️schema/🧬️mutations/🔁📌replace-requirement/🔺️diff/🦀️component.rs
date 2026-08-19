//! 🔺️ Sparse diff construction for the `replace-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::mutation::ReplaceRequirement;
use crate::artifacts::program::diff::{ProgramRequirementsDelta, ProgramRequirementsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.requirements.iter().find(|row| row.header.id == payload.requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No requirement exists with this id.", [payload.requirement.header.id.0.clone()]);
    };
    if existing == &payload.requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { requirements: Some(ProgramRequirementsDelta { patched: vec![ProgramRequirementsPatchEntry { id: payload.requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
