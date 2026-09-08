//! 🔺️ `change-fu-mpa` — sparse diff construction.

use super::ChangeFUMpa;
use crate::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFUMpa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_f_u_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fu mpa must be a finite number.", Vec::<String>::new());
    }
    if base.f_u_mpa == payload.new_f_u_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fu mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { f_u_mpa: Some(payload.new_f_u_mpa), ..Default::default() })
}
//#endregion 🔖️Diff
