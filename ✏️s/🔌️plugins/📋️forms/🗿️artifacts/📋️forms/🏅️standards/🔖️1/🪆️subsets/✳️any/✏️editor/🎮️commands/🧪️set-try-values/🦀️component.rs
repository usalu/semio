//! 🧪️ 🧪️ Forms play app commands command — `set-try-values`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::{effective_try_values, parse_value_json, reset_try_config_mutations, try_values_json_text, try_values_map};
use crate::artifacts::forms::schema::can_advance;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{dispatch, forms_app, render};
    use crate::editor::forms::{FormsCommand, FORMS_PLAY_BODY_TRY};

    fn seed_example(app: &mut crate::editor::forms::testkit::FormsApp, example_id: &str) {
        dispatch(app, FormsCommand::SetActiveExample(crate::editor::forms::commands::set_active_example::SetActiveExample { example_id: example_id.into() }));
    }

    #[test]
    fn try_wizard_gates_navigation_and_reports_inline_errors() {
        let mut app = forms_app();
        seed_example(&mut app, "default");
        dispatch(&mut app, FormsCommand::SetTryValues(SetTryValues { values_json: r#"{"name":"","email":""}"#.into() }));
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
        dispatch(&mut app, FormsCommand::SetTryValues(SetTryValues { values_json: r#"{"full-name":"Ada"}"#.into() }));
        dispatch(&mut app, FormsCommand::NextStep(crate::editor::forms::commands::next_step::NextStep {}));
        let second_json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(second_json.contains(r#""unit":"%""#));
    }

    #[test]
    fn set_try_values_updates_config() {
        let mut app = forms_app();
        seed_example(&mut app, "default");
        dispatch(&mut app, FormsCommand::SetTryValues(SetTryValues { values_json: r#"{"name":"Ada"}"#.into() }));
        let json = render(&mut app, FORMS_PLAY_BODY_TRY);
        assert!(json.contains("Ada"));
    }

    #[test]
    fn wizard_step_navigation() {
        let mut app = forms_app();
        seed_example(&mut app, "onboarding");
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 1 / 3"));
        dispatch(&mut app, FormsCommand::NextStep(crate::editor::forms::commands::next_step::NextStep {}));
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 2 / 3"));
        dispatch(&mut app, FormsCommand::PreviousStep(crate::editor::forms::commands::previous_step::PreviousStep {}));
        assert!(render(&mut app, FORMS_PLAY_BODY_TRY).contains("Step 1 / 3"));
    }

    #[test]
    fn conditional_visibility_hides_team_size() {
        let mut app = forms_app();
        seed_example(&mut app, "onboarding");
        let spec = app.snapshot().expect("projection");
        let steps = forms_steps(&spec);
        let advanced = steps.iter().find(|step| step.id == "advanced").expect("advanced step");
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
