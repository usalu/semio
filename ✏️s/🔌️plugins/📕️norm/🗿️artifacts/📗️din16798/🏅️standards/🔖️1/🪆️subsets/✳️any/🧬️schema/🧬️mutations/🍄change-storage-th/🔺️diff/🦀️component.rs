//! 🔺️ `change-storage-th` sparse diff construction — writes only `Din16798Diff.storage_t_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_storage_t_h::mutation::ChangeStorageTH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeStorageTH, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_storage_t_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Storage duration must be a finite number, got {}.", payload.new_storage_t_h), Vec::<String>::new());
    }
    if base.storage_t_h == payload.new_storage_t_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Storage duration is already {}.", payload.new_storage_t_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { storage_t_h: Some(payload.new_storage_t_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
