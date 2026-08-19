//! 🔺️ Sparse diff construction for the `rename-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::mutation::RenamePrivacyRequirement;
use crate::artifacts::program::diff::{ProgramPrivacyDelta, ProgramPrivacyPatchEntry};
use crate::artifacts::program::registers::PrivacyRequirementPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenamePrivacyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.privacy.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No privacy requirement exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This privacy requirement already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = PrivacyRequirementPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { privacy: Some(ProgramPrivacyDelta { patched: vec![ProgramPrivacyPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
