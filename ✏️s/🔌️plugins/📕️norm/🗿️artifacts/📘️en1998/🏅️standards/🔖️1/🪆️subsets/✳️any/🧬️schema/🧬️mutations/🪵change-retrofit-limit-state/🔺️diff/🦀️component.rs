//! 🔺️ `change-retrofit-limit-state` sparse diff construction — writes only `En1998Diff.retrofit_limit_state` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_retrofit_limit_state::mutation::ChangeRetrofitLimitState;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRetrofitLimitState, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.retrofit_limit_state == payload.new_retrofit_limit_state {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Retrofit limit state is already \"{}\".", payload.new_retrofit_limit_state));
    }
    protocol::MutationOutcome::new(En1998Diff { retrofit_limit_state: Some(payload.new_retrofit_limit_state.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
