//! 🔺️ `change-delta-tau-stud-mpa` — sparse diff construction.

use super::ChangeDeltaTauStudMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaTauStudMpa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_delta_tau_stud_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Delta tau stud mpa must be a finite number.", Vec::<String>::new());
    }
    if base.delta_tau_stud_mpa == payload.new_delta_tau_stud_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Delta tau stud mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { delta_tau_stud_mpa: Some(payload.new_delta_tau_stud_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
