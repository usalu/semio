//! 🔺️ Sparse diff builder for `DeleteElement`.
use super::DeleteElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dElementsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dElementRemoval;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::element_referrers;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::target_referenced;

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteElement, base: &Fem3dSnapshot) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(at) = base.elements.iter().position(|element| element_id(element) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let referrers = element_referrers(base, &payload.id);
    if !referrers.is_empty() {
        return target_referenced("Element", &payload.id, referrers);
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { removed: vec![Fem3dElementRemoval { id: payload.id.clone(), index: at }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
