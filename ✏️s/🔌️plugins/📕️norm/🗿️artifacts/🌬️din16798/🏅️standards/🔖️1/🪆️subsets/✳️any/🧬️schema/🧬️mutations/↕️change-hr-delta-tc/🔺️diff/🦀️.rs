//! 🔺️ `change-hr-delta-tc` sparse diff construction — writes only `Din16798Diff.hr_delta_t_c` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_hr_delta_t_c::ChangeHrDeltaTC;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrDeltaTC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_hr_delta_t_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Heat recovery temperature difference must be a finite number, got {}.", payload.new_hr_delta_t_c), Vec::<String>::new());
    }
    if base.hr_delta_t_c == payload.new_hr_delta_t_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Heat recovery temperature difference is already {}.", payload.new_hr_delta_t_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { hr_delta_t_c: Some(payload.new_hr_delta_t_c), ..Default::default() })
}
//#endregion 🔖️Diff
