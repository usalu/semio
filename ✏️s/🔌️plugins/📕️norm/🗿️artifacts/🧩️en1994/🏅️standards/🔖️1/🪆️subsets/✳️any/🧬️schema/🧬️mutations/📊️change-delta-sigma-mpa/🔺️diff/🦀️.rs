//! 🔺️ `change-delta-sigma-mpa` — sparse diff construction.

use super::ChangeDeltaSigmaMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeltaSigmaMpa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_delta_sigma_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Delta sigma mpa must be a finite number.", Vec::<String>::new());
    }
    if base.delta_sigma_mpa == payload.new_delta_sigma_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Delta sigma mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { delta_sigma_mpa: Some(payload.new_delta_sigma_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
