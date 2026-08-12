//! 🔺️ Sparse diff construction for the `site_context` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateSiteContext, DeleteSiteContext, RenameSiteContext, ReplaceSiteContext};
use crate::artifacts::program::diff::{ProgramSiteContextDelta, ProgramSiteContextPatchEntry};
use crate::artifacts::program::registers::SiteContextPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.site_context` on apply.
pub fn diff_create(payload: &CreateSiteContext, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { site_context: Some(ProgramSiteContextDelta { added: vec![payload.site_context.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteSiteContext, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { site_context: Some(ProgramSiteContextDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameSiteContext, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SiteContextPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { site_context: Some(ProgramSiteContextDelta { patched: vec![ProgramSiteContextPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceSiteContext, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.site_context.iter().find(|row| row.header.id == payload.site_context.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.site_context).expect("diff_patch always produces a full patch");
    ProgramDiff { site_context: Some(ProgramSiteContextDelta { patched: vec![ProgramSiteContextPatchEntry { id: payload.site_context.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
