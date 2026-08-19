//! 🔺️ Sparse diff builder for `ChangeNotes`.
use crate::artifacts::vcs::{VcsDiff, VcsSnapshot};

//#region 🔖️Diff
/// 🔺️ Warning `no-op` when `new_notes` already equals `base.notes`.
pub async fn diff(payload: &super::mutation::ChangeNotes, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
    if base.notes == payload.new_notes {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Notes are already identical to the requested replacement.");
    }
    protocol::MutationOutcome::new(VcsDiff { notes: Some(payload.new_notes.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
