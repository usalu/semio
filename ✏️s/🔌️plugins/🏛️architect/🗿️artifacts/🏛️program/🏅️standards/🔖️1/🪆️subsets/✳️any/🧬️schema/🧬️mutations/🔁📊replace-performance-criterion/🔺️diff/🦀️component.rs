//! 🔺️ Sparse diff construction for the `replace-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::mutation::ReplacePerformanceCriterion;
use crate::artifacts::program::diff::{ProgramPerformanceDelta, ProgramPerformancePatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplacePerformanceCriterion, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.performance.iter().find(|row| row.header.id == payload.performance_criterion.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.performance_criterion).expect("diff_patch always produces a full patch");
    ProgramDiff { performance: Some(ProgramPerformanceDelta { patched: vec![ProgramPerformancePatchEntry { id: payload.performance_criterion.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
