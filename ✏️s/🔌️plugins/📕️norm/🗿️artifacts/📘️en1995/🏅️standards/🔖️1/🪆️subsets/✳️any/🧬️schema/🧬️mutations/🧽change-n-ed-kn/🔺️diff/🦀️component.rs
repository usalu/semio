//! 🔺️ `change-n-ed-kn` sparse diff construction — writes only `En1995Diff.n_ed_kn` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_n_ed_kn::mutation::ChangeNEdKn;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNEdKn, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_n_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "N ed kn must be a finite number.", Vec::<String>::new());
    }
    if base.n_ed_kn == payload.new_n_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "N ed kn already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { n_ed_kn: Some(payload.new_n_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
