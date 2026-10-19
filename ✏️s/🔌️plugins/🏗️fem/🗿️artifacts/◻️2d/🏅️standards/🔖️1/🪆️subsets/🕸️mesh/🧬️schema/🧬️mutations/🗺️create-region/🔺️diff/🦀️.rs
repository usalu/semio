//! 🔺️ Sparse diff builder for `CreateRegion`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), the `material_id` resolution
//! (`mutation.target-missing`, Error), then the shared `guards::region_geometry` meshability bounds
//! (`mutation.invariant`, Fatal) — outline arity and area, thickness, mesh size, hole containment.
use super::CreateRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionInsertion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateRegion, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    if let Some(rejection) = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards::scoped_identity(&["region", &payload.region.id]) { return rejection; }
    if base.regions.iter().any(|region| region.id == payload.region.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A region with id \"{}\" already exists.", payload.region.id), [payload.region.id.clone()]);
    }
    if let Some(rejection) = guards::material_reference(base, &payload.region.material_id) {
        return rejection;
    }
    if let Some(rejection) = guards::region_geometry(&payload.region) {
        return rejection;
    }
    if payload.index.is_some_and(|at| at > base.regions.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.regions.len()), [&payload.region.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { inserted: vec![Fem2dRegionInsertion { index: payload.index.unwrap_or(base.regions.len()), row: payload.region.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
