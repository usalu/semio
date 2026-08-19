//! 🔺️ `change-h-ef-mm` sparse diff construction — writes only `En1996Diff.h_ef_mm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_h_ef_mm::mutation::ChangeHEfMm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeHEfMm, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_h_ef_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "H ef mm must be a finite number.", Vec::<String>::new());
    }
    if base.h_ef_mm == payload.new_h_ef_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "H ef mm already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { h_ef_mm: Some(payload.new_h_ef_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
