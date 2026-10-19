//! 🔺️ Validates supplied section fields on the current replay base.
use super::ChangeSection;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSectionsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSectionsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

pub fn diff(payload: &ChangeSection, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.sections.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = payload.patch.candidate(existing);
    if let Some(rejection) = guards::section_plausibility(&candidate) { return rejection; }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Section \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { sections: Some(Fem2dSectionsDelta { modified: vec![Fem2dSectionsModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() }), ..Default::default() })
}
