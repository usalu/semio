//! 🔺️ Diff constructor for `CreatePropertyTemplate`: one created template entry. The id must be free in every collection, the template needs a name no other template has, kinds it lists at most once and sound,
//! uniquely named property definitions (a default and every allowed value of the kind of the property and within its own range). Which elements the template applies to is not checked here: it is inferred.

use super::super::elements;
use super::CreatePropertyTemplate;
use crate::{template_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreatePropertyTemplate, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let template = &payload.template;
    if let Some((field, message)) = template_problem(&template.name, &template.applies_to, &template.properties) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["template", field]);
    }
    if base.property_templates.values().any(|row| row.name == template.name) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("A property set template named \"{}\" already exists.", template.name), ["template", "name"]);
    }
    MutationOutcome::new(ModelDiff::property_templates(payload.id.clone(), Entry::Created(template.clone())))
}
