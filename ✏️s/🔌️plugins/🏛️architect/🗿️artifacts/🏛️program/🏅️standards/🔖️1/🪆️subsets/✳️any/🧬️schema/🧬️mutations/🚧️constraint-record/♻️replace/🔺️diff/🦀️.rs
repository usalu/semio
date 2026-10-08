//! 🔺️ Sparse diff construction for the `replace-constraint-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🚧constraints` per Wave C.

use super::ReplaceConstraintRecord;
use crate::diff::ProgramConstraintsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceConstraintRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.constraint_record.header.id;
    let Some(position) = base.constraints.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No constraint record exists with this id.", [id.0.clone()]);
    };
    if base.constraints[position] == payload.constraint_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This constraint record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramConstraintsDelta::removal(&base.constraints, position);
    delta.absorb(ProgramConstraintsDelta::insertion(position, payload.constraint_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { constraints: Some(delta), ..Default::default() })
}
