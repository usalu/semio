//! 🔺️ Sparse diff builder for `ChangeCounter`.
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeCounter, _base: &VcsSnapshot) -> VcsDiff {
    VcsDiff { counter: Some(payload.new_counter), ..Default::default() }
}
//#endregion 🔖️Diff
