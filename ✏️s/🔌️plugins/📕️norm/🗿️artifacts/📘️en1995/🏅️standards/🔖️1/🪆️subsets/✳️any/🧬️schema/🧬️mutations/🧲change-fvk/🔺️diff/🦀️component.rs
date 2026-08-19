//! 🔺️ `change-f-v-k` sparse diff construction — writes only `En1995Diff.f_v_k` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_f_v_k::mutation::ChangeFVK;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeFVK, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_f_v_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fvk must be a finite number.", Vec::<String>::new());
    }
    if base.f_v_k == payload.new_f_v_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fvk already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { f_v_k: Some(payload.new_f_v_k.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
