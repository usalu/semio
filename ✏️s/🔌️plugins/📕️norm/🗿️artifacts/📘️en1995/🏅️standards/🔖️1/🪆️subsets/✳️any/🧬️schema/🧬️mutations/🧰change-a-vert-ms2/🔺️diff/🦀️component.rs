//! 🔺️ `change-a-vert-m-s2` sparse diff construction — writes only `En1995Diff.a_vert_m_s2` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_a_vert_m_s2::mutation::ChangeAVertMS2;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAVertMS2, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_a_vert_m_s2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A vert ms2 must be a finite number.", Vec::<String>::new());
    }
    if base.a_vert_m_s2 == payload.new_a_vert_m_s2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "A vert ms2 already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { a_vert_m_s2: Some(payload.new_a_vert_m_s2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
