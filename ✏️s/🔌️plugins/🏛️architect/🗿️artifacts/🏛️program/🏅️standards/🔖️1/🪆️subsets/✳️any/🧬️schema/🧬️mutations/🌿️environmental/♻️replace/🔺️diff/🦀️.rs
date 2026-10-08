//! 🔺️ Sparse diff construction for the `replace-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::ReplaceEnvironmentalRequirement;
use crate::diff::ProgramEnvironmentalDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceEnvironmentalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.environmental_requirement.header.id;
    let Some(position) = base.environmental.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No environmental requirement exists with this id.", [id.0.clone()]);
    };
    if base.environmental[position] == payload.environmental_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This environmental requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramEnvironmentalDelta::removal(&base.environmental, position);
    delta.absorb(ProgramEnvironmentalDelta::insertion(position, payload.environmental_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { environmental: Some(delta), ..Default::default() })
}
