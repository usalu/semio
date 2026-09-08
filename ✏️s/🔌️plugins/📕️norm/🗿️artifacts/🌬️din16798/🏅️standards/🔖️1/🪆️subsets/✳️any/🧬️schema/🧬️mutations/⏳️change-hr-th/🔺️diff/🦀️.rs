//! 🔺️ `change-hr-th` sparse diff construction — writes only `Din16798Diff.hr_t_h` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_hr_t_h::ChangeHrTH;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrTH, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_hr_t_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Heat recovery operating time must be a finite number, got {}.", payload.new_hr_t_h), Vec::<String>::new());
    }
    if base.hr_t_h == payload.new_hr_t_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Heat recovery operating time is already {}.", payload.new_hr_t_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { hr_t_h: Some(payload.new_hr_t_h), ..Default::default() })
}
//#endregion 🔖️Diff
