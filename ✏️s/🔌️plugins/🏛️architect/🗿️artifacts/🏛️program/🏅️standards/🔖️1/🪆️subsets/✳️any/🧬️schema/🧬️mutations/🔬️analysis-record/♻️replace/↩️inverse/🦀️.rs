//! ↩️ Inverse (undo) construction for the `replace-analysis-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔬analyses` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceAnalysisRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.analyses.iter().find(|row| row.header.id == payload.analysis_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAnalysisRecord(super::ReplaceAnalysisRecord { analysis_record: existing.clone() })],
        None => Vec::new(),
    }
}
