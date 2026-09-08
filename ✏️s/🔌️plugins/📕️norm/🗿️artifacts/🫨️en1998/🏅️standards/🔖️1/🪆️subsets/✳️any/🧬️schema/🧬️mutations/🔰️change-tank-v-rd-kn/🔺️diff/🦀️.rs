//! 🔺️ `change-tank-v-rd-kn` sparse diff construction — writes only `En1998Diff.tank_v_rd_kn` from the payload.

use crate::diff::En1998Diff;
use crate::mutations::change_tank_v_rd_kn::ChangeTankVRdKn;
use crate::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTankVRdKn, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tank_v_rd_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tank shear resistance V_Rd [kN] must be a finite number, got {}.", payload.new_tank_v_rd_kn), Vec::<String>::new());
    }
    if base.tank_v_rd_kn == payload.new_tank_v_rd_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tank shear resistance V_Rd [kN] is already {}.", payload.new_tank_v_rd_kn));
    }
    protocol::MutationOutcome::new(En1998Diff { tank_v_rd_kn: Some(payload.new_tank_v_rd_kn), ..Default::default() })
}
//#endregion 🔖️Diff
