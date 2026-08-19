//! 🔺️ `change-m-pl-rd` — sparse diff construction.

use super::mutation::ChangeMPlRd;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMPlRd, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_m_pl_rd.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "M pl rd must be a finite number.", Vec::<String>::new());
    }
    if base.m_pl_rd == payload.new_m_pl_rd {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "M pl rd already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { m_pl_rd: Some(payload.new_m_pl_rd.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
