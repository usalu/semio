//! ▶️ Forms play app — the Try window: a wizard preview of the form as an end user would fill it out.

use crate::artifacts::forms::schema::{can_advance, default_value_for_question, is_extension_question_kind, json_f64_value, json_string_value, step_errors, visible_questions};
use crate::artifacts::forms::FormQuestion;
use crate::editor::forms::config::FormsConfig;
use crate::editor::forms::terminology::FormsLabels;
use crate::editor::forms::{effective_try_values, forms_action, parse_contributions, render_extension_question, ProgramContributionEntry};
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract as ui;
use ui::{Buildable, HasBase, HasChildren};
use crate::editor::forms::{ui_admit, ui_label, ui_text_value, ui_value_map, ui_value_text, ui_value_number};
use dsl::os_pack::json::{Object, Value};
use std::collections::HashMap;
use std::collections::HashSet;

//#region 🔖️Constants
pub const FORMS_PLAY_WINDOW_TRY: &str = "forms-try";
pub const FORMS_PLAY_BODY_TRY: &str = "forms.play.try";
const AVATAR_PLACEHOLDER_PNG_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FORMS_PLAY_WINDOW_TRY.into(),
        label: LocalizedLabel::native("Try", "Testen"),
        body_key: FORMS_PLAY_BODY_TRY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "play".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ Non-interactive preview — no `.window_kind_interactions(..)` reference for this window.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn answer_args(key: &str) -> UiAssemblyResult<ui::UiValue> {
    ui_value_map([("key", ui_value_text(key)?)])
}

fn display(value: &str, emphasize: bool) -> UiAssemblyResult<ui::BuiltNode> {
    ui_admit(ui::text(ui_label(value)?).emphasize(emphasize).try_build())
}

fn stack(axis: ui::Axis, children: Vec<ui::BuiltNode>) -> UiAssemblyResult<ui::BuiltNode> {
    ui_admit(ui_admit(ui::stack(axis).try_children(children))?.try_build())
}

fn control(builder: impl Into<ui::BuiltNode>, id: &str, label: &str, args: ui::UiValue) -> UiAssemblyResult<ui::BuiltNode> {
    let mut node = builder.into();
    node.key = ui_text_value(id)?;
    node.accessibility.label = Some(ui_label(label)?);
    let (action, args) = forms_action("setTryValue", Some(args))?;
    ui_admit(node.bindings.try_push(ui::ActionBinding { trigger: ui::Trigger::Change, action, args, capability: None }))?;
    Ok(node)
}

fn image_question_src(question: &FormQuestion) -> String {
    let src = question.src.as_deref().unwrap_or("");
    if src.is_empty() {
        return format!("data:image/png;base64,{AVATAR_PLACEHOLDER_PNG_BASE64}");
    }
    if src.starts_with("data:") || src.starts_with("http") || src.starts_with('/') || src.ends_with(".svg") {
        return src.into();
    }
    format!("data:image/png;base64,{src}")
}

fn try_field(question: &FormQuestion, error: Option<&str>, child: ui::BuiltNode) -> UiAssemblyResult<ui::BuiltNode> {
    let mut field = ui_admit(ui::field(ui_label(&question.label)?).try_id(format!("forms-try.{}", question.id)))?.required(question.required.unwrap_or(false));
    if let Some(description) = &question.description {
        field = field.description(ui_text_value(description)?);
    }
    if let Some(error) = error {
        field = field.error(ui_text_value(error)?);
    }
    ui_admit(ui_admit(field.try_child(child))?.try_build())
}

