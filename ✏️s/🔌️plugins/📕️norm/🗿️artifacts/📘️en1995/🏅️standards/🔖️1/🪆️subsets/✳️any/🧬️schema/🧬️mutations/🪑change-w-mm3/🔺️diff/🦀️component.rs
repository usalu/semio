//! 🔺️ `change-w-mm3` sparse diff construction — writes only `En1995Diff.w_mm3` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_w_mm3::mutation::ChangeWMm3;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeWMm3, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_w_mm3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "W mm3 must be a finite number.", Vec::<String>::new());
    }
    if base.w_mm3 == payload.new_w_mm3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "W mm3 already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { w_mm3: Some(payload.new_w_mm3.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
