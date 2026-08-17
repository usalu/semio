//! 🔺️ `change-h-st-wk` sparse diff construction — writes only `Din16798Diff.h_st_w_k` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_h_st_w_k::mutation::ChangeHStWK;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHStWK, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_h_st_w_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Storage heat transfer coefficient must be a finite number, got {}.", payload.new_h_st_w_k), Vec::<String>::new());
    }
    if base.h_st_w_k == payload.new_h_st_w_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Storage heat transfer coefficient is already {}.", payload.new_h_st_w_k));
    }
    protocol::MutationOutcome::new(Din16798Diff { h_st_w_k: Some(payload.new_h_st_w_k.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
