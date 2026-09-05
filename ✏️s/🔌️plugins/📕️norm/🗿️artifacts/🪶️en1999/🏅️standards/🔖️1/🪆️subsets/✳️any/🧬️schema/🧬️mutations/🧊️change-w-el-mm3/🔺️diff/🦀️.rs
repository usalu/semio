//! 🔺️ `change-w-el-mm3` sparse diff construction — writes only `En1999Diff.w_el_mm3` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_w_el_mm3::ChangeWElMm3;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWElMm3, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_w_el_mm3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Elastic section modulus W_el [mm3] must be a finite number, got {}.", payload.new_w_el_mm3), Vec::<String>::new());
    }
    if base.w_el_mm3 == payload.new_w_el_mm3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Elastic section modulus W_el [mm3] is already {}.", payload.new_w_el_mm3));
    }
    protocol::MutationOutcome::new(En1999Diff { w_el_mm3: Some(payload.new_w_el_mm3.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
