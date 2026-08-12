//! 🔺️ Sparse diff construction for the `replace-assumption` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `💭assumptions` per Wave C.

use super::mutation::ReplaceAssumption;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramAssumptionsDelta, ProgramAssumptionsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceAssumption, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.assumptions.iter().find(|row| row.header.id == payload.assumption.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.assumption).expect("diff_patch always produces a full patch");
    ProgramDiff { assumptions: Some(ProgramAssumptionsDelta { patched: vec![ProgramAssumptionsPatchEntry { id: payload.assumption.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
