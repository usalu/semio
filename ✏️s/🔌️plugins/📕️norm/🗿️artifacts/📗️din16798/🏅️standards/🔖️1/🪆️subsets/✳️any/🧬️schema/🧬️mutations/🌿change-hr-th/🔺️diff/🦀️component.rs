//! 🔺️ `change-hr-th` sparse diff construction — writes only `Din16798Diff.hr_t_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_hr_t_h::mutation::ChangeHrTH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrTH, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_hr_t_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Heat recovery operating time must be a finite number, got {}.", payload.new_hr_t_h), Vec::<String>::new());
    }
    if base.hr_t_h == payload.new_hr_t_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Heat recovery operating time is already {}.", payload.new_hr_t_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { hr_t_h: Some(payload.new_hr_t_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
