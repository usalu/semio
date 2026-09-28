//! 📥️ Import a validated Forms design through undoable document events.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 📝️ Replaces authored content while retaining the current document identity and saved answers.
pub(super) fn replace_design_operations(current: &FormsSnapshot, next: &FormsSnapshot) -> Vec<FormMutation> {
    use crate::mutations::{change_form_title, create_step, delete_step};
    if current.title == next.title && current.definition == next.definition { return Vec::new(); }
    let mut operations = Vec::new();
    if next.title != current.title {
        operations.push(FormMutation::ChangeFormTitle(change_form_title::mutation::ChangeFormTitle { new_title: next.title.clone() }));
    }
    if current.definition != next.definition {
        operations.extend(current.definition.steps.iter().map(|step| FormMutation::DeleteStep(delete_step::mutation::DeleteStep { id: step.id.clone() })));
        operations.extend(next.definition.steps.iter().map(|step| FormMutation::CreateStep(create_step::mutation::CreateStep { step: step.clone(), index: None })));
    }
    operations
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "spec-json")]
pub struct SetSpecJson {
    pub json: String,
}

pub fn handle(payload: &SetSpecJson, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let next: FormsSnapshot = dsl::os_pack::json::from_json_str(&payload.json).map_err(|error| Fault::from(format!("forms.import.invalid: {error}")))?;
    Ok(Emit { artifact_mutations: replace_design_operations(doc.snapshot, &next), ..Default::default() })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
