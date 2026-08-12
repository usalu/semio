//! 🔺️ `change-retrofit-gamma-el` sparse diff construction — writes only `En1998Diff.retrofit_gamma_el` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_gamma_el::mutation::ChangeRetrofitGammaEl;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitGammaEl, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { retrofit_gamma_el: Some(payload.new_retrofit_gamma_el.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
