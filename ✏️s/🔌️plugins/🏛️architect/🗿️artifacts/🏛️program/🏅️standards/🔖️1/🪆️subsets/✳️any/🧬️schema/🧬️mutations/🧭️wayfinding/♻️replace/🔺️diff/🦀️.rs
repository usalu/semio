//! 🔺️ Sparse diff construction for the `replace-wayfinding-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧭wayfinding` per Wave C.

use super::ReplaceWayfindingRequirement;
use crate::diff::ProgramWayfindingDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceWayfindingRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.wayfinding_requirement.header.id;
    let Some(position) = base.wayfinding.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No wayfinding requirement exists with this id.", [id.0.clone()]);
    };
    if base.wayfinding[position] == payload.wayfinding_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This wayfinding requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.wayfinding.len()).then(|| base.wayfinding.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { wayfinding: Some(ProgramWayfindingDelta { removed: vec![id.0.clone()], added: vec![payload.wayfinding_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
