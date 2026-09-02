//! 🔺️ Sparse diff builder for `ChangeStatus`.
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Warning `no-op` when `new_status` already equals `base.status`.
pub fn diff(payload: &super::ChangeStatus, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    if base.status == payload.new_status {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Status is already \"{}\".", payload.new_status));
    }
    protocol::MutationOutcome::new(VcsDiff { status: Some(payload.new_status.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
