//! ↩️ `change-retrofit-rk-kn` inverse — restores the pre-change `retrofit_r_k_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_retrofit_r_k_kn::mutation::ChangeRetrofitRKKn;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRetrofitRKKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeRetrofitRKKn(ChangeRetrofitRKKn { new_retrofit_r_k_kn: base.retrofit_r_k_kn.clone() })]
}
//#endregion 🔖️Inverse
