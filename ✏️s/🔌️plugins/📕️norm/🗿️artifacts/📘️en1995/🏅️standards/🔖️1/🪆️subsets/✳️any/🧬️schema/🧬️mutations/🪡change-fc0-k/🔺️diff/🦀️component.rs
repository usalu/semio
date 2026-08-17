//! 🔺️ `change-f-c-0-k` sparse diff construction — writes only `En1995Diff.f_c_0_k` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_f_c_0_k::mutation::ChangeFC0K;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFC0K, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_f_c_0_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fc0 k must be a finite number.", Vec::<String>::new());
    }
    if base.f_c_0_k == payload.new_f_c_0_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fc0 k already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { f_c_0_k: Some(payload.new_f_c_0_k.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
