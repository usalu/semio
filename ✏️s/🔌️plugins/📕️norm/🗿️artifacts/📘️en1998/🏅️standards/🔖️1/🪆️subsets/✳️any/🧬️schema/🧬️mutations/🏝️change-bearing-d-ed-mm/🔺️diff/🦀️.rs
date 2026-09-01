//! 🔺️ `change-bearing-d-ed-mm` sparse diff construction — writes only `En1998Diff.bearing_d_ed_mm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_bearing_d_ed_mm::ChangeBearingDEdMm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBearingDEdMm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_bearing_d_ed_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bearing design displacement D_Ed [mm] must be a finite number, got {}.", payload.new_bearing_d_ed_mm), Vec::<String>::new());
    }
    if base.bearing_d_ed_mm == payload.new_bearing_d_ed_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Bearing design displacement D_Ed [mm] is already {}.", payload.new_bearing_d_ed_mm));
    }
    protocol::MutationOutcome::new(En1998Diff { bearing_d_ed_mm: Some(payload.new_bearing_d_ed_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
