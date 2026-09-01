//! 🔺️ `change-bearing-d-rd-mm` sparse diff construction — writes only `En1998Diff.bearing_d_rd_mm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_bearing_d_rd_mm::ChangeBearingDRdMm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBearingDRdMm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_bearing_d_rd_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Bearing design displacement capacity D_Rd [mm] must be a finite number, got {}.", payload.new_bearing_d_rd_mm), Vec::<String>::new());
    }
    if base.bearing_d_rd_mm == payload.new_bearing_d_rd_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Bearing design displacement capacity D_Rd [mm] is already {}.", payload.new_bearing_d_rd_mm));
    }
    protocol::MutationOutcome::new(En1998Diff { bearing_d_rd_mm: Some(payload.new_bearing_d_rd_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
