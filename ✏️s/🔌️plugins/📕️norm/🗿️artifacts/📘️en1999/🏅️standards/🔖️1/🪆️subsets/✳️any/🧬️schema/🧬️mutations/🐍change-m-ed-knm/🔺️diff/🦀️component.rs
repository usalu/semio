//! 🔺️ `change-m-ed-knm` sparse diff construction — writes only `En1999Diff.m_ed_knm` from the payload.

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::change_m_ed_knm::mutation::ChangeMEdKnm;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeMEdKnm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_m_ed_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Design bending moment M_Ed [kNm] must be a finite number, got {}.", payload.new_m_ed_knm), Vec::<String>::new());
    }
    if base.m_ed_knm == payload.new_m_ed_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Design bending moment M_Ed [kNm] is already {}.", payload.new_m_ed_knm));
    }
    protocol::MutationOutcome::new(En1999Diff { m_ed_knm: Some(payload.new_m_ed_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
