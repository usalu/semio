//! 🔺️ Diff constructor for `SetPropertyTemplate`: a sparse template patch of exactly the provided fields that differ from the base. The template that results must pass the same checks as a created one (a name no other
//! template has, kinds listed at most once, sound and uniquely named definitions); providing only equal values is a no-op. Elements are not touched: their effective properties follow by inference.

use super::SetPropertyTemplate;
use crate::{template_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetPropertyTemplate, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.property_templates.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Property template \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch();
    let name = patch.name.as_deref().unwrap_or(&record.name);
    let applies_to = patch.applies_to.as_deref().unwrap_or(&record.applies_to);
    let properties = patch.properties.as_deref().unwrap_or(&record.properties);
    if let Some((field, message)) = template_problem(name, applies_to, properties) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
    }
    if base.property_templates.iter().any(|(other, row)| *other != payload.id && row.name == name) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("A property set template named \"{name}\" already exists."), ["name"]);
    }
    let minimal = patch.minimal(record);
    if minimal.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Property template \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::property_templates(payload.id.clone(), Entry::Patched(minimal)))
}
