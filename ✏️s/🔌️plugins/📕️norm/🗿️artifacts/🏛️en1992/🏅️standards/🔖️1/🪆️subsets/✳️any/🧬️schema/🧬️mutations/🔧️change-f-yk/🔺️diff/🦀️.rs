//! 🔺️ `change-f-yk` sparse diff construction — writes only `En1992Diff.f_yk` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_f_yk::ChangeFYk;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFYk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_f_yk.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "F yk must be a finite number.", Vec::<String>::new());
    }
    if base.f_yk == payload.new_f_yk {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "F yk already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { f_yk: Some(payload.new_f_yk), ..Default::default() })
}
//#endregion 🔖️Diff
