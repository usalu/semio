//! 🔺️ `change-p-kn` sparse diff construction — writes only `En1992Diff.p_kn` from the payload.

use crate::diff::En1992Diff;
use crate::mutations::change_p_kn::ChangePKn;
use crate::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePKn, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_p_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "P kn must be a finite number.", Vec::<String>::new());
    }
    if base.p_kn == payload.new_p_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "P kn already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { p_kn: Some(payload.new_p_kn), ..Default::default() })
}
//#endregion 🔖️Diff
