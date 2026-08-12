//! 🔺️ Sparse diff construction for the `replace-meta` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🏷️update-meta` per Wave C.

use super::mutation::ReplaceMeta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ New `ProgramMeta` wholesale.
pub fn diff(payload: &ReplaceMeta, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { meta: Some(payload.new_meta.clone()), ..Default::default() }
}
