//! 🔺️ Sparse diff construction for the `replace-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📌requirements` per Wave C.

use super::ReplaceRequirement;
use crate::diff::ProgramRequirementsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.requirement.header.id;
    let Some(position) = base.requirements.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No requirement exists with this id.", [id.0.clone()]);
    };
    if base.requirements[position] == payload.requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramRequirementsDelta::removal(&base.requirements, position);
    delta.absorb(ProgramRequirementsDelta::insertion(position, payload.requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { requirements: Some(delta), ..Default::default() })
}
