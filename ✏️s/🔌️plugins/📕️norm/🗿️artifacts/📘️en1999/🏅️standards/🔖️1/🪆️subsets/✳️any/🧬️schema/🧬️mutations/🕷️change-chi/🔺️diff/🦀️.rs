//! 🔺️ `change-chi` sparse diff construction — writes only `En1999Diff.chi` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_chi::ChangeChi;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeChi, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_chi.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Buckling reduction factor chi must be a finite number, got {}.", payload.new_chi), Vec::<String>::new());
    }
    if base.chi == payload.new_chi {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Buckling reduction factor chi is already {}.", payload.new_chi));
    }
    protocol::MutationOutcome::new(En1999Diff { chi: Some(payload.new_chi.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
