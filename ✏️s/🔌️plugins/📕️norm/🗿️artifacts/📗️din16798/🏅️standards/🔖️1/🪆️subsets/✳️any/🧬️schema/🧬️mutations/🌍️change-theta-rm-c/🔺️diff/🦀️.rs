//! 🔺️ `change-theta-rm-c` sparse diff construction — writes only `Din16798Diff.theta_rm_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_theta_rm_c::ChangeThetaRmC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaRmC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_theta_rm_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Running mean outdoor temperature must be a finite number, got {}.", payload.new_theta_rm_c), Vec::<String>::new());
    }
    if base.theta_rm_c == payload.new_theta_rm_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Running mean outdoor temperature is already {}.", payload.new_theta_rm_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { theta_rm_c: Some(payload.new_theta_rm_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
