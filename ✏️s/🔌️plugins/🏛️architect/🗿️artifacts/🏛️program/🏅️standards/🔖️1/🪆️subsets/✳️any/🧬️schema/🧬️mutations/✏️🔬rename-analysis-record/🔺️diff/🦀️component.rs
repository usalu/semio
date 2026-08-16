//! 🔺️ Sparse diff construction for the `rename-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::mutation::RenameAnalysisRecord;
use crate::artifacts::program::diff::{ProgramAnalysesDelta, ProgramAnalysesPatchEntry};
use crate::artifacts::program::registers::AnalysisRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameAnalysisRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = AnalysisRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { analyses: Some(ProgramAnalysesDelta { patched: vec![ProgramAnalysesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
