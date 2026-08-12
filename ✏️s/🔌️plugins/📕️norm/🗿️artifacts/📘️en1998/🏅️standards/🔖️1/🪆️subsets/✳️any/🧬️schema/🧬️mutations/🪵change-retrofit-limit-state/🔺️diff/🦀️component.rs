//! 🔺️ `change-retrofit-limit-state` sparse diff construction — writes only `En1998Diff.retrofit_limit_state` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_limit_state::mutation::ChangeRetrofitLimitState;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitLimitState, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { retrofit_limit_state: Some(payload.new_retrofit_limit_state.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
