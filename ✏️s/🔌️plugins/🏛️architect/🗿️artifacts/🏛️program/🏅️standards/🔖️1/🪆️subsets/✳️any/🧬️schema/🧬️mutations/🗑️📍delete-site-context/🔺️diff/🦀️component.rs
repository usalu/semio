//! 🔺️ Sparse diff construction for the `delete-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::mutation::DeleteSiteContext;
use crate::artifacts::program::diff::ProgramSiteContextDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteSiteContext, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { site_context: Some(ProgramSiteContextDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
