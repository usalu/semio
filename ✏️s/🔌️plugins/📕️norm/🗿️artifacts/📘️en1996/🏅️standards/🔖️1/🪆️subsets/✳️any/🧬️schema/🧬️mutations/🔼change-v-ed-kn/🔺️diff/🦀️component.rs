//! 🔺️ `change-v-ed-kn` sparse diff construction — writes only `En1996Diff.v_ed_kn` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_v_ed_kn::mutation::ChangeVEdKn;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeVEdKn, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_v_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "V ed kn must be a finite number.", Vec::<String>::new());
    }
    if base.v_ed_kn == payload.new_v_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "V ed kn already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { v_ed_kn: Some(payload.new_v_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
