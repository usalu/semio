//! 🔺️ Sparse diff builder for `DeleteSolid`.
use super::DeleteSolid;
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dSolidsDelta};
use crate::artifacts::fem3d::mutations::{solid_referrers, target_referenced};
use crate::artifacts::fem3d::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    if !base.solids.iter().any(|solid| solid.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solid \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let referrers = solid_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Solid", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
