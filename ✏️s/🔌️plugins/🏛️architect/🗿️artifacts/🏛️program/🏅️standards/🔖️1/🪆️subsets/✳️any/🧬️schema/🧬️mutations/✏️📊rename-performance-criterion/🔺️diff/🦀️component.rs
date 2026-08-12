//! 🔺️ Sparse diff construction for the `rename-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::mutation::RenamePerformanceCriterion;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramPerformanceDelta, ProgramPerformancePatchEntry};
use crate::artifacts::program::registers::PerformanceCriterionPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenamePerformanceCriterion, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = PerformanceCriterionPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { performance: Some(ProgramPerformanceDelta { patched: vec![ProgramPerformancePatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
