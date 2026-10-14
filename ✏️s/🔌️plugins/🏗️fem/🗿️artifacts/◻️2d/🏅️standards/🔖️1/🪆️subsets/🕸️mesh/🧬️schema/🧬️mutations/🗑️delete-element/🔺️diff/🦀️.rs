//! 🔺️ Sparse diff builder for `DeleteElement`.
//!
//! Guards, in the order they run: `mutation.target-missing` (Error), then
//! `mutation.target-referenced` (Error) while any load case still carries a member UDL naming this
//! element — deleting it would leave that UDL pointing at nothing.
use super::DeleteElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementRemoval;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteElement, base: &Fem2dSnapshot) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(at) = base.elements.iter().position(|element| element_id(element) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(rejection) = guards::referenced("Element", "member UDL", &payload.id, guards::element_referrers(base, &payload.id)) {
        return rejection;
    }
    protocol::MutationOutcome::new(Fem2dDiff { elements: Some(Fem2dElementsDelta { removed: vec![Fem2dElementRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
