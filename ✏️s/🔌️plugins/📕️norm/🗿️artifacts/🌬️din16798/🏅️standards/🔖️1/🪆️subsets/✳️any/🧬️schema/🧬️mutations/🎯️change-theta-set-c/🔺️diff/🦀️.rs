//! 🔺️ `change-theta-set-c` sparse diff construction — writes only `Din16798Diff.theta_set_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_theta_set_c::ChangeThetaSetC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaSetC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_theta_set_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cooling set-point temperature must be a finite number, got {}.", payload.new_theta_set_c), Vec::<String>::new());
    }
    if base.theta_set_c == payload.new_theta_set_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cooling set-point temperature is already {}.", payload.new_theta_set_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { theta_set_c: Some(payload.new_theta_set_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
