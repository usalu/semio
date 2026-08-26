//! 🔺️ `change-foundation-area-m2` sparse diff construction — writes only `En1998Diff.foundation_area_m2` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_foundation_area_m2::mutation::ChangeFoundationAreaM2;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFoundationAreaM2, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_foundation_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Foundation area [m2] must be a finite number, got {}.", payload.new_foundation_area_m2), Vec::<String>::new());
    }
    if base.foundation_area_m2 == payload.new_foundation_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Foundation area [m2] is already {}.", payload.new_foundation_area_m2));
    }
    protocol::MutationOutcome::new(En1998Diff { foundation_area_m2: Some(payload.new_foundation_area_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
