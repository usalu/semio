//! 🔺️ Diff constructor for `SetAreaScheme`: a sparse area scheme patch of exactly the provided fields that differ from the base. Counted zones must
//! exist and counted usages must not be blank (the rule of `create-area-scheme`); providing only equal values is a no-op.

use super::super::elements;
use super::SetAreaScheme;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetAreaScheme, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(scheme) = base.area_schemes.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Area scheme \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some((code, message, field)) = elements::scheme_rule_issue(base, payload.usages.as_deref().unwrap_or_default(), payload.zones.as_deref().unwrap_or_default()) {
        return MutationOutcome::refuse(code, message, [field]);
    }
    let patch = payload.patch().minimal(scheme);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Area scheme \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::area_schemes(payload.id.clone(), Entry::Patched(patch)))
}
