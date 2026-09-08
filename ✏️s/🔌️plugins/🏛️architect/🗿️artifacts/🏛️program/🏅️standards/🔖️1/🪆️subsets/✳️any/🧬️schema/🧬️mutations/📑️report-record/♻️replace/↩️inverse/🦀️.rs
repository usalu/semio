//! ↩️ Inverse (undo) construction for the `replace-report-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📑reports` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceReportRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.reports.iter().find(|row| row.header.id == payload.report_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceReportRecord(super::ReplaceReportRecord { report_record: existing.clone() })],
        None => Vec::new(),
    }
}
