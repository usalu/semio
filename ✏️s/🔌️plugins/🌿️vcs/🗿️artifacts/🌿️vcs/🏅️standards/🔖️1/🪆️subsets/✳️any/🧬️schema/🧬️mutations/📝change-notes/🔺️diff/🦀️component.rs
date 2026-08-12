//! 🔺️ Sparse diff builder for `ChangeNotes`.
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNotes, _base: &VcsSnapshot) -> VcsDiff {
    VcsDiff { notes: Some(payload.new_notes.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
