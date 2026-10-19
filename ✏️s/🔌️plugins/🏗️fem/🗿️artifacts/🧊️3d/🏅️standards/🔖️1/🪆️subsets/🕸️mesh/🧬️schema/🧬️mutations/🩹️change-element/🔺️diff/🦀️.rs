//! 🔺️ Validates supplied element fields on the current replay base.
use super::ChangeElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dElementsDelta;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dElementsModification;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::guards::resolve_element;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

pub fn diff(payload: &ChangeElement, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<Fem3dDiff> {
    let Some(existing) = base.elements.iter().find(|row| semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id(row) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = match payload.patch.candidate(existing) { Ok(value) => value, Err(error) => return protocol::MutationOutcome::fatal("mutation.invariant", error.message, error.target) };
    if let Some(rejection) = resolve_element(base, &candidate) { return rejection; }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Element \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem3dDiff { elements: Some(Fem3dElementsDelta { modified: vec![Fem3dElementsModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() }), ..Default::default() })
}
