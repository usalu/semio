//! 🧪️ Forms play app commands — the Try wizard: in-progress answer values and step navigation.
//!
//! Every handler here is config-only (never emits document operations) — the Try wizard's state is
//! ephemeral session data, not part of the authored form.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::{effective_try_values, parse_value_json, reset_try_config_mutations, try_values_json_text, try_values_map};
use crate::artifacts::forms::schema::can_advance;
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//#region 🔖️Values
/// ✏️ Patches one JSON-object field of a try value keyed by `key` (used by the vector-field-parameter
/// try-value shape, e.g. a building-component question's `height`/`radius`/`sides` params).
fn patch_try_object_field(values: &mut Map<String, Value>, key: &str, field: &str, raw: &Value) {
    let mut object = values.get(key).cloned().unwrap_or_else(|| json!({}));
    if let Some(map) = object.as_object_mut() {
        map.insert(field.into(), raw.clone());
        values.insert(key.into(), object);
    }
}

/// ✏️ Patches one numeric index of a try value keyed by `key` (used by the vector question kind's
/// per-component try value).
fn patch_try_vector_field(values: &mut Map<String, Value>, key: &str, index: usize, raw: &Value) {
    let mut array = values.get(key).and_then(|value| value.as_array().cloned()).unwrap_or_default();
    while array.len() <= index {
        array.push(json!(0.0));
    }
    array[index] = raw.clone();
    values.insert(key.into(), Value::Array(array));
}
//#endregion 🔖️Values

//#region 🔖️SetTryValue
pub mod set_try_value {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "try-value")]
    pub struct SetTryValue {
        pub key: String,
        pub value_json: Option<String>,
        pub option_value: Option<String>,
        pub vector_index: Option<u64>,
        pub param_key: Option<String>,
    }

    pub fn handle(payload: &SetTryValue, _doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let mut values = try_values_map(config);
        if let Some(option_value) = &payload.option_value {
            let mut selected = values.get(payload.key.as_str()).and_then(|value| value.as_array().cloned()).unwrap_or_default();
            let pressed = payload.value_json.as_deref().map(parse_value_json).and_then(|value| value.as_bool()).unwrap_or(false);
            if pressed {
                if !selected.iter().any(|entry| entry.as_str() == Some(option_value.as_str())) {
                    selected.push(json!(option_value));
                }
            } else {
                selected.retain(|entry| entry.as_str() != Some(option_value.as_str()));
            }
            values.insert(payload.key.clone(), Value::Array(selected));
        } else if let Some(index) = payload.vector_index {
            if let Some(raw) = payload.value_json.as_deref().map(parse_value_json) {
                patch_try_vector_field(&mut values, &payload.key, index as usize, &raw);
            }
        } else if let Some(param_key) = &payload.param_key {
            if let Some(raw) = payload.value_json.as_deref().map(parse_value_json) {
                patch_try_object_field(&mut values, &payload.key, param_key, &raw);
            }
        } else if let Some(raw) = payload.value_json.as_deref().map(parse_value_json) {
            values.insert(payload.key.clone(), raw);
        }
        Ok(Emit::config(vec![FormsConfigMutation::SetTryValues { json: try_values_json_text(&values) }]))
    }
}
//#endregion 🔖️SetTryValue

//#region 🔖️SetTryValues
pub mod set_try_values {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "try-values")]
    pub struct SetTryValues {
        pub values_json: String,
    }

    pub fn handle(payload: &SetTryValues, _doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let mut values = try_values_map(cfg.snapshot);
        if let Some(incoming) = serde_json::from_str::<Value>(&payload.values_json).ok().and_then(|value| value.as_object().cloned()) {
            for (key, value) in incoming {
                values.insert(key, value);
            }
        }
        Ok(Emit::config(vec![FormsConfigMutation::SetTryValues { json: try_values_json_text(&values) }]))
    }
}
//#endregion 🔖️SetTryValues

//#region 🔖️ResetTry
pub mod reset_try {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reset-try")]
    pub struct ResetTry {}

    pub fn handle(_payload: &ResetTry, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        Ok(Emit::config(reset_try_config_mutations()))
    }
}
//#endregion 🔖️ResetTry

