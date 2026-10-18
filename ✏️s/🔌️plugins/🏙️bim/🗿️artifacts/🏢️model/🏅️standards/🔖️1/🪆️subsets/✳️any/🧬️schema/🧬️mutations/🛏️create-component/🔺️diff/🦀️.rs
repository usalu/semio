//! 🔺️ Diff constructor for `CreateComponent`: one created component entry. The id is free in every collection, the storey exists, the family exists and is no profile, every number is finite and a host is a
//! wall of the same storey. The solids, the fit onto the host, the volume and the issues of the instance are inferred, never stored.

use crate::standards::v1::subsets::any::schema::mutations::component_rules;
use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::create_component::CreateComponent;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateComponent, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = component_rules::component_fault(base, &payload.component) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("component")));
    }
    MutationOutcome::new(ModelDiff::components(payload.id.clone(), Entry::Created(payload.component.clone())))
}
