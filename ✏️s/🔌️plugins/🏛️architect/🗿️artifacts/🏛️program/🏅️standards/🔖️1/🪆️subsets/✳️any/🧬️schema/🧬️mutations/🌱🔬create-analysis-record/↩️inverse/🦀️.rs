//! ↩️ Inverse (undo) construction for the `create-analysis-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔬analyses` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::CreateAnalysisRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAnalysisRecord(super::super::delete_analysis_record::DeleteAnalysisRecord { id: payload.analysis_record.header.id.clone() })]
}
