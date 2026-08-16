//! 🔺️ Sparse diff construction for the `replace-decision` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `✅decisions` per Wave C.

use super::mutation::ReplaceDecision;
use crate::artifacts::program::diff::{ProgramDecisionsDelta, ProgramDecisionsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceDecision, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.decisions.iter().find(|row| row.header.id == payload.decision.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.decision).expect("diff_patch always produces a full patch");
    ProgramDiff { decisions: Some(ProgramDecisionsDelta { patched: vec![ProgramDecisionsPatchEntry { id: payload.decision.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
