//! 🔺️ `change-theta-st-c` sparse diff construction — writes only `Din16798Diff.theta_st_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_theta_st_c::mutation::ChangeThetaStC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaStC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_theta_st_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Storage temperature must be a finite number, got {}.", payload.new_theta_st_c), Vec::<String>::new());
    }
    if base.theta_st_c == payload.new_theta_st_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Storage temperature is already {}.", payload.new_theta_st_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { theta_st_c: Some(payload.new_theta_st_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
