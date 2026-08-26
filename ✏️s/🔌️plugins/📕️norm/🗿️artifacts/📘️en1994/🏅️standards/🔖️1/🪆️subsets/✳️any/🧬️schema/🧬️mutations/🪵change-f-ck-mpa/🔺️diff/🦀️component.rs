//! 🔺️ `change-f-ck-mpa` — sparse diff construction.

use super::mutation::ChangeFCkMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFCkMpa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_f_ck_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "F ck mpa must be a finite number.", Vec::<String>::new());
    }
    if base.f_ck_mpa == payload.new_f_ck_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "F ck mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { f_ck_mpa: Some(payload.new_f_ck_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
