//! 🔺️ `change-h-ve-wk` sparse diff construction — writes only `Din16798Diff.h_ve_w_k` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_h_ve_w_k::ChangeHVeWK;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHVeWK, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_h_ve_w_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Ventilation heat transfer coefficient must be a finite number, got {}.", payload.new_h_ve_w_k), Vec::<String>::new());
    }
    if base.h_ve_w_k == payload.new_h_ve_w_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Ventilation heat transfer coefficient is already {}.", payload.new_h_ve_w_k));
    }
    protocol::MutationOutcome::new(Din16798Diff { h_ve_w_k: Some(payload.new_h_ve_w_k), ..Default::default() })
}
//#endregion 🔖️Diff
