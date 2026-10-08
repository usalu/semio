//! 🔺️ Sparse diff construction for the `replace-stakeholder` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `👥stakeholders` per Wave C.

use super::ReplaceStakeholder;
use crate::diff::ProgramStakeholdersDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceStakeholder, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.stakeholder.header.id;
    let Some(position) = base.stakeholders.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No stakeholder exists with this id.", [id.0.clone()]);
    };
    if base.stakeholders[position] == payload.stakeholder {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This stakeholder already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramStakeholdersDelta::removal(&base.stakeholders, position);
    delta.absorb(ProgramStakeholdersDelta::insertion(position, payload.stakeholder.clone()));
    protocol::MutationOutcome::new(ProgramDiff { stakeholders: Some(delta), ..Default::default() })
}
