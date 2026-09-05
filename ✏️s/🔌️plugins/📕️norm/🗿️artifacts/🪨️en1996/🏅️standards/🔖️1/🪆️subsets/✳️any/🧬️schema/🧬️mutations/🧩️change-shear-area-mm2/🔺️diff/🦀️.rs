//! 🔺️ `change-shear-area-mm2` sparse diff construction — writes only `En1996Diff.shear_area_mm2` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_shear_area_mm2::ChangeShearAreaMm2;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeShearAreaMm2, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_shear_area_mm2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Shear area mm2 must be a finite number.", Vec::<String>::new());
    }
    if base.shear_area_mm2 == payload.new_shear_area_mm2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Shear area mm2 already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { shear_area_mm2: Some(payload.new_shear_area_mm2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
