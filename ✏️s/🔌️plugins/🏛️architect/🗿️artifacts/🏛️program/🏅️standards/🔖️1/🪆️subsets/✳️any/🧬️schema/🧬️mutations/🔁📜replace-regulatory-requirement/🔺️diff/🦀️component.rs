//! 🔺️ Sparse diff construction for the `replace-regulatory-requirement` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📜regulatory` per Wave C.

use super::mutation::ReplaceRegulatoryRequirement;
use crate::artifacts::program::diff::{ProgramRegulatoryDelta, ProgramRegulatoryPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceRegulatoryRequirement, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.regulatory.iter().find(|row| row.header.id == payload.regulatory_requirement.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.regulatory_requirement).expect("diff_patch always produces a full patch");
    ProgramDiff { regulatory: Some(ProgramRegulatoryDelta { patched: vec![ProgramRegulatoryPatchEntry { id: payload.regulatory_requirement.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
