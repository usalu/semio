//! 🔺️ `change-dfm` sparse diff construction — writes only `En1997Diff.d_f_m` from the payload.

use crate::diff::En1997Diff;
use crate::mutations::change_d_f_m::ChangeDFM;
use crate::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDFM, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_d_f_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Founding depth D_f [m] must be a finite number, got {}.", payload.new_d_f_m), Vec::<String>::new());
    }
    if base.d_f_m == payload.new_d_f_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Founding depth D_f [m] is already {}.", payload.new_d_f_m));
    }
    protocol::MutationOutcome::new(En1997Diff { d_f_m: Some(payload.new_d_f_m), ..Default::default() })
}
//#endregion 🔖️Diff
