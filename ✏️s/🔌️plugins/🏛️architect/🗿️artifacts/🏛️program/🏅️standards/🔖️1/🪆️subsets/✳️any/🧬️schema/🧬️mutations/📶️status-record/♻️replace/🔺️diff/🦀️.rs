//! 🔺️ Sparse diff construction for the `replace-status-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📶status-records` per Wave C.

use super::ReplaceStatusRecord;
use crate::diff::ProgramStatusRecordsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceStatusRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.status_record.header.id;
    let Some(position) = base.status_records.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No status record exists with this id.", [id.0.clone()]);
    };
    if base.status_records[position] == payload.status_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This status record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramStatusRecordsDelta::removal(&base.status_records, position);
    delta.absorb(ProgramStatusRecordsDelta::insertion(position, payload.status_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { status_records: Some(delta), ..Default::default() })
}
