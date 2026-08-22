//! 🔺️ Sparse diff builder for `ReplaceRegion`.
use super::mutation::ReplaceRegion;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dRegionsDelta, Fem2dRegionsPatchEntry};
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceRegion, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.regions.iter().find(|region| region.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if *existing == payload.new_region {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Region \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { patched: vec![Fem2dRegionsPatchEntry { id: payload.id.clone(), item: payload.new_region.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
