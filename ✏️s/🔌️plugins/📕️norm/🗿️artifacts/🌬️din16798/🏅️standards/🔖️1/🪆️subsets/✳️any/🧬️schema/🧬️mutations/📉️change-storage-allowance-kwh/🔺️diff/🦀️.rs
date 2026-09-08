//! 🔺️ `change-storage-allowance-kwh` sparse diff construction — writes only `Din16798Diff.storage_allowance_kwh` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_storage_allowance_kwh::ChangeStorageAllowanceKwh;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStorageAllowanceKwh, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_storage_allowance_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Storage loss allowance must be a finite number, got {}.", payload.new_storage_allowance_kwh), Vec::<String>::new());
    }
    if base.storage_allowance_kwh == payload.new_storage_allowance_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Storage loss allowance is already {}.", payload.new_storage_allowance_kwh));
    }
    protocol::MutationOutcome::new(Din16798Diff { storage_allowance_kwh: Some(payload.new_storage_allowance_kwh), ..Default::default() })
}
//#endregion 🔖️Diff
