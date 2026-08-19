//! 🔺️ `change-v-ed-kn` sparse diff construction — writes only `En1992Diff.v_ed_kn` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_v_ed_kn::mutation::ChangeVEdKn;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeVEdKn, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_v_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "V ed kn must be a finite number.", Vec::<String>::new());
    }
    if base.v_ed_kn == payload.new_v_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "V ed kn already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { v_ed_kn: Some(payload.new_v_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
