//! 🔺️ Sparse diff construction for the `delete-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::mutation::DeletePerformanceCriterion;
use crate::artifacts::program::diff::ProgramPerformanceDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeletePerformanceCriterion, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { performance: Some(ProgramPerformanceDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
