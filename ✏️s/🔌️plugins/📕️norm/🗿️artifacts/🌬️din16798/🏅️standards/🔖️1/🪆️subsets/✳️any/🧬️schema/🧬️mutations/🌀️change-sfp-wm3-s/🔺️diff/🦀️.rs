//! 🔺️ `change-sfp-wm3-s` sparse diff construction — writes only `Din16798Diff.sfp_w_m3_s` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_sfp_w_m3_s::ChangeSfpWM3S;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSfpWM3S, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_sfp_w_m3_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Specific fan power must be a finite number, got {}.", payload.new_sfp_w_m3_s), Vec::<String>::new());
    }
    if base.sfp_w_m3_s == payload.new_sfp_w_m3_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Specific fan power is already {}.", payload.new_sfp_w_m3_s));
    }
    protocol::MutationOutcome::new(Din16798Diff { sfp_w_m3_s: Some(payload.new_sfp_w_m3_s), ..Default::default() })
}
//#endregion 🔖️Diff
