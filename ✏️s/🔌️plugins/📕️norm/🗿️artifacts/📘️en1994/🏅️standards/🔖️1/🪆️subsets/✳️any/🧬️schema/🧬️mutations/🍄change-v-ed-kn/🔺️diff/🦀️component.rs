//! 🔺️ `change-v-ed-kn` — sparse diff construction.

use super::mutation::ChangeVEdKn;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeVEdKn, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_v_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "V ed kn must be a finite number.", Vec::<String>::new());
    }
    if base.v_ed_kn == payload.new_v_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "V ed kn already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { v_ed_kn: Some(payload.new_v_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
