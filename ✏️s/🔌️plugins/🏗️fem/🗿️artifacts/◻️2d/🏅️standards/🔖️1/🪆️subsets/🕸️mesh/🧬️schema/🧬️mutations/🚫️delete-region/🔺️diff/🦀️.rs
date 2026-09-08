//! 🔺️ Sparse diff builder for `DeleteRegion`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error), then
//! `mutation.target-referenced` (Error) while any load case still carries an area pressure over
//! this region.
use super::DeleteRegion;
use crate::diff::{Fem2dDiff, Fem2dRegionsDelta};
use crate::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteRegion, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.regions.iter().any(|region| region.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some(rejection) = guards::referenced("Region", "area load", &payload.id, guards::region_referrers(base, &payload.id)) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
