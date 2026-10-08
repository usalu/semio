//! 🔺️ Sparse diff construction for the `replace-privacy-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔒privacy` per Wave C.

use super::ReplacePrivacyRequirement;
use crate::diff::ProgramPrivacyDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplacePrivacyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.privacy_requirement.header.id;
    let Some(position) = base.privacy.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No privacy requirement exists with this id.", [id.0.clone()]);
    };
    if base.privacy[position] == payload.privacy_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This privacy requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.privacy.len()).then(|| base.privacy.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { privacy: Some(ProgramPrivacyDelta { removed: vec![id.0.clone()], added: vec![payload.privacy_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
