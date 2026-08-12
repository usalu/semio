//! 🔺️ Sparse diff construction for the `replace-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::mutation::ReplaceSiteContext;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSiteContextDelta, ProgramSiteContextPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceSiteContext, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.site_context.iter().find(|row| row.header.id == payload.site_context.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.site_context).expect("diff_patch always produces a full patch");
    ProgramDiff { site_context: Some(ProgramSiteContextDelta { patched: vec![ProgramSiteContextPatchEntry { id: payload.site_context.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
