//! 🔺️ Sparse diff builder for `ReplaceElement`.
use super::ReplaceElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dElementsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dElementsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_mismatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::resolve_element;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceElement, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.elements.iter().find(|element| element_id(element) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let replacement = element_id(&payload.new_element);
    if replacement != payload.id {
        return target_mismatch("Element", &payload.id, replacement);
    }
    if existing == payload.new_element.as_ref() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Element \"{}\" already has that value.", payload.id));
    }
    if let Some(refusal) = resolve_element(base, &payload.new_element) {
        return refusal;
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { modified: vec![Fem3dElementsModification { id: payload.id.clone(), patch: semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dElementPatch::replacement(&payload.new_element) }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
