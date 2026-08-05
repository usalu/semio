//! 📥️ Forms play app commands — whole-document import: raw JSON staging and built-in example switching.
//! Both replace the current spec with a new one through the existing `FormOperation` vocabulary (remove
//! every current step, retitle, re-add the new steps) so the edit still records a true inverse.

use crate::apps::forms::config::{FormsConfig, FormsConfigOperation};
use crate::apps::forms::reset_try_config_operations;
use crate::artifacts::forms::engine::{default_example_spec, empty_forms_projection, onboarding_example_spec};
use crate::artifacts::forms::{dsl, op::FormOperation, FormSpec};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Shell
/// ✏️ Emits the operations that replace the current form spec's title + steps with those of `next` — a
/// legitimate whole-document swap for import/example-switch, expressed granularly through the existing
/// `FormOperation` vocabulary so it still records a true inverse.
fn replace_spec_operations(current: &FormSpec, next: &FormSpec) -> Vec<FormOperation> {
    let mut operations: Vec<FormOperation> = current.steps.iter().map(|step| FormOperation::RemoveStep { step_id: step.id.clone() }).collect();
    if next.title != current.title {
        operations.push(FormOperation::UpdatePlaybook { title: next.title.clone() });
    }
    for step in &next.steps {
        operations.push(FormOperation::AddStep { step: step.clone(), index: None });
    }
    operations
}
//#endregion 🔖️Shell

//#region 🔖️SetSpecJson
pub mod set_spec_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "spec-json")]
    pub struct SetSpecJson {
        pub json: String,
    }

    pub fn handle(payload: &SetSpecJson, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormOperation, FormsConfigOperation>, Fault> {
        let Ok(next) = serde_json::from_str::<FormSpec>(&payload.json) else {
            return Ok(Emit::default());
        };
        let mut config_operations = reset_try_config_operations();
        config_operations.push(FormsConfigOperation::SetSelection { ids: Vec::new() });
        Ok(Emit { document_operations: replace_spec_operations(doc.projection, &next), config_operations, ..Default::default() })
    }
}
//#endregion 🔖️SetSpecJson

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, doc: &DocumentView<'_, FormSpec>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormOperation, FormsConfigOperation>, Fault> {
        let next = match payload.example_id.as_str() {
            "" => Some(empty_forms_projection()),
            "building-component" => dsl::parse_dsl(dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).ok(),
            "default" => Some(default_example_spec()),
            "onboarding" => Some(onboarding_example_spec()),
            _ => None,
        };
        let Some(next) = next else {
            return Ok(Emit::default());
        };
        let mut config_operations = reset_try_config_operations();
        config_operations.push(FormsConfigOperation::SetSelection { ids: Vec::new() });
        Ok(Emit { document_operations: replace_spec_operations(doc.projection, &next), config_operations, ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app};
    use crate::apps::forms::FormsCommand;
    use set_active_example::SetActiveExample;
    use set_spec_json::SetSpecJson;

    #[test]
    fn set_active_example_switches_to_the_onboarding_fixture() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::SetActiveExample(SetActiveExample { example_id: "onboarding".into() }));
        let spec = app.projection().expect("projection");
        assert_eq!(spec.id, "onboarding");
        assert_eq!(spec.steps.len(), 3);
    }

    #[test]
    fn set_active_example_with_blank_id_clears_the_document() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::SetActiveExample(SetActiveExample { example_id: "".into() }));
        let spec = app.projection().expect("projection");
        assert!(crate::artifacts::forms::engine::flatten_questions(&spec).is_empty());
    }

    #[test]
    fn set_spec_json_replaces_the_document() {
        let mut app = forms_app();
        let onboarding = serde_json::to_string(&crate::artifacts::forms::engine::onboarding_example_spec()).unwrap();
        dispatch(&mut app, FormsCommand::SetSpecJson(SetSpecJson { json: onboarding }));
        let spec = app.projection().expect("projection");
        assert_eq!(spec.id, "onboarding");
    }

    #[test]
    fn set_spec_json_with_invalid_json_is_a_no_operation() {
        let mut app = forms_app();
        let before = app.projection().expect("projection");
        dispatch(&mut app, FormsCommand::SetSpecJson(SetSpecJson { json: "not json".into() }));
        assert_eq!(app.projection().expect("projection"), before);
    }
}
//#endregion 🧪️Tests
