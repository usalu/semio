//! 🔺️ `change-floor-area-m2` sparse diff construction — writes only `Din16798Diff.floor_area_m2` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_floor_area_m2::mutation::ChangeFloorAreaM2;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFloorAreaM2, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_floor_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Floor area must be a finite number, got {}.", payload.new_floor_area_m2), Vec::<String>::new());
    }
    if base.floor_area_m2 == payload.new_floor_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Floor area is already {}.", payload.new_floor_area_m2));
    }
    protocol::MutationOutcome::new(Din16798Diff { floor_area_m2: Some(payload.new_floor_area_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
