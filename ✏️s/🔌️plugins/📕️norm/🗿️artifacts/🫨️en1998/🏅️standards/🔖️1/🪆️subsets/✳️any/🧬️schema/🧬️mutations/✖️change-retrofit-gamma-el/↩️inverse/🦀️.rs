//! ↩️ `change-retrofit-gamma-el` inverse — restores the pre-change `retrofit_gamma_el` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_retrofit_gamma_el::ChangeRetrofitGammaEl;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRetrofitGammaEl, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeRetrofitGammaEl(ChangeRetrofitGammaEl { new_retrofit_gamma_el: base.retrofit_gamma_el.clone() })]
}
//#endregion 🔖️Inverse
