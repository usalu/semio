//! 🔺️ Sparse diff builder for `CreateRegion`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), the `material_id` resolution
//! (`mutation.target-missing`, Error), then the shared `guards::region_geometry` meshability bounds
//! (`mutation.invariant`, Fatal) — outline arity and area, thickness, mesh size, hole containment.
use super::CreateRegion;
use crate::standards::v1::subsets::any::schema::diff::{Fem2dDiff, Fem2dRegionsDelta};
use crate::standards::v1::subsets::any::schema::mutations::guards;
use crate::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateRegion, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.regions.iter().any(|region| region.id == payload.region.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A region with id \"{}\" already exists.", payload.region.id), [payload.region.id.clone()]);
    }
    if let Some(rejection) = guards::material_reference(base, &payload.region.material_id) {
        return rejection;
    }
    if let Some(rejection) = guards::region_geometry(&payload.region) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { added: vec![payload.region.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
