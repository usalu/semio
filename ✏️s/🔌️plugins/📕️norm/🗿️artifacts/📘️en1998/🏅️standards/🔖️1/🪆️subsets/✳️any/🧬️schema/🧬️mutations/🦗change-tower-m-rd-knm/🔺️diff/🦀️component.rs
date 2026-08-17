//! 🔺️ `change-tower-m-rd-knm` sparse diff construction — writes only `En1998Diff.tower_m_rd_knm` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_tower_m_rd_knm::mutation::ChangeTowerMRdKnm;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeTowerMRdKnm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_tower_m_rd_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tower moment resistance M_Rd [kNm] must be a finite number, got {}.", payload.new_tower_m_rd_knm), Vec::<String>::new());
    }
    if base.tower_m_rd_knm == payload.new_tower_m_rd_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tower moment resistance M_Rd [kNm] is already {}.", payload.new_tower_m_rd_knm));
    }
    protocol::MutationOutcome::new(En1998Diff { tower_m_rd_knm: Some(payload.new_tower_m_rd_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
