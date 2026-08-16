//! 🔺️ Sparse diff construction for the `create-status-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📶status-records` per Wave C.

use super::mutation::CreateStatusRecord;
use crate::artifacts::program::diff::ProgramStatusRecordsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateStatusRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.status_record.header.id.clone();
    if base.status_records.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A status record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { added: vec![payload.status_record.clone()], ..Default::default() }), ..Default::default() })
}
