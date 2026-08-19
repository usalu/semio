//! 🔺️ `change-p-kn` sparse diff construction — writes only `En1992Diff.p_kn` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_p_kn::mutation::ChangePKn;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangePKn, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_p_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "P kn must be a finite number.", Vec::<String>::new());
    }
    if base.p_kn == payload.new_p_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "P kn already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { p_kn: Some(payload.new_p_kn.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
