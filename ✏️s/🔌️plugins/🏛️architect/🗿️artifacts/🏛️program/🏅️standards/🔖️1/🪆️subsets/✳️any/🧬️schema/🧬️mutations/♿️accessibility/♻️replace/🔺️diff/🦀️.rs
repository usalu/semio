//! 🔺️ Sparse diff construction for the `replace-accessibility-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♿accessibility` per Wave C.

use super::ReplaceAccessibilityRequirement;
use crate::diff::ProgramAccessibilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceAccessibilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.accessibility_requirement.header.id;
    let Some(position) = base.accessibility.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No accessibility requirement exists with this id.", [id.0.clone()]);
    };
    if base.accessibility[position] == payload.accessibility_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This accessibility requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramAccessibilityDelta::removal(&base.accessibility, position);
    delta.absorb(ProgramAccessibilityDelta::insertion(position, payload.accessibility_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { accessibility: Some(delta), ..Default::default() })
}
