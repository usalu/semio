//! 🔺️ `change-hr-cp-j-kgk` sparse diff construction — writes only `Din16798Diff.hr_cp_j_kgk` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_hr_cp_j_kgk::mutation::ChangeHrCpJKgk;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHrCpJKgk, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_hr_cp_j_kgk.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Heat recovery specific heat capacity must be a finite number, got {}.", payload.new_hr_cp_j_kgk), Vec::<String>::new());
    }
    if base.hr_cp_j_kgk == payload.new_hr_cp_j_kgk {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Heat recovery specific heat capacity is already {}.", payload.new_hr_cp_j_kgk));
    }
    protocol::MutationOutcome::new(Din16798Diff { hr_cp_j_kgk: Some(payload.new_hr_cp_j_kgk.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
