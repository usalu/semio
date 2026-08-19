//! 🔺️ `change-l-cr-mm` sparse diff construction — writes only `En1999Diff.l_cr_mm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_l_cr_mm::mutation::ChangeLCrMm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeLCrMm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_l_cr_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Buckling length L_cr [mm] must be a finite number, got {}.", payload.new_l_cr_mm), Vec::<String>::new());
    }
    if base.l_cr_mm == payload.new_l_cr_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Buckling length L_cr [mm] is already {}.", payload.new_l_cr_mm));
    }
    protocol::MutationOutcome::new(En1999Diff { l_cr_mm: Some(payload.new_l_cr_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