fn render_try_question(question: &FormQuestion, values: &Object, contributions: &[ProgramContributionEntry], error: Option<&str>, labels: &FormsLabels) -> UiAssemblyResult<ui::BuiltNode> {
    let value = values.get(&question.id).cloned().unwrap_or_else(|| json_value_from_dsl(question));
    let key = question.id.as_str();
    let label = question.label.as_str();
    let child = match question.kind.as_str() {
        "text" | "longText" | "number" | "date" | "color" | "file" => {
            let kind = match question.kind.as_str() {
                "longText" => ui::InputKind::LongText,
                "number" => ui::InputKind::Number,
                "date" => ui::InputKind::Date,
                "color" => ui::InputKind::Color,
                "file" => ui::InputKind::File,
                _ => ui::InputKind::Text,
            };
            let mut input = ui::input(kind).value(ui_text_value(json_string_value(&value))?);
            if let Some(placeholder) = &question.placeholder { input = input.placeholder(ui_label(placeholder)?); }
            if let Some(min) = question.min { input = input.min(min); }
            if let Some(max) = question.max { input = input.max(max); }
            if let Some(step) = question.step { input = input.step(step); }
            if let Some(accept) = &question.accept { input = input.accept(ui_text_value(accept)?); }
            control(input, &format!("forms-try.{key}.input"), label, answer_args(key)?)?
        }
        "slider" => {
            let mut slider = ui::slider(json_f64_value(&value)).min(question.min.unwrap_or(0.0)).max(question.max.unwrap_or(100.0)).step(question.step.unwrap_or(1.0));
            if let Some(unit) = &question.unit { slider = slider.unit(ui_text_value(unit)?); }
            control(slider, &format!("forms-try.{key}.slider"), label, answer_args(key)?)?
        }
        "boolean" => {
            let on = value.as_bool().unwrap_or(false);
            control(ui::toggle(on).icon(ui_text_value("check")?).text(ui_label(if on { labels.yes.as_str() } else { labels.no.as_str() })?), &format!("forms-try.{key}.toggle"), label, answer_args(key)?)?
        }
        "single" => {
            let mut select = ui::select(ui_text_value(json_string_value(&value))?);
            for option in question.options.iter().flatten() {
                select = ui_admit(select.try_item(ui_text_value(&option.value)?, ui_label(&option.label)?))?;
            }
            control(select, &format!("forms-try.{key}.select"), label, answer_args(key)?)?
        }
        "multi" => {
            let selected: HashSet<&str> = value.as_array().into_iter().flatten().filter_map(Value::as_str).collect();
            let mut children = Vec::new();
            for option in question.options.iter().flatten() {
                let args = ui_value_map([("key", ui_value_text(key)?), ("optionValue", ui_value_text(&option.value)?)])?;
                children.push(control(ui::toggle(selected.contains(option.value.as_str())).icon(ui_text_value("hash")?).text(ui_label(&option.label)?), &format!("forms-try.{key}.{}.toggle", option.value), &option.label, args)?);
            }
            stack(ui::Axis::Horizontal, children)?
        }
        "vector" => {
            let mut children = Vec::new();
            for (index, field) in question.fields.iter().flatten().enumerate() {
                let value = value.as_array().and_then(|array| array.get(index)).and_then(Value::as_f64).unwrap_or(field.value.unwrap_or(0.0));
                let args = || ui_value_map([("key", ui_value_text(key)?), ("vectorIndex", ui_value_number(index as f64))]);
                let id = format!("forms-try.{key}.{}.stepper", field.key);
                let mut node = control(ui_admit(ui::BuiltNode::try_new(&id, ui::Component::NumberStepper(ui::NumberStepperProps { value, step: question.step.unwrap_or(0.1), uniform: true })))?, &id, field.label.as_deref().unwrap_or(&field.key), args()?)?;
                let (action, args) = forms_action("setTryValue", Some(args()?))?;
                ui_admit(node.bindings.try_push(ui::ActionBinding { trigger: ui::Trigger::Delta, action, args, capability: None }))?;
                let field = ui_admit(ui::field(ui_label(field.label.as_deref().unwrap_or(&field.key))?).try_id(format!("forms-try.{key}.{}", field.key)))?;
                children.push(ui_admit(ui_admit(field.try_child(node))?.try_build())?);
            }
            stack(ui::Axis::Horizontal, children)?
        }
        "image" => ui_admit(ui_admit(ui::image(ui_text_value(image_question_src(question))?).alt(ui_label(label)?).try_id(format!("forms-try.{key}.image")))?.try_build())?,
        "note" => return display(question.text.as_deref().unwrap_or(label), false),
        kind if is_extension_question_kind(kind) => return render_extension_question(question, values, contributions, "try", true),
        _ => return display(&format!("Unsupported kind: {}", question.kind), false),
    };
    try_field(question, error, child)
}

fn json_value_from_dsl(question: &FormQuestion) -> Value {
    crate::artifacts::forms::schema::dsl_to_value(&default_value_for_question(question))
}

fn navigation(id: &str, label: &str, icon: &str, command: &str, disabled: bool) -> UiAssemblyResult<ui::BuiltNode> {
    let (action, args) = forms_action(command, None)?;
    let mut node: ui::BuiltNode = ui::button(ui_label(label)?).icon(ui_text_value(icon)?).disabled(disabled).into();
    node.key = ui_text_value(id)?;
    ui_admit(node.bindings.try_push(ui::ActionBinding { trigger: ui::Trigger::Activate, action, args, capability: None }))?;
    Ok(node)
}

