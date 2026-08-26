//! 🔺️ `change-en-vbms` — sparse diff construction.

use super::mutation::ChangeEnVBMS;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnVBMS, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_en_v_b_m_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "En vbms must be a finite number.", Vec::<String>::new());
    }
    if base.en_v_b_m_s == payload.new_en_v_b_m_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "En vbms already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { en_v_b_m_s: Some(payload.new_en_v_b_m_s.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
