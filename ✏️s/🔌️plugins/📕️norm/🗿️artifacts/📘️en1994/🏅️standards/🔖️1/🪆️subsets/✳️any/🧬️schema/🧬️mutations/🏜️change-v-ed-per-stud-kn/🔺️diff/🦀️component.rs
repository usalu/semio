//! 🔺️ `change-v-ed-per-stud-kn` — sparse diff construction.

use super::mutation::ChangeVEdPerStudKn;
use crate::artifacts::en1994::{En1994Diff, En1994Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeVEdPerStudKn, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_v_ed_per_stud_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "V ed per stud kn must be a finite number.", Vec::<String>::new());
    }
    if base.v_ed_per_stud_kn == payload.new_v_ed_per_stud_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "V ed per stud kn already has this value.");
    }
    protocol::MutationOutcome::new(En1994Diff { v_ed_per_stud_kn: Some(payload.new_v_ed_per_stud_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
