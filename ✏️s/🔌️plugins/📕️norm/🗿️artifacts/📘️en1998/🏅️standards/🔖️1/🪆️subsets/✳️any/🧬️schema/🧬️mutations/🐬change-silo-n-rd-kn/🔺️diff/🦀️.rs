//! 🔺️ `change-silo-n-rd-kn` sparse diff construction — writes only `En1998Diff.silo_n_rd_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_silo_n_rd_kn::ChangeSiloNRdKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloNRdKn, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_silo_n_rd_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Silo axial resistance N_Rd [kN] must be a finite number, got {}.", payload.new_silo_n_rd_kn), Vec::<String>::new());
    }
    if base.silo_n_rd_kn == payload.new_silo_n_rd_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Silo axial resistance N_Rd [kN] is already {}.", payload.new_silo_n_rd_kn));
    }
    protocol::MutationOutcome::new(En1998Diff { silo_n_rd_kn: Some(payload.new_silo_n_rd_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
