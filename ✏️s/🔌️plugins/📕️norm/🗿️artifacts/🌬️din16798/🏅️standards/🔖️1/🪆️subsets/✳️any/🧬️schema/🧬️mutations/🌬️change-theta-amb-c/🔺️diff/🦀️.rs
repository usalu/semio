//! 🔺️ `change-theta-amb-c` sparse diff construction — writes only `Din16798Diff.theta_amb_c` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_theta_amb_c::ChangeThetaAmbC;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeThetaAmbC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_theta_amb_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Ambient temperature must be a finite number, got {}.", payload.new_theta_amb_c), Vec::<String>::new());
    }
    if base.theta_amb_c == payload.new_theta_amb_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Ambient temperature is already {}.", payload.new_theta_amb_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { theta_amb_c: Some(payload.new_theta_amb_c), ..Default::default() })
}
//#endregion 🔖️Diff
