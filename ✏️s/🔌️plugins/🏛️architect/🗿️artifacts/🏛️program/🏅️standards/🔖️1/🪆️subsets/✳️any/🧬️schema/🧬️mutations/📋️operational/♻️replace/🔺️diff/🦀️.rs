//! 🔺️ Sparse diff construction for the `replace-operational-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📋operations` per Wave C.

use super::ReplaceOperationalRequirement;
use crate::diff::ProgramOperationsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceOperationalRequirement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.operational_requirement.header.id;
    let Some(position) = base.operations.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No operational requirement exists with this id.", [id.0.clone()]);
    };
    if base.operations[position] == payload.operational_requirement {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This operational requirement already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.operations.len()).then(|| base.operations.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { operations: Some(ProgramOperationsDelta { removed: vec![id.0.clone()], added: vec![payload.operational_requirement.clone()], reordered, ..Default::default() }), ..Default::default() })
}
