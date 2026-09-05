//! 🔺️ `change-tank-radius-m` sparse diff construction — writes only `En1998Diff.tank_radius_m` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tank_radius_m::ChangeTankRadiusM;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTankRadiusM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tank_radius_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tank radius [m] must be a finite number, got {}.", payload.new_tank_radius_m), Vec::<String>::new());
    }
    if base.tank_radius_m == payload.new_tank_radius_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tank radius [m] is already {}.", payload.new_tank_radius_m));
    }
    protocol::MutationOutcome::new(En1998Diff { tank_radius_m: Some(payload.new_tank_radius_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
