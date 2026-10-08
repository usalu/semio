//! 🔺️ Sparse diff builder for `DeleteSolid`.
use super::DeleteSolid;
use crate::standards::v1::subsets::any::schema::diff::{Fem3dDiff, Fem3dSolidRemoval, Fem3dSolidsDelta};
use crate::standards::v1::subsets::any::schema::mutations::{solid_referrers,target_referenced};

use crate::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSolid, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(at) = base.solids.iter().position(|solid| solid.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solid \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let referrers = solid_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Solid", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { solids: Some(Fem3dSolidsDelta { removed: vec![Fem3dSolidRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
