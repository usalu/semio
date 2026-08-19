//! 🔺️ `change-storage-allowance-kwh` sparse diff construction — writes only `Din16798Diff.storage_allowance_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_storage_allowance_kwh::mutation::ChangeStorageAllowanceKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeStorageAllowanceKwh, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_storage_allowance_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Storage loss allowance must be a finite number, got {}.", payload.new_storage_allowance_kwh), Vec::<String>::new());
    }
    if base.storage_allowance_kwh == payload.new_storage_allowance_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Storage loss allowance is already {}.", payload.new_storage_allowance_kwh));
    }
    protocol::MutationOutcome::new(Din16798Diff { storage_allowance_kwh: Some(payload.new_storage_allowance_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
