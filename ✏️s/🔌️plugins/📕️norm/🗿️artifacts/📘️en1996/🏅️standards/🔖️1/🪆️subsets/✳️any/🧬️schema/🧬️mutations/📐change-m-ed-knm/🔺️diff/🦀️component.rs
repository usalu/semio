//! 🔺️ `change-m-ed-knm` sparse diff construction — writes only `En1996Diff.m_ed_knm` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMEdKnm, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_m_ed_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "M ed knm must be a finite number.", Vec::<String>::new());
    }
    if base.m_ed_knm == payload.new_m_ed_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "M ed knm already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { m_ed_knm: Some(payload.new_m_ed_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
