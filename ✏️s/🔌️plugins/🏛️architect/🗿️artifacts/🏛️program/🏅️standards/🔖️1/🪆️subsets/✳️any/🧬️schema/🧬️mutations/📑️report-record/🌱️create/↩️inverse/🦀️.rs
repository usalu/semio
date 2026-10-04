//! ↩️ Inverse (undo) construction for the `create-report-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📑reports` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateReportRecord, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteReportRecord(super::super::delete_report_record::DeleteReportRecord { id: payload.report_record.header.id.clone() })]

    })())
}
