//! 🔺️ `change-m-ed-knm` sparse diff construction — writes only `En1992Diff.m_ed_knm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMEdKnm, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_m_ed_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "M ed knm must be a finite number.", Vec::<String>::new());
    }
    if base.m_ed_knm == payload.new_m_ed_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "M ed knm already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { m_ed_knm: Some(payload.new_m_ed_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
