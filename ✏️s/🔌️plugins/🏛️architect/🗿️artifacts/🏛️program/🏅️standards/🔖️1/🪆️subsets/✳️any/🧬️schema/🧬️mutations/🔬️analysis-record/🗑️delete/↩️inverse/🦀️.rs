//! ↩️ Inverse (undo) construction for the `delete-analysis-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔬analyses` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteAnalysisRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.analyses.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateAnalysisRecord(super::super::create_analysis_record::CreateAnalysisRecord { analysis_record: base.analyses[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
