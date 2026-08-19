//! ↩️ Inverse (undo) construction for the `replace-report-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📑reports` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceReportRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.reports.iter().find(|row| row.header.id == payload.report_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceReportRecord(super::mutation::ReplaceReportRecord { report_record: existing.clone() })],
        None => Vec::new(),
    }
}
