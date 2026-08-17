//! 🔺️ `change-theta-ec` sparse diff construction — writes only `Din16798Diff.theta_e_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_theta_e_c::mutation::ChangeThetaEC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaEC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_theta_e_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Outdoor design temperature must be a finite number, got {}.", payload.new_theta_e_c), Vec::<String>::new());
    }
    if base.theta_e_c == payload.new_theta_e_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Outdoor design temperature is already {}.", payload.new_theta_e_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { theta_e_c: Some(payload.new_theta_e_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
