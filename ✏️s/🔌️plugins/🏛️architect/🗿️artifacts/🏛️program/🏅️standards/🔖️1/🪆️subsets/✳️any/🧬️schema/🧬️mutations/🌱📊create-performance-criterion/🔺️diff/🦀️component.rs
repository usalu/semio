//! 🔺️ Sparse diff construction for the `create-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::mutation::CreatePerformanceCriterion;
use crate::artifacts::program::diff::ProgramPerformanceDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.performance` on apply.
pub fn diff(payload: &CreatePerformanceCriterion, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { performance: Some(ProgramPerformanceDelta { added: vec![payload.performance_criterion.clone()], ..Default::default() }), ..Default::default() }
}
