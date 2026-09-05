//! ↩️ `change-retrofit-ed-kn` inverse — restores the pre-change `retrofit_e_d_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_retrofit_e_d_kn::ChangeRetrofitEDKn;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRetrofitEDKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeRetrofitEDKn(ChangeRetrofitEDKn { new_retrofit_e_d_kn: base.retrofit_e_d_kn.clone() })]
}
//#endregion 🔖️Inverse
