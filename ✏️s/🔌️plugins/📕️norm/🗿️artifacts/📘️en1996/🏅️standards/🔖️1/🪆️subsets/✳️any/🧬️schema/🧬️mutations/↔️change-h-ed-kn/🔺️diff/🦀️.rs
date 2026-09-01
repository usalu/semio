//! 🔺️ `change-h-ed-kn` sparse diff construction — writes only `En1996Diff.h_ed_kn` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_h_ed_kn::ChangeHEdKn;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHEdKn, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_h_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "H ed kn must be a finite number.", Vec::<String>::new());
    }
    if base.h_ed_kn == payload.new_h_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "H ed kn already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { h_ed_kn: Some(payload.new_h_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
