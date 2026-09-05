//! 🔺️ `change-silo-v-rd-kn` sparse diff construction — writes only `En1998Diff.silo_v_rd_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_silo_v_rd_kn::ChangeSiloVRdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloVRdKn, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_silo_v_rd_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo shear resistance V_Rd [kN] must be a finite number, got {}.", payload.new_silo_v_rd_kn), Vec::<String>::new());
    }
    if base.silo_v_rd_kn == payload.new_silo_v_rd_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Silo shear resistance V_Rd [kN] is already {}.", payload.new_silo_v_rd_kn));
    }
    protocol::MutationOutcome::new(En1998Diff { silo_v_rd_kn: Some(payload.new_silo_v_rd_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
