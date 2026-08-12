//! 🔺️ Sparse diff construction for the `delete-analysis-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔬analyses` per Wave C.

use super::mutation::DeleteAnalysisRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAnalysesDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteAnalysisRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { analyses: Some(ProgramAnalysesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
