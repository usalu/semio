//! 🔺️ Sparse diff builder for `DeleteSection`.
use super::DeleteSection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSectionRemoval;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSectionsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::section_referrers;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_referenced;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &DeleteSection, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(at) = base.sections.iter().position(|section| section.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let referrers = section_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Section", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { removed: vec![Fem3dSectionRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
