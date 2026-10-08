//! 🔺️ Sparse diff construction for the `replace-safety-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🦺safety` per Wave C.

use super::ReplaceSafetyRequirement;
use crate::diff::ProgramSafetyDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceSafetyRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.safety_requirement.header.id;
    let Some(position) = base.safety.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No safety requirement exists with this id.", [id.0.clone()]);
    };
    if base.safety[position] == payload.safety_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This safety requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramSafetyDelta::removal(&base.safety, position);
    delta.absorb(ProgramSafetyDelta::insertion(position, payload.safety_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { safety: Some(delta), ..Default::default() })
}
