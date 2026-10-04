//! 📥️ 📥️ Forms play app commands command — `set-active-example`.

use crate::document_dsl as forms_dsl;
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::schema::empty_forms_snapshot;
use crate::{op::FormMutation, FormsSnapshot};
use super::set_spec_json::replace_design_operations;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let next = if payload.example_id.is_empty() { empty_forms_snapshot() } else {
        let text = match payload.example_id.as_str() {
            "building-component" | crate::examples::demo::ID => forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT,
            "default" => forms_dsl::DEFAULT_EXAMPLE_TEXT,
            "onboarding" => forms_dsl::ONBOARDING_EXAMPLE_TEXT,
            _ => return Err(Fault::from("forms.template.unknown")),
        };
        forms_dsl::parse_dsl(text).map_err(|error| Fault::from(format!("forms.template.invalid: {error}")))?
    };
    Ok(Emit { artifact_mutations: replace_design_operations(doc.snapshot, &next), ..Default::default() })
}
