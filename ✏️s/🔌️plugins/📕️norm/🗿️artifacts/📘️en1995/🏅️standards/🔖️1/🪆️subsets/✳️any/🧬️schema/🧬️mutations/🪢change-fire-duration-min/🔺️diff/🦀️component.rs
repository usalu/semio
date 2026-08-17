//! 🔺️ `change-fire-duration-min` sparse diff construction — writes only `En1995Diff.fire_duration_min` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_fire_duration_min::mutation::ChangeFireDurationMin;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireDurationMin, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_fire_duration_min.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fire duration min must be a finite number.", Vec::<String>::new());
    }
    if base.fire_duration_min == payload.new_fire_duration_min {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fire duration min already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { fire_duration_min: Some(payload.new_fire_duration_min.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
