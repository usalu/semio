//! 🔺️ Diff constructor for `SetFamilyParameter`: the parameter `name` of a family is one record keyed `family.name`. An existing parameter gets a sparse patch of exactly the provided kind and formula
//! that differ (providing nothing new is a no-op); an absent one is created and needs both. A formula must parse, use only existing parameters of the family and never close a circle; whether it also
//! computes the declared kind is the business of the inference (a diagnostic), not of the diff. The key is free in every other collection.

use super::super::elements;
use super::super::family_rules;
use super::SetFamilyParameter;
use crate::standards::v1::subsets::any::schema::inferences::families::formula;
use crate::{Entry, FamilyParameter, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetFamilyParameter, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.families.contains_key(&payload.family) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \"{}\" does not exist.", payload.family), ["family"]);
    }
    if let Some(fault) = family_rules::name_fault(&payload.name) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    if let Some(text) = &payload.value {
        let fault = family_rules::formula_fault(text, "value").or_else(|| family_rules::reference_fault(base, &payload.family, text, "value")).or_else(|| family_rules::cycle_fault(base, &payload.family, &payload.name, text));
        if let Some(fault) = fault {
            return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
        }
    }
    let key = formula::parameter_id(&payload.family, &payload.name);
    if let Some(record) = base.family_parameters.get(&key) {
        let change = payload.patch().minimal(record);
        if change.is_empty() {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Parameter \"{}\" of family \"{}\" already has these values.", payload.name, payload.family), [key]);
        }
        return MutationOutcome::new(ModelDiff::family_parameters(key, Entry::Patched(change)));
    }
    if let Some(noun) = elements::taken(base, &key) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{key}\" already exists."), [key]);
    }
    let Some(kind) = payload.kind else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A new parameter needs a kind.", ["kind"]);
    };
    let Some(value) = payload.value.clone() else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A new parameter needs a formula.", ["value"]);
    };
    MutationOutcome::new(ModelDiff::family_parameters(key, Entry::Created(FamilyParameter { family: payload.family.clone(), name: payload.name.clone(), kind, value })))
}
