//! 🔺️ Sparse diff builder for `ChangeStatus`.
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeStatus, _base: &VcsSnapshot) -> VcsDiff {
    VcsDiff { status: Some(payload.new_status.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
