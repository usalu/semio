//! 🔺️ Sparse diff builder for `CreateElement`.
//!
//! Guards, in the order they run: `mutation.duplicate-id` (Fatal), then the shared
//! `guards::element_references` resolution of all four foreign keys — `start`, `end`,
//! `material_id`, `section_id` (`mutation.target-missing`, Error). `replace-element` calls the
//! SAME guard, so the twins cannot drift apart.
use super::CreateElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementInsertion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &CreateElement, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    if let Some(rejection) = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards::scoped_identity(&["element", semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id(&payload.element)]) { return rejection; }
    let new_id = element_id(&payload.element);
    if base.elements.iter().any(|element| element_id(element) == new_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An element with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    if let Some(rejection) = guards::element_references(base, &payload.element) {
        return rejection;
    }
    if payload.index.is_some_and(|at| at > base.elements.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), base.elements.len()), [new_id.to_string()]);
    }
    protocol::MutationOutcome::new(Fem2dDiff { elements: Some(Fem2dElementsDelta { inserted: vec![Fem2dElementInsertion { index: payload.index.unwrap_or(base.elements.len()), row: (*payload.element).clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
