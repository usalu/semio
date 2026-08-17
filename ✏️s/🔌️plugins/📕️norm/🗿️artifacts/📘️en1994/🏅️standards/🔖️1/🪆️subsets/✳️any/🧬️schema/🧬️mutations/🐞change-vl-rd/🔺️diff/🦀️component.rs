//! 🔺️ `change-vl-rd` — sparse diff construction.

use super::mutation::ChangeVLRd;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeVLRd, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_v_l_rd.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Vl rd must be a finite number.", Vec::<String>::new());
    }
    if base.v_l_rd == payload.new_v_l_rd {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Vl rd already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { v_l_rd: Some(payload.new_v_l_rd.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
