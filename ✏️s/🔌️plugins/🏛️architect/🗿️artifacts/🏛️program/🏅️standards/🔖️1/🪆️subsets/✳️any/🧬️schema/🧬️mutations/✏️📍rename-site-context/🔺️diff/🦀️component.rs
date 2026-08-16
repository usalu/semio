//! 🔺️ Sparse diff construction for the `rename-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::mutation::RenameSiteContext;
use crate::artifacts::program::diff::{ProgramSiteContextDelta, ProgramSiteContextPatchEntry};
use crate::artifacts::program::registers::SiteContextPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameSiteContext, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SiteContextPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { site_context: Some(ProgramSiteContextDelta { patched: vec![ProgramSiteContextPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
