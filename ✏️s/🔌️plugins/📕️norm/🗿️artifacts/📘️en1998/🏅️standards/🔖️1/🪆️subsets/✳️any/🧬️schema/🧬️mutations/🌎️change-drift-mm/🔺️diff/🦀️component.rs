//! 🔺️ `change-drift-mm` sparse diff construction — writes only `En1998Diff.drift_mm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_drift_mm::mutation::ChangeDriftMm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDriftMm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_drift_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Interstorey drift [mm] must be a finite number, got {}.", payload.new_drift_mm), Vec::<String>::new());
    }
    if base.drift_mm == payload.new_drift_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Interstorey drift [mm] is already {}.", payload.new_drift_mm));
    }
    protocol::MutationOutcome::new(En1998Diff { drift_mm: Some(payload.new_drift_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
