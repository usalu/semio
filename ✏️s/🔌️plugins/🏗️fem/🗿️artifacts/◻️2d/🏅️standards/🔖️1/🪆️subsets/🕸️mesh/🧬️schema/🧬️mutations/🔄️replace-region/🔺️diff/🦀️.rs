//! 🔺️ Sparse diff builder for `ReplaceRegion`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.target-mismatch` (Error) when the replacement renames it, the SAME `material_id`
//! resolution and meshability bounds `create-region` runs, and finally `mutation.no-op`.
use super::ReplaceRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

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
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Region \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { modified: vec![Fem2dRegionsModification { id: payload.id.clone(), patch: payload.new_region.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
