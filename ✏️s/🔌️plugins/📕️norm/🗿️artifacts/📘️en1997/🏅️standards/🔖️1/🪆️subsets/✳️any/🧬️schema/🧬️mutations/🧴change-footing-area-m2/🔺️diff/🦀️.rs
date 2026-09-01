//! 🔺️ `change-footing-area-m2` sparse diff construction — writes only `En1997Diff.footing_area_m2` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_footing_area_m2::ChangeFootingAreaM2;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFootingAreaM2, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_footing_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Footing area [m2] must be a finite number, got {}.", payload.new_footing_area_m2), Vec::<String>::new());
    }
    if base.footing_area_m2 == payload.new_footing_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Footing area [m2] is already {}.", payload.new_footing_area_m2));
    }
    protocol::MutationOutcome::new(En1997Diff { footing_area_m2: Some(payload.new_footing_area_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
