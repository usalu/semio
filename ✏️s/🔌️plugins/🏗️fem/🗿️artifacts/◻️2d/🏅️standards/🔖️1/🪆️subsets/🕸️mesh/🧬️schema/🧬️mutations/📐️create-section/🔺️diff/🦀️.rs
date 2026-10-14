//! 🔺️ Sparse diff builder for `CreateSection`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::section_plausibility` positivity bounds (`mutation.invariant`, Fatal).
use super::CreateSection;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSectionInsertion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSectionsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateSection, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    if base.sections.iter().any(|section| section.id == payload.section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A section with id \"{}\" already exists.", payload.section.id), [payload.section.id.clone()]);
    }
    if let Some(rejection) = guards::section_plausibility(&payload.section) {
        return rejection;
    }
    if payload.index.is_some_and(|at| at > base.sections.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.sections.len()), [&payload.section.id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { sections: Some(Fem2dSectionsDelta { inserted: vec![Fem2dSectionInsertion { index: payload.index.unwrap_or(base.sections.len()), row: payload.section.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
