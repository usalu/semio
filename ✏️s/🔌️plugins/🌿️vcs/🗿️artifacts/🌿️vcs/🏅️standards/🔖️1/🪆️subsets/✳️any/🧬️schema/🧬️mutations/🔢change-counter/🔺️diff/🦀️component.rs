//! 🔺️ Sparse diff builder for `ChangeCounter`.
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Warning `no-op` when `new_counter` already equals `base.counter`.
pub fn diff(payload: &super::mutation::ChangeCounter, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    if base.counter == payload.new_counter {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Counter is already {}.", payload.new_counter));
    }
    protocol::MutationOutcome::new(VcsDiff { counter: Some(payload.new_counter), ..Default::default() })
}
//#endregion 🔖️Diff
