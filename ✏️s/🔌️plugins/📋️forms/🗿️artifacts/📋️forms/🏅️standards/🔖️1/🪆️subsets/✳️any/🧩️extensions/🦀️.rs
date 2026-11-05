//! 🧩️ Exact authored context passed to a question extension.
use crate::FormQuestion;
use semio_framework_pack_json::{Object, Value};

/// 🪟️ Authoring changes the definition; answering addresses one concrete Fill Form window.
#[derive(Clone, Copy, Debug)]
pub enum ExtensionSurface<'a> {
    Blueprint,
    Try { window_id: &'a str, window_kind_id: &'a str },
    View,
}

/// 🎨️ Carries the selected fixture and preserves explicit answers, including null.
pub fn render_payload(question: &FormQuestion, values: &Object, controller_id: &str, surface: ExtensionSurface<'_>, interactive: bool) -> Value {
    let mut payload = Object::new();
    if let Some(fixture) = &question.example_id { payload.insert("exampleId", Value::from(fixture.clone())); }
    payload.insert("params", values.get(&question.id).cloned().or_else(|| question.params.as_ref().map(crate::standards::v1::subsets::any::io::text::snapshot::dsl_to_value)).unwrap_or_else(|| Value::Object(Object::new())));
    payload.insert("authoredParams", question.params.as_ref().map(crate::standards::v1::subsets::any::io::text::snapshot::dsl_to_value).unwrap_or_else(|| Value::Object(Object::new())));
    payload.insert("questionId", Value::from(question.id.clone()));
    payload.insert("controllerId", Value::from(controller_id));
    match surface {
        ExtensionSurface::Blueprint => { payload.insert("surface", Value::from("blueprint")); }
        ExtensionSurface::View => { payload.insert("surface", Value::from("try")); }
        ExtensionSurface::Try { window_id, window_kind_id } => {
            payload.insert("surface", Value::from("try"));
            payload.insert("windowId", Value::from(window_id));
            payload.insert("windowKindId", Value::from(window_kind_id));
        }
    }
    payload.insert("interactive", Value::from(interactive));
    Value::Object(payload)
}



/// 🗂️ Schema-owned provider metadata for every contributed question kind.
#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct QuestionKindContribution {
    pub app_id: String,
    pub question_kind: String,
    pub label: semio_ui_locale::LocalizedLabel,
    pub icon_id: String,
    pub params_body_key: String,
    pub preview_body_key: String,
    pub default_value_json: Option<String>,
}

use semio_framework::ProgramContributionEntry;
use semio_plugin::{PluginAssemblyError, UiAssemblyResult};
use semio_ui_contract::{Label, UiText, UiValue, UiMapBuilder};

fn ui_admit<T,E>(value: Result<T,E>) -> UiAssemblyResult<T> { value.map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "question extension admission failed")) }
fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<Label> { ui_admit(Label::try_from(value.as_ref())) }
fn ui_text_value(value: impl AsRef<str>) -> UiAssemblyResult<UiText> { UiText::try_from_str(value.as_ref()).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "question extension text admission failed")) }
fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> { Ok(UiValue::Text(ui_text_value(value)?)) }
fn ui_value_map<const N: usize>(mut entries: [(&str, UiValue); N]) -> UiAssemblyResult<UiValue> {
    entries.sort_unstable_by(|a,b| a.0.cmp(b.0));
    let mut map = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "question extension map admission failed"))?;
    for (key,value) in entries { ui_admit(map.push(key.to_owned(),value))?; }
    Ok(UiValue::Map(map.finish()))
}
pub const FORMS_QUESTION_KIND_TOPIC: &str = "forms.questionKind";

/// 🎯️ A resolved question-kind contribution's routing fields, sourced from the open `forms.questionKind`
/// topic shape.
struct QuestionKindRoute {
    app_id: String,
    params_body_key: String,
    preview_body_key: String,
}

fn question_kind_route_from_topic(topic_contribution: &semio_plugin::TopicContribution, kind: &str) -> Option<QuestionKindRoute> {
    if topic_contribution.topic != FORMS_QUESTION_KIND_TOPIC {
        return None;
    }
    let payload = topic_contribution.decode::<QuestionKindContribution>().ok()?;
    (payload.question_kind == kind).then_some(QuestionKindRoute { app_id: payload.app_id, params_body_key: payload.params_body_key, preview_body_key: payload.preview_body_key })
}

/// 🗂️ Reads the open `TopicContribution` (`"forms.questionKind"` topic) shape per entry.
fn find_question_kind_contribution<'a>(contributions: &'a [ProgramContributionEntry], kind: &str) -> Option<(&'a str, QuestionKindRoute)> {
    contributions.iter().find_map(|entry| {
        let route = entry.topic_contribution.as_ref().and_then(|topic_contribution| question_kind_route_from_topic(topic_contribution, kind))?;
        Some((entry.plugin_id.as_str(), route))
    })
}

/// 🧩️ Renders a contributed (extension) question kind as a pair of external slots (params editor +
/// preview), or an "Extension unavailable" diagnostic when no contribution is registered for it. Shared
/// by the try wizard and the inspection panel's kind-specific editor fields.
pub fn render_extension_question(question: &FormQuestion, values: &Object, contributions: &[ProgramContributionEntry], controller_id: &str, surface: ExtensionSurface<'_>, interactive: bool, unavailable_label: &str) -> semio_plugin::UiAssemblyResult<semio_plugin::BuiltNode> {
    use semio_ui_contract::{self as ui, Buildable, HasBase, HasChildren};
    let key = format!("{}.extension", question.id);
    let Some((plugin_id, route)) = find_question_kind_contribution(contributions, &question.kind) else {
        return ui_admit(ui_admit(ui::text(ui_label(format!("{}: {}", unavailable_label, question.kind))?).try_id(&key))?.try_build());
    };
    let payload = render_payload(question, values, controller_id, surface, interactive).to_string();
    let mut column = ui_admit(ui::column().try_id(&key))?;
    for body_key in [&route.params_body_key, &route.preview_body_key] {
        let props = ui_value_map([("bodyKey", ui_value_text(body_key)?), ("paramsJsonChunks", ui_admit(ui::UiValue::try_text_chunks(&payload, 65_536))?)])?;
        let slot = ui_admit(ui::extension(ui_text_value(format!("{plugin_id}/{}", route.app_id))?).props(props).try_id(format!("{}.{}", question.id, body_key)))?;
        column = ui_admit(column.try_child(slot))?;
    }
    ui_admit(column.try_build())
}

