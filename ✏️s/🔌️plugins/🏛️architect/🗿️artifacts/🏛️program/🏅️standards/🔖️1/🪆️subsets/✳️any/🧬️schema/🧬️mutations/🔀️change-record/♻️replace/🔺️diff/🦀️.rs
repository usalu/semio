//! 🔺️ Sparse diff construction for the `replace-change-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📝changes` per Wave C.

use super::ReplaceChangeRecord;
use crate::diff::ProgramChangesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceChangeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.change_record.header.id;
    let Some(position) = base.changes.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No change record exists with this id.", [id.0.clone()]);
    };
    if base.changes[position] == payload.change_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This change record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.changes.len()).then(|| base.changes.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { changes: Some(ProgramChangesDelta { removed: vec![id.0.clone()], added: vec![payload.change_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
