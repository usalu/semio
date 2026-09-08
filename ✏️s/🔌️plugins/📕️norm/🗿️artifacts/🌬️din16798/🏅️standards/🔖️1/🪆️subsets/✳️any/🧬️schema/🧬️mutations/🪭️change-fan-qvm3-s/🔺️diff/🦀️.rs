//! 🔺️ `change-fan-qvm3-s` sparse diff construction — writes only `Din16798Diff.fan_q_v_m3_s` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_fan_q_v_m3_s::ChangeFanQVM3S;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFanQVM3S, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_fan_q_v_m3_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fan volume flow must be a finite number, got {}.", payload.new_fan_q_v_m3_s), Vec::<String>::new());
    }
    if base.fan_q_v_m3_s == payload.new_fan_q_v_m3_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fan volume flow is already {}.", payload.new_fan_q_v_m3_s));
    }
    protocol::MutationOutcome::new(Din16798Diff { fan_q_v_m3_s: Some(payload.new_fan_q_v_m3_s), ..Default::default() })
}
//#endregion 🔖️Diff
