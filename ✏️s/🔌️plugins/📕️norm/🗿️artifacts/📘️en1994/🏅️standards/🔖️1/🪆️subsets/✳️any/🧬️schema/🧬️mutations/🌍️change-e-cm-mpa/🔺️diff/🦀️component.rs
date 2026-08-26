//! 🔺️ `change-e-cm-mpa` — sparse diff construction.

use super::mutation::ChangeECmMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeECmMpa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_e_cm_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "E cm mpa must be a finite number.", Vec::<String>::new());
    }
    if base.e_cm_mpa == payload.new_e_cm_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "E cm mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { e_cm_mpa: Some(payload.new_e_cm_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
