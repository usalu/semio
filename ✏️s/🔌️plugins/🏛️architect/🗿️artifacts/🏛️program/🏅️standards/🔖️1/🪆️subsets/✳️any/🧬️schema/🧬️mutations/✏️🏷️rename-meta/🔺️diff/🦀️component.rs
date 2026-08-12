//! 🔺️ Sparse diff construction for the `rename-meta` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏷️update-meta` per Wave C.

use super::mutation::RenameMeta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `ProgramMeta` with only `title` changed.
pub fn diff(payload: &RenameMeta, base: &ProgramSnapshot) -> ProgramDiff {
    let mut value = base.meta.clone();
    value.title = payload.new_title.clone();
    ProgramDiff { meta: Some(value), ..Default::default() }
}
