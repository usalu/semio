//! 🔺️ Sparse diff construction for the `create-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::mutation::CreatePrivacyRequirement;
use crate::artifacts::program::diff::ProgramPrivacyDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreatePrivacyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.privacy_requirement.header.id.clone();
    if base.privacy.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A privacy requirement already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { privacy: Some(ProgramPrivacyDelta { added: vec![payload.privacy_requirement.clone()], ..Default::default() }), ..Default::default() })
}
