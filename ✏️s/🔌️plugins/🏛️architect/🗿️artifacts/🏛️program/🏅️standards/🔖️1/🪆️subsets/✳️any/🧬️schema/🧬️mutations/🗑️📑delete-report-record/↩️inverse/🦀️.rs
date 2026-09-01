//! ↩️ Inverse (undo) construction for the `delete-report-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📑reports` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteReportRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.reports.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateReportRecord(super::super::create_report_record::CreateReportRecord { report_record: existing.clone() })],
        None => Vec::new(),
    }
}
