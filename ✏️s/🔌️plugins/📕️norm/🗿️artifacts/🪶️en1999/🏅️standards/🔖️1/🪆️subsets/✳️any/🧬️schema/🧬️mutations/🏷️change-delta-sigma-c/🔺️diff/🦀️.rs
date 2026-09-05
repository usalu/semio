//! 🔺️ `change-delta-sigma-c` sparse diff construction — writes only `En1999Diff.delta_sigma_c` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_delta_sigma_c::ChangeDeltaSigmaC;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaSigmaC, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_delta_sigma_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fatigue reference stress range [MPa] must be a finite number, got {}.", payload.new_delta_sigma_c), Vec::<String>::new());
    }
    if base.delta_sigma_c == payload.new_delta_sigma_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fatigue reference stress range [MPa] is already {}.", payload.new_delta_sigma_c));
    }
    protocol::MutationOutcome::new(En1999Diff { delta_sigma_c: Some(payload.new_delta_sigma_c.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
