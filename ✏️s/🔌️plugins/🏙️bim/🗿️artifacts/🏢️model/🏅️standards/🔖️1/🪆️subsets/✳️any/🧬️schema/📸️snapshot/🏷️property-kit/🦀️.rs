//! 🏷️ The vocabulary of property templates and classification systems: what a property value is made of, which violations a definition can find in a value, the checks every authored template and classification table must pass, and the
//! navigation of an entry table. Pure and base-free: whether the elements a template applies to exist, and which values they carry, is decided by the leaves and by the inference, never here.

use crate::standards::v1::subsets::any::schema::snapshot::values::{ClassificationItem, PropertyDef, TemplateTarget};
use crate::standards::v1::subsets::any::schema::snapshot::entities::ClassificationSystem;
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

impl ClassificationSystem {
    /// 🔎️ The row of a code.
    pub fn entry(&self, code: &str) -> Option<&ClassificationItem> {
        self.entries.iter().find(|entry| entry.code == code)
    }

    /// 🌳️ The rows whose parent is `parent` (the roots for `None`), in table order.
    pub fn children(&self, parent: Option<&str>) -> Vec<&ClassificationItem> {
        self.entries.iter().filter(|entry| entry.parent.as_deref() == parent).collect()
    }

    /// 🧭️ The rows from the root down to the row of `code`, empty when the code is no row.
    pub fn lineage(&self, code: &str) -> Vec<&ClassificationItem> {
        let mut path = Vec::new();
        let mut at = self.entry(code);
        while let Some(entry) = at {
            if path.len() > self.entries.len() {
                break;
            }
            path.push(entry);
            at = entry.parent.as_deref().and_then(|parent| self.entry(parent));
        }
        path.reverse();
        path
    }

    /// 🔢️ How deep the row of `code` stands: zero for a root, none for an unknown code.
    pub fn depth(&self, code: &str) -> Option<usize> {
        self.entry(code).map(|_| self.lineage(code).len().saturating_sub(1))
    }

    /// 📖️ The rows in tree order: every root with its descendants below it, depth first, each branch in table order.
    pub fn tree(&self) -> Vec<(usize, &ClassificationItem)> {
        fn walk<'a>(system: &'a ClassificationSystem, parent: Option<&str>, depth: usize, out: &mut Vec<(usize, &'a ClassificationItem)>) {
            if depth > system.entries.len() {
                return;
            }
            for entry in system.children(parent) {
                out.push((depth, entry));
                walk(system, Some(entry.code.as_str()), depth + 1, out);
            }
        }
        let mut out = Vec::new();
        walk(self, None, 0, &mut out);
        out
    }
}


