//! 🔺️ Diff constructor for `SetFamilySolid`: a sparse solid patch of exactly the provided fields that differ. The solid that results must break none of the create rules (its family cannot change).
//! Providing only equal values, or no field, is a no-op.

use crate::standards::v1::subsets::any::schema::mutations::family_rules;
use crate::standards::v1::subsets::any::schema::mutations::set_family_solid::SetFamilySolid;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff, Patch}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetFamilySolid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.family_solids.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family solid \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = family_rules::solid_fault(base, &next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Family solid \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::family_solids(payload.id.clone(), Entry::Patched(change)))
}
