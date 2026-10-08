//! 🔺️ Sparse diff construction for the `replace-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::ReplaceOperationalRequirement;
use crate::diff::ProgramOperationsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceOperationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.operational_requirement.header.id;
    let Some(position) = base.operations.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No operational requirement exists with this id.", [id.0.clone()]);
    };
    if base.operations[position] == payload.operational_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This operational requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramOperationsDelta::removal(&base.operations, position);
    delta.absorb(ProgramOperationsDelta::insertion(position, payload.operational_requirement.clone()));
    protocol::MutationOutcome::new(ProgramDiff { operations: Some(delta), ..Default::default() })
}
