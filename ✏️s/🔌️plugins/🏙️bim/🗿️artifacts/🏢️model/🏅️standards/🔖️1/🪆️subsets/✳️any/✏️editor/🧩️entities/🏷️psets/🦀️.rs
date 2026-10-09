//! 🏷️ The rows of the entity table for the property set templates and the classification systems of the library: how they read off the snapshot, how an edited value becomes a `set-property-template` or
//! `set-classification-system` mutation, what a new template or system is, and the kinds a template applies to as text. The definitions of a template and the entry table of a system are edited by the
//! `editTemplate` and `editClassification` commands, row by row; the classification browser assigns codes.

use super::{partial, Created, FieldRow, InferredRow};
use crate::mutations::set_classification_system::SetClassificationSystem;
use crate::{Assigned, ClassificationSystem, ClassificationSystemPatch, ModelMutation, ModelSnapshot, PropertyTemplate, TemplateTarget};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;

//#region 🔖️Targets
/// 🎯️ The kinds a template applies to as comma separated kebab-case names.
pub fn targets_text(targets: &[TemplateTarget]) -> String {
    targets.iter().map(|target| target.name()).collect::<Vec<_>>().join(", ")
}

/// 🎯️ The kind a name denotes (`wall`, `curtain-wall`, `wall type`, …).
pub fn parse_target(text: &str) -> Option<TemplateTarget> {
    let word = text.trim().to_ascii_lowercase().replace([' ', '_'], "-");
    TemplateTarget::ALL.into_iter().find(|target| target.name() == word)
}

/// 🎯️ The kinds a comma separated list names, each once in order of appearance; a name that denotes no kind refuses the whole edit, an empty text is no kind.
pub fn parse_targets(text: &str) -> Option<Vec<TemplateTarget>> {
    let mut found: Vec<TemplateTarget> = Vec::new();
    for word in text.split(',').map(str::trim).filter(|word| !word.is_empty()) {
        let target = parse_target(word)?;
        if !found.contains(&target) {
            found.push(target);
        }
    }
    Some(found)
}
//#endregion 🔖️Targets

//#region 🔖️Fields
/// 🧾️ The authored parameters of a property set template; its definitions are edited row by row.
pub static PROPERTY_TEMPLATE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.property_templates.get(id).map(|row| row.name.clone()), |_, id, value| set!(set_property_template::SetPropertyTemplate, id, "name", &value.trim().to_string())),
    field!("applies_to", field_applies_to, Text, |s, id| s.property_templates.get(id).map(|row| targets_text(&row.applies_to)), |_, id, value| set!(set_property_template::SetPropertyTemplate, id, "applies_to", &parse_targets(value)?)),
];

fn source_patch(id: &str, value: &str) -> Option<ModelMutation> {
    let source = Some(value.trim().to_string()).filter(|text| !text.is_empty());
    Some(ModelMutation::SetClassificationSystem(SetClassificationSystem::from_patch(id.into(), ClassificationSystemPatch { source: Some(Assigned::new(source)), ..Default::default() })))
}

/// 🧾️ The authored parameters of a classification system; its entry table is edited row by row.
pub static CLASSIFICATION_SYSTEM_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.classification_systems.get(id).map(|row| row.name.clone()), |_, id, value| set!(set_classification_system::SetClassificationSystem, id, "name", &value.trim().to_string())),
    field!("edition", field_edition, Text, |s, id| s.classification_systems.get(id).map(|row| row.edition.clone()), |_, id, value| set!(set_classification_system::SetClassificationSystem, id, "edition", &value.trim().to_string())),
    field!("source", field_source, Text, |s, id| s.classification_systems.get(id).map(|row| row.source.clone().unwrap_or_default()), |_, id, value| source_patch(id, value)),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
/// 🏷️ How many holders a template reaches: the holders with an effective property it defines.
pub fn template_holders(inference: &crate::ModelInference, id: &str) -> usize {
    inference.effective_properties.values().filter(|effective| effective.values.values().any(|set| set.values().any(|value| value.template.as_deref() == Some(id)))).count()
}

/// 🗂️ How many holders carry a code of a classification system.
pub fn system_holders(snapshot: &ModelSnapshot, id: &str) -> usize {
    snapshot.classifications.values().filter(|set| set.contains_key(id)).count()
}

/// 💡️ What a template defines and reaches.
pub static PROPERTY_TEMPLATE_INFERRED: &[InferredRow] = &[
    inferred!("definitions", field_definitions, |s, _, id| s.property_templates.get(id).map(|row| row.properties.len().to_string())),
    inferred!("reached", field_reached, |s, inference, id| s.property_templates.contains_key(id).then(|| template_holders(inference, id).to_string())),
];

/// 💡️ What a classification system lists and how much of the model uses it.
pub static CLASSIFICATION_SYSTEM_INFERRED: &[InferredRow] = &[
    inferred!("entries", field_entries, |s, _, id| s.classification_systems.get(id).map(|row| row.entries.len().to_string())),
    inferred!("classified", field_classified, |s, _, id| s.classification_systems.contains_key(id).then(|| system_holders(s, id).to_string())),
];
//#endregion 🔖️Inferred

//#region 🔖️Create
/// 🏷️ A new property set template: named, applying to nothing yet, with no definition.
pub fn create_property_template(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreatePropertyTemplate(crate::mutations::create_property_template::CreatePropertyTemplate { id: id.into(), template: PropertyTemplate { name: name.into(), applies_to: Vec::new(), properties: Vec::new() } }))
}

/// 🗂️ A new classification system: named, with no edition, no source and an empty entry table.
pub fn create_classification_system(_: &ModelSnapshot, id: &str, _parent: &str, name: &str) -> Created {
    Ok(ModelMutation::CreateClassificationSystem(crate::mutations::create_classification_system::CreateClassificationSystem { id: id.into(), system: ClassificationSystem { name: name.into(), edition: String::new(), source: None, entries: Vec::new() } }))
}
//#endregion 🔖️Create

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
