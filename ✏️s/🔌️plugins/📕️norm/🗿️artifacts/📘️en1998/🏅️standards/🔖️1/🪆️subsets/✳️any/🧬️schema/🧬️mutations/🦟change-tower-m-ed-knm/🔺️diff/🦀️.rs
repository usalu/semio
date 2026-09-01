//! 🔺️ `change-tower-m-ed-knm` sparse diff construction — writes only `En1998Diff.tower_m_ed_knm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tower_m_ed_knm::ChangeTowerMEdKnm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTowerMEdKnm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tower_m_ed_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tower design moment M_Ed [kNm] must be a finite number, got {}.", payload.new_tower_m_ed_knm), Vec::<String>::new());
    }
    if base.tower_m_ed_knm == payload.new_tower_m_ed_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tower design moment M_Ed [kNm] is already {}.", payload.new_tower_m_ed_knm));
    }
    protocol::MutationOutcome::new(En1998Diff { tower_m_ed_knm: Some(payload.new_tower_m_ed_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
