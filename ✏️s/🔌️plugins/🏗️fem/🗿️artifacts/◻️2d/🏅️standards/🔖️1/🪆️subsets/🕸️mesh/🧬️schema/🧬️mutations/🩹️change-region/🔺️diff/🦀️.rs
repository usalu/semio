//! 🔺️ Validates supplied region fields on the current replay base.
use super::ChangeRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dRegionsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

pub fn diff(payload: &ChangeRegion, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.regions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Region \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = payload.patch.candidate(existing);
    if let Some(rejection) = guards::material_reference(base, &candidate.material_id) { return rejection; }
    if let Some(rejection) = guards::region_geometry(&candidate) { return rejection; }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Region \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { regions: Some(Fem2dRegionsDelta { modified: vec![Fem2dRegionsModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() }), ..Default::default() })
}
