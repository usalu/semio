//! 🔺️ `change-delta-sigma-ed` sparse diff construction — writes only `En1999Diff.delta_sigma_ed` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_delta_sigma_ed::mutation::ChangeDeltaSigmaEd;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeDeltaSigmaEd, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_delta_sigma_ed.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fatigue design stress range [MPa] must be a finite number, got {}.", payload.new_delta_sigma_ed), Vec::<String>::new());
    }
    if base.delta_sigma_ed == payload.new_delta_sigma_ed {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fatigue design stress range [MPa] is already {}.", payload.new_delta_sigma_ed));
    }
    protocol::MutationOutcome::new(En1999Diff { delta_sigma_ed: Some(payload.new_delta_sigma_ed.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
