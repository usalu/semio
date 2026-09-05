//! 🔺️ `change-h-ed-kn` sparse diff construction — writes only `En1997Diff.h_ed_kn` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_h_ed_kn::ChangeHEdKn;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHEdKn, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_h_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Design horizontal load H_Ed [kN] must be a finite number, got {}.", payload.new_h_ed_kn), Vec::<String>::new());
    }
    if base.h_ed_kn == payload.new_h_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Design horizontal load H_Ed [kN] is already {}.", payload.new_h_ed_kn));
    }
    protocol::MutationOutcome::new(En1997Diff { h_ed_kn: Some(payload.new_h_ed_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
