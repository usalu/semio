//! 🔺️ `change-f-m-k` sparse diff construction — writes only `En1995Diff.f_m_k` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_f_m_k::ChangeFMK;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeFMK, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_f_m_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Fmk must be a finite number.", Vec::<String>::new());
    }
    if base.f_m_k == payload.new_f_m_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Fmk already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { f_m_k: Some(payload.new_f_m_k.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
