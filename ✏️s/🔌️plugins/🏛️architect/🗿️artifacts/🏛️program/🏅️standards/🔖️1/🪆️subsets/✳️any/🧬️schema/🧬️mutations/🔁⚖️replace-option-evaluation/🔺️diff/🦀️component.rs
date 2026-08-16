//! 🔺️ Sparse diff construction for the `replace-option-evaluation` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚖️options` per Wave C.

use super::mutation::ReplaceOptionEvaluation;
use crate::artifacts::program::diff::{ProgramOptionsDelta, ProgramOptionsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceOptionEvaluation, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.options.iter().find(|row| row.header.id == payload.option_evaluation.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.option_evaluation).expect("diff_patch always produces a full patch");
    ProgramDiff { options: Some(ProgramOptionsDelta { patched: vec![ProgramOptionsPatchEntry { id: payload.option_evaluation.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
