//! 🔺️ Sparse diff construction for the `create-status-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📶status-records` per Wave C.

use super::CreateStatusRecord;
use crate::diff::ProgramStatusRecordsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateStatusRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.status_record.header.id;
    if base.status_records.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A status record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.status_records.len());
    if at > base.status_records.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the status record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { status_records: Some(ProgramStatusRecordsDelta::insertion(at, payload.status_record.clone())), ..Default::default() })
}
