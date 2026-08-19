//! 🔺️ `change-theta-c` sparse diff construction — writes only `En1999Diff.theta_c` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_theta_c::mutation::ChangeThetaC;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeThetaC, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_theta_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fatigue detail category theta_C [MPa] must be a finite number, got {}.", payload.new_theta_c), Vec::<String>::new());
    }
    if base.theta_c == payload.new_theta_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fatigue detail category theta_C [MPa] is already {}.", payload.new_theta_c));
    }
    protocol::MutationOutcome::new(En1999Diff { theta_c: Some(payload.new_theta_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
