//! 🔺️ `change-t1-s` sparse diff construction — writes only `En1998Diff.t1_s` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_t1_s::mutation::ChangeT1S;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeT1S, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_t1_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fundamental period T1 [s] must be a finite number, got {}.", payload.new_t1_s), Vec::<String>::new());
    }
    if base.t1_s == payload.new_t1_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fundamental period T1 [s] is already {}.", payload.new_t1_s));
    }
    protocol::MutationOutcome::new(En1998Diff { t1_s: Some(payload.new_t1_s.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
