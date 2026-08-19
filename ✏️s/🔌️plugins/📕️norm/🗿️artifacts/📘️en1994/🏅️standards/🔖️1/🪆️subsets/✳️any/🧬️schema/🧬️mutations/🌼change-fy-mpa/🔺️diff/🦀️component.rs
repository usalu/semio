//! 🔺️ `change-fy-mpa` — sparse diff construction.

use super::mutation::ChangeFYMpa;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFYMpa, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_f_y_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fy mpa must be a finite number.", Vec::<String>::new());
    }
    if base.f_y_mpa == payload.new_f_y_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fy mpa already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { f_y_mpa: Some(payload.new_f_y_mpa.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