pub fn render(spec: &crate::artifacts::forms::FormsSnapshot, config: &FormsConfig, labels: &FormsLabels) -> UiAssemblyResult<ui::BuiltNode> {
    let steps = crate::artifacts::forms::forms_steps(spec);
    if steps.is_empty() { return display(labels.no_steps_in_form.as_str(), false); }
    let contributions = parse_contributions(config);
    let step_index = (config.current_step_index as usize).min(steps.len().saturating_sub(1));
    let step = &steps[step_index];
    let values = effective_try_values(spec, config);
    let validation_values = values.iter().map(|(key, value)| (key.to_owned(), crate::artifacts::forms::schema::value_to_dsl(value))).collect();
    let visible = visible_questions(step, &validation_values);
    let errors = step_errors(step, &validation_values);
    let advance = can_advance(step, &validation_values);
    let errors_by_question: HashMap<&str, &str> = errors.iter().map(|error| (error.block_id.as_str(), error.message.as_str())).collect();
    let mut children = vec![
        display(spec.title.as_deref().unwrap_or(labels.form_fallback_title.as_str()), true)?,
        display(&format!("{} {} / {}", labels.step_progress.as_str(), step_index + 1, steps.len()), false)?,
        display(&step.title, true)?,
    ];
    if let Some(description) = &step.description { children.push(display(description, false)?); }
    for question in visible {
        children.push(render_try_question(question, &values, &contributions, errors_by_question.get(question.id.as_str()).copied(), labels)?);
    }
    let next = if step_index + 1 < steps.len() {
        navigation("forms-try.next", labels.next.as_str(), "chevron-right", "nextStep", !advance)?
    } else {
        navigation("forms-try.submit", labels.submit.as_str(), "check", "submit", !advance)?
    };
    children.push(stack(ui::Axis::Horizontal, vec![navigation("forms-try.back", labels.back.as_str(), "chevron-left", "previousStep", step_index == 0)?, next])?);
    stack(ui::Axis::Vertical, children)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{building_component_contributions, building_component_question, forms_app, render as render_body};
    use crate::editor::forms::FORMS_PLAY_BODY_TRY as BODY_TRY;

    #[semio_framework_async_macros::async_test]
    async fn renders_try_wizard() {
        let mut app = forms_app().await;
        crate::editor::forms::testkit::dispatch(&mut app, crate::editor::forms::FormsCommand::SetActiveExample(crate::editor::forms::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
        let json = render_body(&mut app, BODY_TRY).await;
        assert!(json.contains("forms-try"));
        assert!(json.contains("Step 1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn image_question_with_url_src_emits_image_node() {
        let question = FormQuestion { src: Some("https://example.com/picture.png".into()), ..crate::editor::forms::commands::add_question::question_shell("q-image".into(), "Picture".into(), "image".into()) };
        let node = render_try_question(&question, &Object::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&FormsConfig::default()));
        let json = serde_json::to_string(&node.expect("semantic component")).expect("component JSON");
        assert!(json.contains(r#""type":"image""#));
        assert!(json.contains("https://example.com/picture.png"));
    }

    #[semio_framework_async_macros::async_test]
    async fn extension_question_emits_external_slot_when_contribution_registered() {
        let node = render_try_question(&building_component_question(), &Object::new(), &building_component_contributions(), None, crate::editor::forms::terminology::forms_play_labels(&FormsConfig::default()));
        let json = serde_json::to_string(&node.expect("semantic component")).expect("component JSON");
        assert!(json.contains("\"type\":\"extension\""));
        assert!(json.contains("forms-module-procedural"));
    }

    #[semio_framework_async_macros::async_test]
    async fn extension_question_falls_back_without_contribution() {
        let node = render_try_question(&building_component_question(), &Object::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&FormsConfig::default()));
        let json = serde_json::to_string(&node.expect("semantic component")).expect("component JSON");
        assert!(json.contains("Extension unavailable"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_canvas2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, FORMS_PLAY_BODY_TRY);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod control_vectors {
    use super::*;
    #[test]
    fn semantic_question_controls_match_the_language_neutral_vectors() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️controls.json")).unwrap();
        for vector in vectors["cases"].as_array().unwrap() {
            let question = &vector["question"];
            let question = crate::editor::forms::commands::add_question::question_shell(question["id"].as_str().unwrap().into(), question["label"].as_str().unwrap().into(), question["kind"].as_str().unwrap().into());
            let node = render_try_question(&question, &Object::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&FormsConfig::default())).unwrap();
            let control = if question.kind == "note" { &node } else { node.children.get(0).unwrap() };
            let actual = serde_json::to_value(control).unwrap();
            assert_eq!(actual["component"]["type"], vector["component"], "{}", question.kind);
            if vector["interactive"].as_bool().unwrap() {
                assert_eq!(actual["accessibility"]["label"], vector["question"]["label"]);
                assert_eq!(actual["bindings"][0]["trigger"], "change");
                assert_eq!(actual["bindings"][0]["args"]["key"], vector["question"]["id"]);
            }
        }
    }
}
