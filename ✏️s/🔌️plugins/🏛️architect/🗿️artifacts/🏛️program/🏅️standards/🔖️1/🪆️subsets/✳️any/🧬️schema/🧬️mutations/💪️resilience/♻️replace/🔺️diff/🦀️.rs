//! 🔺️ Sparse diff construction for the `replace-resilience-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💪resilience` per Wave C.

use super::ReplaceResilienceRequirement;
use crate::diff::ProgramResilienceDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceResilienceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.resilience_requirement.header.id;
    let Some(position) = base.resilience.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No resilience requirement exists with this id.", [id.0.clone()]);
    };
    if base.resilience[position] == payload.resilience_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This resilience requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramResilienceDelta::removal(&base.resilience, position);
    delta.absorb(ProgramResilienceDelta::insertion(position, payload.resilience_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { resilience: Some(delta), ..Default::default() })
}
