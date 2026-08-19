//! 🔺️ `change-theta-amb-c` sparse diff construction — writes only `Din16798Diff.theta_amb_c` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_theta_amb_c::mutation::ChangeThetaAmbC;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeThetaAmbC, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_theta_amb_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Ambient temperature must be a finite number, got {}.", payload.new_theta_amb_c), Vec::<String>::new());
    }
    if base.theta_amb_c == payload.new_theta_amb_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Ambient temperature is already {}.", payload.new_theta_amb_c));
    }
    protocol::MutationOutcome::new(Din16798Diff { theta_amb_c: Some(payload.new_theta_amb_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
