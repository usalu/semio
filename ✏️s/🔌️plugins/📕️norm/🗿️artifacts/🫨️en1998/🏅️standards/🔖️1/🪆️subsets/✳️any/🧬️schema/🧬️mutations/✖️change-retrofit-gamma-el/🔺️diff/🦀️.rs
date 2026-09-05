//! 🔺️ `change-retrofit-gamma-el` sparse diff construction — writes only `En1998Diff.retrofit_gamma_el` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_gamma_el::ChangeRetrofitGammaEl;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitGammaEl, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_retrofit_gamma_el.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Retrofit confidence factor gamma_el must be a finite number, got {}.", payload.new_retrofit_gamma_el), Vec::<String>::new());
    }
    if base.retrofit_gamma_el == payload.new_retrofit_gamma_el {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Retrofit confidence factor gamma_el is already {}.", payload.new_retrofit_gamma_el));
    }
    protocol::MutationOutcome::new(En1998Diff { retrofit_gamma_el: Some(payload.new_retrofit_gamma_el.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
