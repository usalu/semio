//! 🧩️ Exact authored context passed to a question extension.
use crate::FormQuestion;
use semio_framework_pack_json::{Object, Value};

/// 🪟️ Authoring changes the definition; answering addresses one concrete Fill Form window.
#[derive(Clone, Copy, Debug)]
pub enum ExtensionSurface<'a> {
    Blueprint,
    Try { window_id: &'a str },
}

/// 🎨️ Carries the selected fixture and preserves explicit answers, including null.
pub fn render_payload(question: &FormQuestion, values: &Object, controller_id: &str, surface: ExtensionSurface<'_>, interactive: bool) -> Value {
    let mut payload = Object::new();
    if let Some(fixture) = &question.fixture_slug { payload.insert("fixtureSlug", Value::from(fixture.clone())); }
    payload.insert("params", values.get(&question.id).cloned().or_else(|| question.params.as_ref().map(crate::schema::dsl_to_value)).unwrap_or_else(|| Value::Object(Object::new())));
    payload.insert("questionId", Value::from(question.id.clone()));
    payload.insert("controllerId", Value::from(controller_id));
    match surface {
        ExtensionSurface::Blueprint => { payload.insert("surface", Value::from("blueprint")); }
        ExtensionSurface::Try { window_id } => {
            payload.insert("surface", Value::from("try"));
            payload.insert("windowId", Value::from(window_id));
            payload.insert("windowKindId", Value::from(crate::editor::forms::modes::blueprint::windows::try_wizard::FORMS_PLAY_WINDOW_TRY));
        }
    }
    payload.insert("interactive", Value::from(interactive));
    Value::Object(payload)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 🗂️ Schema-owned provider metadata for every contributed question kind.
#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct QuestionKindContribution {
    pub app_id: String,
    pub question_kind: String,
    pub label: semio_framework_ui_locale::LocalizedLabel,
    pub icon_id: String,
    pub params_body_key: String,
    pub preview_body_key: String,
    pub default_value_json: Option<String>,
}
