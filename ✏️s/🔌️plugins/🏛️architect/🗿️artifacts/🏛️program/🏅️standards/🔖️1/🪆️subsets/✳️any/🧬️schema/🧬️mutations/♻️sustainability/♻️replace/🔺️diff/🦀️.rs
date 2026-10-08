//! 🔺️ Sparse diff construction for the `replace-sustainability-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `♻️sustainability` per Wave C.

use super::ReplaceSustainabilityRequirement;
use crate::diff::ProgramSustainabilityDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceSustainabilityRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.sustainability_requirement.header.id;
    let Some(position) = base.sustainability.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No sustainability requirement exists with this id.", [id.0.clone()]);
    };
    if base.sustainability[position] == payload.sustainability_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This sustainability requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramSustainabilityDelta::removal(&base.sustainability, position);
    delta.absorb(ProgramSustainabilityDelta::insertion(position, payload.sustainability_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { sustainability: Some(delta), ..Default::default() })
}
