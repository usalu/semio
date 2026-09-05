//! 🔺️ `change-retrofit-rk-kn` sparse diff construction — writes only `En1998Diff.retrofit_r_k_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_r_k_kn::ChangeRetrofitRKKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitRKKn, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_retrofit_r_k_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Retrofit capacity R_k [kN] must be a finite number, got {}.", payload.new_retrofit_r_k_kn), Vec::<String>::new());
    }
    if base.retrofit_r_k_kn == payload.new_retrofit_r_k_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Retrofit capacity R_k [kN] is already {}.", payload.new_retrofit_r_k_kn));
    }
    protocol::MutationOutcome::new(En1998Diff { retrofit_r_k_kn: Some(payload.new_retrofit_r_k_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
