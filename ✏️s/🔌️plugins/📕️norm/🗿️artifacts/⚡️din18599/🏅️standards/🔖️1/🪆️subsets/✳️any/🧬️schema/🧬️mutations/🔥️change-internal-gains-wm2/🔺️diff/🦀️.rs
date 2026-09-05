//! 🔺️ `change-internal-gains-wm2` sparse diff construction — writes only `Din18599Diff.internal_gains_w_m2` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_internal_gains_w_m2::ChangeInternalGainsWM2;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeInternalGainsWM2, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_internal_gains_w_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Internal gains wm2 must be a finite number.", Vec::<String>::new());
    }
    if base.internal_gains_w_m2 == payload.new_internal_gains_w_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Internal gains wm2 already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { internal_gains_w_m2: Some(payload.new_internal_gains_w_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
