//! 🔺️ `change-dfm` sparse diff construction — writes only `En1997Diff.d_f_m` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_d_f_m::mutation::ChangeDFM;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeDFM, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_d_f_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Founding depth D_f [m] must be a finite number, got {}.", payload.new_d_f_m), Vec::<String>::new());
    }
    if base.d_f_m == payload.new_d_f_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Founding depth D_f [m] is already {}.", payload.new_d_f_m));
    }
    protocol::MutationOutcome::new(En1997Diff { d_f_m: Some(payload.new_d_f_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
