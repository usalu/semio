//! 🔺️ `change-volume-m3` sparse diff construction — writes only `Din16798Diff.volume_m3` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_volume_m3::ChangeVolumeM3;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeVolumeM3, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_volume_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Building volume must be a finite number, got {}.", payload.new_volume_m3), Vec::<String>::new());
    }
    if base.volume_m3 == payload.new_volume_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Building volume is already {}.", payload.new_volume_m3));
    }
    protocol::MutationOutcome::new(Din16798Diff { volume_m3: Some(payload.new_volume_m3.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
