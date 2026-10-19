//! 🔺️ Sparse diff builder for `ReplaceSection`.
use super::ReplaceSection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSectionPatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSectionsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dSectionsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_mismatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::invariant;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::section_breach;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSection, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.sections.iter().find(|section| section.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_section.id != payload.id {
        return target_mismatch("Section", &payload.id, &payload.new_section.id);
    }
    if existing == &payload.new_section {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Section \"{}\" already has that value.", payload.id));
    }
    if let Some(breach) = section_breach(&payload.new_section) {
        return invariant(breach, vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Fem3dDiff { sections: Some(Fem3dSectionsDelta { modified: vec![Fem3dSectionsModification { id: payload.id.clone(), patch: Fem3dSectionPatch::replacement(&payload.new_section) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
