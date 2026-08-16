//! 🔺️ Sparse diff construction for the `delete-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::mutation::DeletePrivacyRequirement;
use crate::artifacts::program::diff::ProgramPrivacyDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeletePrivacyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.privacy.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No privacy requirement exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { privacy: Some(ProgramPrivacyDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
