//! 🔺️ Sparse diff construction for the `create-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::mutation::CreateAnalysisRecord;
use crate::artifacts::program::diff::ProgramAnalysesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.analyses` on apply.
pub fn diff(payload: &CreateAnalysisRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { analyses: Some(ProgramAnalysesDelta { added: vec![payload.analysis_record.clone()], ..Default::default() }), ..Default::default() }
}
