//! 🔺️ Sparse diff construction for the `replace-risk` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚠️risks` per Wave C.

use super::mutation::ReplaceRisk;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramRisksDelta, ProgramRisksPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceRisk, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.risks.iter().find(|row| row.header.id == payload.risk.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.risk).expect("diff_patch always produces a full patch");
    ProgramDiff { risks: Some(ProgramRisksDelta { patched: vec![ProgramRisksPatchEntry { id: payload.risk.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
