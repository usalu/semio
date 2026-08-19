//! 🔺️ Sparse diff builder for `DeleteRegion`.
use super::mutation::DeleteRegion;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dRegionsDelta};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteRegion, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if !base.regions.iter().any(|region| region.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
