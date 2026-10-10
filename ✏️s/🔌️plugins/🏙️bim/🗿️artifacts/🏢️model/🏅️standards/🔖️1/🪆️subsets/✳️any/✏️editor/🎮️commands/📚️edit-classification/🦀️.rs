//! 📚️ `editClassification`: one edit of the entry table of a classification system, from the properties panel: add an entry (code, title and optional parent), retitle one, move one under another parent (or to the root) or remove
//! one that has no children. The edit is a pure function of the table ([`apply`]) and becomes one `set-classification-system` mutation that decides whether the result stands.

use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::mutations::set_classification_system::SetClassificationSystem;
use crate::{ClassificationItem, ClassificationSystemPatch, ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit-classification")]
pub struct EditClassification {
    pub id: String,
    pub op: String,
    pub code: String,
    pub title: String,
    pub parent: String,
}

fn optional(text: &str) -> Option<String> {
    Some(text.trim().to_string()).filter(|text| !text.is_empty())
}

/// 📚️ The entry table after one edit; the error is the fault code of an edit that makes no sense (an unknown operation or code, a code that exists already, an entry that still has children).
pub fn apply(entries: &[ClassificationItem], edit: &EditClassification) -> Result<Vec<ClassificationItem>, &'static str> {
    let mut list = entries.to_vec();
    let code = edit.code.trim();
    let at = list.iter().position(|entry| entry.code == code);
    match (edit.op.as_str(), at) {
        ("add", None) if !code.is_empty() => list.push(ClassificationItem { code: code.to_string(), title: edit.title.trim().to_string(), parent: optional(&edit.parent) }),
        ("retitle", Some(index)) => list[index].title = edit.title.trim().to_string(),
        ("reparent", Some(index)) => list[index].parent = optional(&edit.parent),
        ("remove", Some(index)) => {
            if entries.iter().any(|entry| entry.parent.as_deref() == Some(code)) {
                return Err("bim.classification.entry-has-children");
            }
            list.remove(index);
        }
        _ => return Err("bim.classification.edit-invalid"),
    }
    Ok(list)
}

/// 📚️ The edit with the typed line of the panel resolved: without a code the title holds `code | title | parent` (add), `code | title` (retitle), `code | parent` (move) or `code` (remove).
pub fn normalised(edit: &EditClassification) -> EditClassification {
    if !edit.code.trim().is_empty() {
        return edit.clone();
    }
    let mut parts = edit.title.split('|').map(str::trim);
    let (code, first, second) = (parts.next().unwrap_or_default(), parts.next().unwrap_or_default(), parts.next().unwrap_or_default());
    match edit.op.as_str() {
        "add" => EditClassification { code: code.into(), title: first.into(), parent: second.into(), ..edit.clone() },
        "retitle" => EditClassification { code: code.into(), title: first.into(), ..edit.clone() },
        "reparent" => EditClassification { code: code.into(), title: String::new(), parent: first.into(), ..edit.clone() },
        _ => EditClassification { code: code.into(), title: String::new(), ..edit.clone() },
    }
}

pub fn handle(payload: &EditClassification, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let system = doc.snapshot.classification_systems.get(&payload.id).ok_or_else(|| fault("bim.classification.system-missing", format!("the classification system '{}' does not exist", payload.id)))?;
    let edit = normalised(payload);
    let entries = apply(&system.entries, &edit).map_err(|code| fault(code, format!("the system '{}' cannot take the edit '{} {}'", system.name, edit.op, edit.code)))?;
    Ok(Emit::mutations(vec![ModelMutation::SetClassificationSystem(SetClassificationSystem::from_patch(payload.id.clone(), ClassificationSystemPatch { entries: Some(entries), ..Default::default() }))]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
