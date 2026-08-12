//! 🔺️ Sparse diff construction for the `update_meta` mutation leaf. `program.meta`'s diff slot
//! is a plain `Option<ProgramMeta>` whole-value field (no add/remove/patch delta shape), so the
//! diff is just "the new full value of meta".

use super::mutation::{RenameMeta, ReplaceMeta};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ New `ProgramMeta` with only `title` changed.
pub fn diff_rename(payload: &RenameMeta, base: &ProgramSnapshot) -> ProgramDiff {
    let mut value = base.meta.clone();
    value.title = payload.new_title.clone();
    ProgramDiff { meta: Some(value), ..Default::default() }
}

/// 🔁️ New `ProgramMeta` wholesale.
pub fn diff_replace(payload: &ReplaceMeta, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { meta: Some(payload.new_meta.clone()), ..Default::default() }
}
