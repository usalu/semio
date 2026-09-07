//! 🔺️ Sparse diff builder for `ReplaceRegion`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.id-mismatch` (Fatal) when the replacement renames it, the SAME `material_id`
//! resolution and meshability bounds `create-region` runs, and finally `mutation.no-op`.
use super::ReplaceRegion;
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dRegionsDelta, Fem2dRegionsPatchEntry};
use crate::artifacts::fem2d::mutations::guards;
use crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceRegion, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.regions.iter().find(|region| region.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("region", &payload.id, &payload.new_region.id) {
        return rejection;
    }
    if let Some(rejection) = guards::material_reference(base, &payload.new_region.material_id) {
        return rejection;
    }
    if let Some(rejection) = guards::region_geometry(&payload.new_region) {
        return rejection;
    }
    if *existing == payload.new_region {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Region \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { patched: vec![Fem2dRegionsPatchEntry { id: payload.id.clone(), item: payload.new_region.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
