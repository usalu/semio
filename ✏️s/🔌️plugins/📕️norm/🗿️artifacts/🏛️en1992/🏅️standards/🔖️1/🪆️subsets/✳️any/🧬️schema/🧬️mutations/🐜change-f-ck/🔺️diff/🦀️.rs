//! 🔺️ `change-f-ck` sparse diff construction — writes only `En1992Diff.f_ck` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_f_ck::ChangeFCk;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFCk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_f_ck.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "F ck must be a finite number.", Vec::<String>::new());
    }
    if base.f_ck == payload.new_f_ck {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "F ck already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { f_ck: Some(payload.new_f_ck.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
