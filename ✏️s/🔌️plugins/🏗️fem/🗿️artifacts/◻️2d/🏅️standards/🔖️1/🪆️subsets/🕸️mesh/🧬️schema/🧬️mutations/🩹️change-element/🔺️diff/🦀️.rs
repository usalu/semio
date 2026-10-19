//! 🔺️ Validates supplied element fields on the current replay base.
use super::ChangeElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementsDelta;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dElementsModification;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::guards;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

pub fn diff(payload: &ChangeElement, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<Fem2dDiff> {
    let Some(existing) = base.elements.iter().find(|row| semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id(row) == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let candidate = match payload.patch.candidate(existing) { Ok(value) => value, Err(error) => return protocol::MutationOutcome::fatal("mutation.invariant", error.message, error.target) };
    if let Some(rejection) = guards::element_references(base, &candidate) { return rejection; }
    if candidate == *existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Element \"{}\" already has the supplied properties.", payload.id));
    }
    protocol::MutationOutcome::new(Fem2dDiff { elements: Some(Fem2dElementsDelta { modified: vec![Fem2dElementsModification { id: payload.id.clone(), patch: payload.patch.clone() }], ..Default::default() }), ..Default::default() })
}
