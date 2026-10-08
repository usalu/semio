//! 🔺️ Sparse diff construction for the `replace-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::ReplaceAssumption;
use crate::diff::ProgramAssumptionsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceAssumption, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.assumption.header.id;
    let Some(position) = base.assumptions.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No assumption exists with this id.", [id.0.clone()]);
    };
    if base.assumptions[position] == payload.assumption {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This assumption already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramAssumptionsDelta::removal(&base.assumptions, position);
    delta.absorb(ProgramAssumptionsDelta::insertion(position, payload.assumption.clone()));
    protocol::MutationOutcome::new(ProgramDiff { assumptions: Some(delta), ..Default::default() })
}
