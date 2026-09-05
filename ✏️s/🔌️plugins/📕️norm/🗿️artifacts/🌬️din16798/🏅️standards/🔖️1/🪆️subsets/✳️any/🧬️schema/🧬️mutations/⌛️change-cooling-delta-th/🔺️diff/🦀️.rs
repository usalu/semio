//! 🔺️ `change-cooling-delta-th` sparse diff construction — writes only `Din16798Diff.cooling_delta_t_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_cooling_delta_t_h::ChangeCoolingDeltaTH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCoolingDeltaTH, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_cooling_delta_t_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cooling degree hours must be a finite number, got {}.", payload.new_cooling_delta_t_h), Vec::<String>::new());
    }
    if base.cooling_delta_t_h == payload.new_cooling_delta_t_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cooling degree hours is already {}.", payload.new_cooling_delta_t_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { cooling_delta_t_h: Some(payload.new_cooling_delta_t_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
