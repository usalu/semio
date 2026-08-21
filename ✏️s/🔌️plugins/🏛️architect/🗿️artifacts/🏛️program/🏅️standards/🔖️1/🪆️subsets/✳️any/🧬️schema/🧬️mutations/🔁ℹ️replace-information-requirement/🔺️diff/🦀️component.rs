//! 🔺️ Sparse diff construction for the `replace-information-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `ℹ️information` per Wave C.

use super::mutation::ReplaceInformationRequirement;
use crate::artifacts::program::diff::{ProgramInformationDelta, ProgramInformationPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceInformationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.information.iter().find(|row| row.header.id == payload.information_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No information requirement exists with this id.", [payload.information_requirement.header.id.0.clone()]);
    };
    if existing == &payload.information_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This information requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.information_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff {
        information: Some(ProgramInformationDelta { patched: vec![ProgramInformationPatchEntry { id: payload.information_requirement.header.id.0.clone(), patch }], ..Default::default() }),
        ..Default::default()
    })
}
