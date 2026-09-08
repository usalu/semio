//! 🔺️ `change-tank-height-m` sparse diff construction — writes only `En1998Diff.tank_height_m` from the payload.

use crate::diff::En1998Diff;
use crate::mutations::change_tank_height_m::ChangeTankHeightM;
use crate::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTankHeightM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tank_height_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tank height [m] must be a finite number, got {}.", payload.new_tank_height_m), Vec::<String>::new());
    }
    if base.tank_height_m == payload.new_tank_height_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tank height [m] is already {}.", payload.new_tank_height_m));
    }
    protocol::MutationOutcome::new(En1998Diff { tank_height_m: Some(payload.new_tank_height_m), ..Default::default() })
}
//#endregion 🔖️Diff
