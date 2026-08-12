//! 🔺️ Sparse diff construction for the `create-site-context` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📍site-context` per Wave C.

use super::mutation::CreateSiteContext;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSiteContextDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.site_context` on apply.
pub fn diff(payload: &CreateSiteContext, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { site_context: Some(ProgramSiteContextDelta { added: vec![payload.site_context.clone()], ..Default::default() }), ..Default::default() }
}
