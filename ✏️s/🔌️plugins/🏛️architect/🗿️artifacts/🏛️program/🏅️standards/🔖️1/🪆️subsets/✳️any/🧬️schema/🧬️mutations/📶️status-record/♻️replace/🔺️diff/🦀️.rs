//! 🔺️ Sparse diff construction for the `replace-status-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📶status-records` per Wave C.

use super::ReplaceStatusRecord;
use crate::diff::ProgramStatusRecordsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceStatusRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.status_record.header.id;
    let Some(position) = base.status_records.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No status record exists with this id.", [id.0.clone()]);
    };
    if base.status_records[position] == payload.status_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This status record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.status_records.len()).then(|| base.status_records.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { removed: vec![id.0.clone()], added: vec![payload.status_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
