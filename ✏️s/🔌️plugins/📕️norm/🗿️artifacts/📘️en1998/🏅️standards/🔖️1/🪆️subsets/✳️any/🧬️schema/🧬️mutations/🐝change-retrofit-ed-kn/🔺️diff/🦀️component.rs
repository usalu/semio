//! 🔺️ `change-retrofit-ed-kn` sparse diff construction — writes only `En1998Diff.retrofit_e_d_kn` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_e_d_kn::mutation::ChangeRetrofitEDKn;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitEDKn, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_retrofit_e_d_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Retrofit demand E_d [kN] must be a finite number, got {}.", payload.new_retrofit_e_d_kn), Vec::<String>::new());
    }
    if base.retrofit_e_d_kn == payload.new_retrofit_e_d_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Retrofit demand E_d [kN] is already {}.", payload.new_retrofit_e_d_kn));
    }
    protocol::MutationOutcome::new(En1998Diff { retrofit_e_d_kn: Some(payload.new_retrofit_e_d_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
