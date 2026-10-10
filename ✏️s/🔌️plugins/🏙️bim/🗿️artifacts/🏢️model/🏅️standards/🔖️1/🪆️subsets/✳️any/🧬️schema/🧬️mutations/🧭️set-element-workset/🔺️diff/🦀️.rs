//! 🔺️ Sparse declarative set-element-workset diff.
use super::SetElementWorkset;
use crate::*;
use super::super::{elements, option_rules};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetElementWorkset, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !option_rules::element_ids(base).contains(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Physical element is missing.", [payload.id.clone()]); }
    if payload.workset.as_ref().is_some_and(|id| !base.worksets.contains_key(id)) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Membership target is missing.", [payload.id.clone()]); }
    let old = base.element_worksets.get(&payload.id);
    if old.map(|entry| &entry.target) == payload.workset.as_ref() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Membership is unchanged.", [payload.id.clone()]); }
    let entry = match (&payload.workset, old) { (None, _) => Entry::Deleted, (Some(target), None) => Entry::Created(ElementMembership { target: target.clone() }), (Some(target), Some(_)) => Entry::Patched(ElementMembershipPatch { target: Some(target.clone()) }) };
    MutationOutcome::new(ModelDiff::element_worksets(payload.id.clone(), entry))
}
