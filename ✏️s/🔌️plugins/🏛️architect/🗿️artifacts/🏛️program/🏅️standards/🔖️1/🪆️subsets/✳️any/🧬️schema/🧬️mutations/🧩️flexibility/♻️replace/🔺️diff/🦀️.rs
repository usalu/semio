//! 🔺️ Sparse diff construction for the `replace-flexibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧩flexibility` per Wave C.

use super::ReplaceFlexibilityRequirement;
use crate::diff::ProgramFlexibilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceFlexibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.flexibility_requirement.header.id;
    let Some(position) = base.flexibility.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No flexibility requirement exists with this id.", [id.0.clone()]);
    };
    if base.flexibility[position] == payload.flexibility_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This flexibility requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramFlexibilityDelta::removal(&base.flexibility, position);
    delta.absorb(ProgramFlexibilityDelta::insertion(position, payload.flexibility_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { flexibility: Some(delta), ..Default::default() })
}
