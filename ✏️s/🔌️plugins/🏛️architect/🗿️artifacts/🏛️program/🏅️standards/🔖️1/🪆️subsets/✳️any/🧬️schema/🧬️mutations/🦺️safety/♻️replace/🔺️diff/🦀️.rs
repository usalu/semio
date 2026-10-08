//! 🔺️ Sparse diff construction for the `replace-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::ReplaceSafetyRequirement;
use crate::diff::ProgramSafetyDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceSafetyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.safety_requirement.header.id;
    let Some(position) = base.safety.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No safety requirement exists with this id.", [id.0.clone()]);
    };
    if base.safety[position] == payload.safety_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This safety requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.safety.len()).then(|| base.safety.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { safety: Some(ProgramSafetyDelta { removed: vec![id.0.clone()], added: vec![payload.safety_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
