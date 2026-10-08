//! 🔺️ Sparse diff construction for the `create-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::CreatePrivacyRequirement;
use crate::diff::ProgramPrivacyDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreatePrivacyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.privacy_requirement.header.id;
    if base.privacy.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A privacy requirement already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.privacy.len());
    if at > base.privacy.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the privacy requirement list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { privacy: Some(ProgramPrivacyDelta::insertion(at, payload.privacy_requirement.clone())), ..Default::default() })
}
