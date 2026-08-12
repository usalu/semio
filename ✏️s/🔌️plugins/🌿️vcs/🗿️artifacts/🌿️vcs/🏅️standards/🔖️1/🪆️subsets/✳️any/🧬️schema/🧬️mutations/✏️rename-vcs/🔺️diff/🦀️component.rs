//! 🔺️ Sparse diff builder for `RenameVcs`.
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameVcs, _base: &VcsSnapshot) -> VcsDiff {
    VcsDiff { title: Some(payload.new_title.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
