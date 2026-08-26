//! 🔺️ `change-t-ef-mm` sparse diff construction — writes only `En1996Diff.t_ef_mm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_t_ef_mm::mutation::ChangeTEfMm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTEfMm, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_t_ef_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "T ef mm must be a finite number.", Vec::<String>::new());
    }
    if base.t_ef_mm == payload.new_t_ef_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "T ef mm already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { t_ef_mm: Some(payload.new_t_ef_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
