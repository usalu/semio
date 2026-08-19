//! 🔺️ `change-f-ed-kn` sparse diff construction — writes only `En1995Diff.f_ed_kn` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_f_ed_kn::mutation::ChangeFEdKn;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFEdKn, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_f_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "F ed kn must be a finite number.", Vec::<String>::new());
    }
    if base.f_ed_kn == payload.new_f_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "F ed kn already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { f_ed_kn: Some(payload.new_f_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
