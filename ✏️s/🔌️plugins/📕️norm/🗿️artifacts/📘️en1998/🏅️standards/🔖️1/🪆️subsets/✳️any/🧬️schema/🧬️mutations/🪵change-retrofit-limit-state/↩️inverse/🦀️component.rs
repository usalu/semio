//! ↩️ `change-retrofit-limit-state` inverse — restores the pre-change `retrofit_limit_state` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_retrofit_limit_state::mutation::ChangeRetrofitLimitState;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeRetrofitLimitState, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeRetrofitLimitState(ChangeRetrofitLimitState { new_retrofit_limit_state: base.retrofit_limit_state.clone() })]
}
//#endregion 🔖️Inverse
