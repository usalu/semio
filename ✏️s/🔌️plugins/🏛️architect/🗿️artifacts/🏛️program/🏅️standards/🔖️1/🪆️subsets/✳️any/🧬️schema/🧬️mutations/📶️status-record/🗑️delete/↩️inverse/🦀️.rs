//! ↩️ Inverse (undo) construction for the `delete-status-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📶status-records` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteStatusRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.status_records.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateStatusRecord(super::super::create_status_record::CreateStatusRecord { status_record: base.status_records[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
