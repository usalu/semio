//! 🔺️ `change-m-ed-knm` — sparse diff construction.

use super::mutation::ChangeMEdKnm;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMEdKnm, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_m_ed_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "M ed knm must be a finite number.", Vec::<String>::new());
    }
    if base.m_ed_knm == payload.new_m_ed_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "M ed knm already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { m_ed_knm: Some(payload.new_m_ed_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
