//! ↩️ Inverse (undo) construction for the `delete-report-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📑reports` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteReportRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.reports.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateReportRecord(super::super::create_report_record::CreateReportRecord { report_record: base.reports[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
