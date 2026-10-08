//! 🔺️ Sparse diff construction for the `replace-environmental-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🌿environmental` per Wave C.

use super::ReplaceEnvironmentalRequirement;
use crate::diff::ProgramEnvironmentalDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceEnvironmentalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.environmental_requirement.header.id;
    let Some(position) = base.environmental.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No environmental requirement exists with this id.", [id.0.clone()]);
    };
    if base.environmental[position] == payload.environmental_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This environmental requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.environmental.len()).then(|| base.environmental.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { environmental: Some(ProgramEnvironmentalDelta { removed: vec![id.0.clone()], added: vec![payload.environmental_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
