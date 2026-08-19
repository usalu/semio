//! 🔺️ Sparse diff builder for `RenameVcs`.
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Warning `no-op` when `new_title` already equals `base.title`.
pub async fn diff(payload: &super::mutation::RenameVcs, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    if base.title == payload.new_title {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Title is already \"{}\".", payload.new_title));
    }
    protocol::MutationOutcome::new(VcsDiff { title: Some(payload.new_title.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
