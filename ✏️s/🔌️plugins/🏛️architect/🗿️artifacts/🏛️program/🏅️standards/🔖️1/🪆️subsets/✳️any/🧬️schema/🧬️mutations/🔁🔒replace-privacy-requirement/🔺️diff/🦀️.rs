//! 🔺️ Sparse diff construction for the `replace-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::ReplacePrivacyRequirement;
use crate::artifacts::program::diff::{ProgramPrivacyDelta, ProgramPrivacyPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplacePrivacyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.privacy.iter().find(|row| row.header.id == payload.privacy_requirement.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No privacy requirement exists with this id.", [payload.privacy_requirement.header.id.0.clone()]);
    };
    if existing == &payload.privacy_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This privacy requirement already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.privacy_requirement).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { privacy: Some(ProgramPrivacyDelta { patched: vec![ProgramPrivacyPatchEntry { id: payload.privacy_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
