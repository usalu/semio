//! 🔺️ `change-m-pla` — sparse diff construction.

use super::mutation::ChangeMPla;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMPla, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_m_pla.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "M pla must be a finite number.", Vec::<String>::new());
    }
    if base.m_pla == payload.new_m_pla {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "M pla already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { m_pla: Some(payload.new_m_pla.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
