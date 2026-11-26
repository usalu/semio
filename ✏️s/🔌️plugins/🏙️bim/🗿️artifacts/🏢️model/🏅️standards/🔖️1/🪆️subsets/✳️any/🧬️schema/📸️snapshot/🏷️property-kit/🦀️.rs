//! 🏷️ The vocabulary of property templates and classification systems: what a property value is made of, which violations a definition can find in a value, the checks every authored template and classification table must pass, and the
//! navigation of an entry table. Pure and base-free: whether the elements a template applies to exist, and which values they carry, is decided by the leaves and by the inference, never here.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{ClassificationItem, PropertyDef, TemplateTarget};
use std::collections::BTreeSet;

/// 🧐️ Why a property set template cannot be stored, `None` when it can: the field of the template it is about and the message. A name; every kind at most once in `applies_to`; sound property definitions with unique names.
pub fn template_problem(name: &str, applies_to: &[TemplateTarget], properties: &[PropertyDef]) -> Option<(&'static str, String)> {
    if name.trim().is_empty() {
        return Some(("name", "A property set template needs a name.".to_string()));
    }
    let mut kinds = BTreeSet::new();
    if let Some(repeated) = applies_to.iter().find(|target| !kinds.insert(**target as u32)) {
        return Some(("applies_to", format!("The template lists the kind {repeated:?} twice.")));
    }
    let mut names = BTreeSet::new();
    for definition in properties {
        if let Some(problem) = definition.problem() {
            return Some(("properties", problem.1));
        }
        if !names.insert(definition.name.as_str()) {
            return Some(("properties", format!("The template defines the property \"{}\" twice.", definition.name)));
        }
    }
    None
}

/// 🧐️ Why an entry table cannot be stored, `None` when it can: a code is needed and unique, a parent is another row of the table, and the parents form a forest (no row is its own ancestor).
pub fn entries_problem(entries: &[ClassificationItem]) -> Option<String> {
    let mut codes = BTreeSet::new();
    for entry in entries {
        if entry.code.trim().is_empty() {
            return Some("A classification entry needs a code.".to_string());
        }
        if !codes.insert(entry.code.as_str()) {
            return Some(format!("The classification code \"{}\" is used by more than one entry.", entry.code));
        }
    }
    for entry in entries {
        let Some(parent) = &entry.parent else { continue };
        if !codes.contains(parent.as_str()) {
            return Some(format!("The parent \"{parent}\" of entry \"{}\" is not an entry of the table.", entry.code));
        }
    }
    for entry in entries {
        let mut at = entry;
        for _ in 0..=entries.len() {
            match at.parent.as_deref().and_then(|parent| entries.iter().find(|row| row.code == parent)) {
                Some(parent) if parent.code == entry.code => return Some(format!("Entry \"{}\" is its own ancestor.", entry.code)),
                Some(parent) => at = parent,
                None => break,
            }
        }
        if at.parent.is_some() {
            return Some(format!("Entry \"{}\" is its own ancestor.", entry.code));
        }
    }
    None
}


