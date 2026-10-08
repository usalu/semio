//! 🔺️ Sparse diff construction for the `replace-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::ReplaceFlexibilityRequirement;
use crate::diff::ProgramFlexibilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceFlexibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.flexibility_requirement.header.id;
    let Some(position) = base.flexibility.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No flexibility requirement exists with this id.", [id.0.clone()]);
    };
    if base.flexibility[position] == payload.flexibility_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This flexibility requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.flexibility.len()).then(|| base.flexibility.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { flexibility: Some(ProgramFlexibilityDelta { removed: vec![id.0.clone()], added: vec![payload.flexibility_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
