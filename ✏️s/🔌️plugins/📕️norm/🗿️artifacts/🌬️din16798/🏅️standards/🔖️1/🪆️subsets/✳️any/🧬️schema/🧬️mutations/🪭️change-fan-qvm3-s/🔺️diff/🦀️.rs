//! 🔺️ `change-fan-qvm3-s` sparse diff construction — writes only `Din16798Diff.fan_q_v_m3_s` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_fan_q_v_m3_s::ChangeFanQVM3S;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFanQVM3S, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_fan_q_v_m3_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fan volume flow must be a finite number, got {}.", payload.new_fan_q_v_m3_s), Vec::<String>::new());
    }
    if base.fan_q_v_m3_s == payload.new_fan_q_v_m3_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fan volume flow is already {}.", payload.new_fan_q_v_m3_s));
    }
    protocol::MutationOutcome::new(Din16798Diff { fan_q_v_m3_s: Some(payload.new_fan_q_v_m3_s.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
