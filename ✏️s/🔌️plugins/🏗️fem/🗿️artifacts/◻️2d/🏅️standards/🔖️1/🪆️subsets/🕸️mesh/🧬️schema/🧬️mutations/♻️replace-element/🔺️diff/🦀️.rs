//! 🔺️ Sparse diff builder for `ReplaceElement`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error) on the selected id,
//! `mutation.target-mismatch` (Error) when the replacement renames it, the SAME four foreign-key
//! resolutions `create-element` runs (`mutation.target-missing`, Error), and finally the
//! `mutation.no-op` warning when the replacement equals what is already there.
use super::ReplaceElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceElement, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.elements.iter().find(|element| element_id(element) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::identity_matches("element", &payload.id, element_id(&payload.new_element)) {
        return rejection;
    }
    if let Some(rejection) = guards::element_references(base, &payload.new_element) {
        return rejection;
    }
    if existing == payload.new_element.as_ref() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Element \"{}\" is already equal to the replacement value.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { elements: Some(Fem2dElementsDelta { modified: vec![Fem2dElementsModification { id: payload.id.clone(), patch: payload.new_element.clone() }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
