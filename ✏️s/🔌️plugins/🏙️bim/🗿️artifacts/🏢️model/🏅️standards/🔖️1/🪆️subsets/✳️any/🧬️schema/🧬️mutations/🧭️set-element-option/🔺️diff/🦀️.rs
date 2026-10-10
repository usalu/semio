//! 🔺️ Sparse declarative set-element-option diff.
use super::SetElementOption;
use crate::*;
use super::super::{elements, option_rules};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetElementOption, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !option_rules::element_ids(base).contains(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Physical element is missing.", [payload.id.clone()]); }
    if payload.option.as_ref().is_some_and(|id| !base.design_options.contains_key(id)) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Membership target is missing.", [payload.id.clone()]); }
    let old = base.element_options.get(&payload.id);
    if old.map(|entry| &entry.target) == payload.option.as_ref() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Membership is unchanged.", [payload.id.clone()]); }
    let entry = match (&payload.option, old) { (None, _) => Entry::Deleted, (Some(target), None) => Entry::Created(ElementMembership { target: target.clone() }), (Some(target), Some(_)) => Entry::Patched(ElementMembershipPatch { target: Some(target.clone()) }) };
    MutationOutcome::new(ModelDiff::element_options(payload.id.clone(), entry))
}