//#region 🔖️PreviousStep
pub mod previous_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "previous-step")]
    pub struct PreviousStep {}

    pub fn handle(_payload: &PreviousStep, _doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        Ok(Emit::config(vec![FormsConfigMutation::SetStepIndex { index: cfg.snapshot.current_step_index.saturating_sub(1) }]))
    }
}
//#endregion 🔖️PreviousStep

//#region 🔖️NextStep
pub mod next_step {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "next-step")]
    pub struct NextStep {}

    pub fn handle(_payload: &NextStep, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        let spec = doc.snapshot;
        let config = cfg.snapshot;
        let index = config.current_step_index as usize;
        if index + 1 < spec.steps.len() {
            let step = &spec.steps[index];
            let values = effective_try_values(spec, config);
            if can_advance(step, &values) {
                return Ok(Emit::config(vec![FormsConfigMutation::SetStepIndex { index: config.current_step_index + 1 }]));
            }
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️NextStep

//#region 🔖️Submit
pub mod submit {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "submit")]
    pub struct Submit {}

    pub fn handle(_payload: &Submit, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️Submit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app, render};
    use crate::apps::forms::{FormsCommand, FORMS_PLAY_BODY_TRY};

    fn seed_example(app: &mut crate::apps::forms::testkit::FormsApp, example_id: &str) {
        dispatch(app, FormsCommand::SetActiveExample(crate::apps::forms::commands::import::set_active_example::SetActiveExample { example_id: example_id.into() }));
    }

    #[test]
    fn try_wizard_gates_navigation_and_reports_inline_errors() {
        let mut app = forms_app();
        seed_example(&mut app, "default");
        dispatch(&mut app, FormsCommand::SetTryValues(set_try_values::SetTryValues { values_json: r#"{"name":"","email":""}"#.into() }));
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        // 🩹️ `UiPresence` serializes disabled state as `{"state":"disabled"}`, not a bare `"disabled":true`
        // boolean — kept from the pre-migration test, which already documented this wire shape.
        assert!(json.contains(r#""state":"disabled""#));
        assert!(json.contains(r#""error":"#));
        assert!(json.contains("forms-try.back"));
    }

    #[test]
    fn try_wizard_emits_slider_unit_and_number_bounds() {
        let mut app = forms_app();
        seed_example(&mut app, "onboarding");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(json.contains(r#""min":13.0"#) || json.contains(r#""min":13"#));
        assert!(json.contains(r#""max":120.0"#) || json.contains(r#""max":120"#));
        dispatch(&mut app, FormsCommand::SetTryValues(set_try_values::SetTryValues { values_json: r#"{"full-name":"Ada"}"#.into() }));
        dispatch(&mut app, FormsCommand::NextStep(next_step::NextStep {}));
        let second_json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(second_json.contains(r#""unit":"%""#));
    }

    #[test]
    fn set_try_values_updates_config() {
        let mut app = forms_app();
        seed_example(&mut app, "default");
        dispatch(&mut app, FormsCommand::SetTryValues(set_try_values::SetTryValues { values_json: r#"{"name":"Ada"}"#.into() }));
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(json.contains("Ada"));
    }

    #[test]
    fn wizard_step_navigation() {
        let mut app = forms_app();
        seed_example(&mut app, "onboarding");
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 1 / 3"));
        dispatch(&mut app, FormsCommand::NextStep(next_step::NextStep {}));
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 2 / 3"));
        dispatch(&mut app, FormsCommand::PreviousStep(previous_step::PreviousStep {}));
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 1 / 3"));
    }

    #[test]
    fn conditional_visibility_hides_team_size() {
        let mut app = forms_app();
        seed_example(&mut app, "onboarding");
        let spec = app.snapshot().expect("projection");
        let advanced = spec.steps.iter().find(|step| step.id == "advanced").expect("advanced step");
        let values = crate::artifacts::forms::schema::initial_try_values(&spec, &Map::new());
        assert_eq!(crate::artifacts::forms::schema::visible_questions(advanced, &values).len(), 1);
    }

    #[test]
    fn renders_try_wizard() {
        let mut app = forms_app();
        seed_example(&mut app, "default");
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(json.contains("forms-try"));
        assert!(json.contains("Step 1"));
    }
}
//#endregion 🧪️Tests
