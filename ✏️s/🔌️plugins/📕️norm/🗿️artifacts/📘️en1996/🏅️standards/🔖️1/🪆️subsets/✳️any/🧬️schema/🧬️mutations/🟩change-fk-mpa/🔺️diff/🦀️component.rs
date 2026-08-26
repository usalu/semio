//! 🔺️ `change-fk-mpa` sparse diff construction — writes only `En1996Diff.f_k_mpa` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_f_k_mpa::mutation::ChangeFKMpa;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFKMpa, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_f_k_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fk mpa must be a finite number.", Vec::<String>::new());
    }
    if base.f_k_mpa == payload.new_f_k_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fk mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { f_k_mpa: Some(payload.new_f_k_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
