//! 🔺️ Sparse diff construction for the `replace-information-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `ℹ️information` per Wave C.

use super::ReplaceInformationRequirement;
use crate::diff::ProgramInformationDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceInformationRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.information_requirement.header.id;
    let Some(position) = base.information.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No information requirement exists with this id.", [id.0.clone()]);
    };
    if base.information[position] == payload.information_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This information requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramInformationDelta::removal(&base.information, position);
    delta.absorb(ProgramInformationDelta::insertion(position, payload.information_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { information: Some(delta), ..Default::default() })
}
