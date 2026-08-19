//! 🔺️ `change-m-crit-knm` sparse diff construction — writes only `En1995Diff.m_crit_knm` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_m_crit_knm::mutation::ChangeMCritKnm;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMCritKnm, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if !payload.new_m_crit_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "M crit knm must be a finite number.", Vec::<String>::new());
    }
    if base.m_crit_knm == payload.new_m_crit_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "M crit knm already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { m_crit_knm: Some(payload.new_m_crit_knm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
