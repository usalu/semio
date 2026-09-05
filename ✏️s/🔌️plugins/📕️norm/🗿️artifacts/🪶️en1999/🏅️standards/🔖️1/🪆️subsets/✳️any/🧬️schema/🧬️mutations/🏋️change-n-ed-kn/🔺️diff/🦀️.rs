//! 🔺️ `change-n-ed-kn` sparse diff construction — writes only `En1999Diff.n_ed_kn` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_n_ed_kn::ChangeNEdKn;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNEdKn, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_n_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Design axial force N_Ed [kN] must be a finite number, got {}.", payload.new_n_ed_kn), Vec::<String>::new());
    }
    if base.n_ed_kn == payload.new_n_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Design axial force N_Ed [kN] is already {}.", payload.new_n_ed_kn));
    }
    protocol::MutationOutcome::new(En1999Diff { n_ed_kn: Some(payload.new_n_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
