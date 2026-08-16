//! ▶️ Forms play app — the Try window: a wizard preview of the form as an end user would fill it out.

use crate::editor::forms::config::FormsConfig;
use crate::editor::forms::terminology::FormsLabels;
use crate::editor::forms::{effective_try_values, forms_action, parse_contributions, render_extension_question, ProgramContributionEntry};
use crate::artifacts::forms::schema::{can_advance, default_value_for_question, is_extension_question_kind, json_f64_value, json_string_value, step_errors, visible_questions};
use crate::artifacts::forms::FormQuestion;
use semio_framework_plugin::{
    ActionDescriptor, Label, LocalizedLabel, SurfaceKind, UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode, UiStackNode, UiTextNode, UiToggleNode, WindowKindDefinition, WindowOptions,
};
use serde_json::{json, Map, Value};
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
fn try_value_action(key: &str) -> ActionDescriptor {
    forms_action("setTryValue", Some(json!({ "key": key })))
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

fn render_image_question(question: &FormQuestion) -> UiNode {
    semio_framework_plugin::ui_image(format!("forms-try.{}.image", question.id), image_question_src(question), Some(Label::data(question.label.clone())))
}

fn ui_text_emphasized(value: impl Into<Label>) -> UiNode {
    UiNode::Text(UiTextNode { value: value.into(), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn ui_stack_horizontal(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "horizontal".into(), gap: Some("tight".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

fn try_field(question: &FormQuestion, error: Option<&str>, child: UiNode) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: format!("forms-try.{}", question.id),
        label: Label::data(question.label.clone()),
        description: question.description.clone(),
        required: question.required.filter(|required| *required),
        error: error.map(str::to_string),
        child: Box::new(child),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn render_try_question(question: &FormQuestion, values: &Map<String, Value>, contributions: &[ProgramContributionEntry], error: Option<&str>, labels: &FormsLabels) -> UiNode {
    let value = values.get(&question.id).cloned().unwrap_or_else(|| json_value_from_dsl(question));
    let key = question.id.clone();
    match question.kind.as_str() {
        "text" | "longText" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: question.kind.clone(),
                value: json_string_value(&value),
                placeholder: question.placeholder.clone().map(Label::data),
                commit: None,
                on_change: try_value_action(&key),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        "number" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "number".into(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_action(&key),
                min: question.min,
                max: question.max,
                step: question.step,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        "slider" => try_field(
            question,
            error,
            UiNode::Slider(UiSliderNode {
                id: format!("forms-try.{key}.slider"),
                value: json_f64_value(&value),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                unit: question.unit.clone(),
                on_change: try_value_action(&key),
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        "boolean" => try_field(
            question,
            error,
            UiNode::Toggle(UiToggleNode {
                id: format!("forms-try.{key}.toggle"),
                icon_id: "check".into(),
                text: Some(if value.as_bool().unwrap_or(false) { labels.yes.into() } else { labels.no.into() }),
                on_change: try_value_action(&key),
                presence: UiPresence::selected(value.as_bool().unwrap_or(false)),
                menu: None,
            }),
        ),
        "single" => {
            let items = question.options.as_ref().map(|options| options.iter().map(|option| UiSelectItem { value: option.value.clone(), label: Label::data(option.label.clone()) }).collect()).unwrap_or_default();
            try_field(question, error, UiNode::Select(UiSelectNode { id: format!("forms-try.{key}.select"), value: json_string_value(&value), placeholder: None, items, on_change: try_value_action(&key), presence: UiPresence::default(), menu: None }))
        }
        "multi" => {
            let selected: HashSet<String> = value.as_array().map(|items| items.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect()).unwrap_or_default();
            let chips = question
                .options
                .as_ref()
                .map(|options| {
                    options
                        .iter()
                        .map(|option| {
                            UiNode::Toggle(UiToggleNode {
                                id: format!("forms-try.{key}.{}.toggle", option.value),
                                icon_id: "hash".into(),
                                text: Some(Label::data(option.label.clone())),
                                on_change: forms_action("setTryValue", Some(json!({ "key": key, "optionValue": option.value }))),
                                presence: UiPresence::selected(selected.contains(&option.value)),
                                menu: None,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            try_field(question, error, ui_stack_horizontal(chips))
        }
        "date" | "color" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: question.kind.clone(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_action(&key),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        "vector" => {
            let array = value.as_array().cloned().unwrap_or_default();
            let fields = question.fields.as_ref().cloned().unwrap_or_default();
            let steppers: Vec<UiNode> = fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let field_value = array.get(index).cloned().unwrap_or(json!(field.value.unwrap_or(0.0)));
                    UiNode::Field(UiFieldNode {
                        id: format!("forms-try.{key}.{}", field.key),
                        label: Label::data(field.label.clone().unwrap_or_else(|| field.key.clone())),
                        description: None,
                        required: None,
                        error: None,
                        presence: UiPresence::default(),
                        child: Box::new(UiNode::NumberStepper(semio_framework_plugin::UiNumberStepperNode {
                            id: format!("forms-try.{key}.{}.stepper", field.key),
                            value: json_f64_value(&field_value),
                            step: question.step.unwrap_or(0.1),
                            uniform: true,
                            on_absolute: forms_action("setTryValue", Some(json!({ "key": key, "vectorIndex": index }))),
                            on_delta: forms_action("setTryValue", Some(json!({ "key": key, "vectorIndex": index }))),
                            presence: UiPresence::default(),
                            menu: None,
                        })),
                        menu: None,
                    })
                })
                .collect();
            try_field(question, error, ui_stack_horizontal(steppers))
        }
        "note" => semio_framework_plugin::ui_text(Label::data(question.text.clone().unwrap_or_else(|| question.label.clone()))),
        "image" => try_field(question, error, render_image_question(question)),
        "file" => try_field(
            question,
            error,
            UiNode::Input(UiInputNode {
                id: format!("forms-try.{key}.input"),
                input_kind: "file".into(),
                value: json_string_value(&value),
                placeholder: None,
                commit: None,
                on_change: try_value_action(&key),
                min: None,
                max: None,
                step: None,
                accept: question.accept.clone(),
                presence: UiPresence::default(),
                menu: None,
            }),
        ),
        kind if is_extension_question_kind(kind) => render_extension_question(question, values, contributions, "try", true),
        _ => semio_framework_plugin::ui_text(Label::data(format!("Unsupported kind: {}", question.kind))),
    }
}

/// 🔄️ The question's typed default, as a `serde_json::Value` — used when no try value has been entered
/// for it yet.
fn json_value_from_dsl(question: &FormQuestion) -> Value {
    crate::artifacts::forms::schema::dsl_to_value(&default_value_for_question(question))
}

pub fn render(spec: &crate::artifacts::forms::FormsSnapshot, config: &FormsConfig, labels: &FormsLabels) -> UiNode {
    let steps = crate::artifacts::forms::forms_steps(spec);
    if steps.is_empty() {
        return semio_framework_plugin::ui_text(labels.no_steps_in_form);
    }
    let contributions = parse_contributions(config);
    let step_index = (config.current_step_index as usize).min(steps.len().saturating_sub(1));
    let step = &steps[step_index];
    let values = effective_try_values(spec, config);
    let visible = visible_questions(step, &values);
    let errors = step_errors(step, &values);
    let advance = can_advance(step, &values);
    let errors_by_question: HashMap<&str, &str> = errors.iter().map(|error| (error.block_id.as_str(), error.message.as_str())).collect();
    let mut children = vec![
        ui_text_emphasized(Label::data(spec.title.clone().unwrap_or_else(|| labels.form_fallback_title.into()))),
        semio_framework_plugin::ui_text(Label::data(format!("{} {} / {}", labels.step_progress.as_str(), step_index + 1, steps.len()))),
        ui_text_emphasized(Label::data(step.title.clone())),
    ];
    if let Some(description) = &step.description {
        children.push(semio_framework_plugin::ui_text(Label::data(description.clone())));
    }
    for question in visible {
        children.push(render_try_question(question, &values, &contributions, errors_by_question.get(question.id.as_str()).copied(), labels));
    }
    let nav = vec![
        UiNode::Button(UiButtonNode {
            id: Some("forms-try.back".into()),
            icon_id: "chevron-left".into(),
            label: labels.back.into(),
            action: forms_action("previousStep", None),
            style: None,
            presence: UiPresence::disabled_if(step_index == 0),
            menu: None,
        }),
        if step_index + 1 < steps.len() {
            UiNode::Button(UiButtonNode { id: Some("forms-try.next".into()), icon_id: "chevron-right".into(), label: labels.next.into(), action: forms_action("nextStep", None), style: None, presence: UiPresence::disabled_if(!advance), menu: None })
        } else {
            UiNode::Button(UiButtonNode { id: Some("forms-try.submit".into()), icon_id: "check".into(), label: labels.submit.into(), action: forms_action("submit", None), style: None, presence: UiPresence::disabled_if(!advance), menu: None })
        },
    ];
    children.push(ui_stack_horizontal(nav));
    semio_framework_plugin::ui_stack_vertical(children)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{building_component_contributions, building_component_question, forms_app, render as render_body};
    use crate::editor::forms::FORMS_PLAY_BODY_TRY as BODY_TRY;

    #[test]
    fn renders_try_wizard() {
        let mut app = forms_app();
        crate::editor::forms::testkit::dispatch(&mut app, crate::editor::forms::FormsCommand::SetActiveExample(crate::editor::forms::commands::set_active_example::SetActiveExample { example_id: "default".into() }));
        let json = render_body(&mut app, BODY_TRY);
        assert!(json.contains("forms-try"));
        assert!(json.contains("Step 1"));
    }

    #[test]
    fn image_question_with_url_src_emits_image_node() {
        let question = FormQuestion { src: Some("https://example.com/picture.png".into()), ..crate::editor::forms::commands::add_question::question_shell("q-image".into(), "Picture".into(), "image".into()) };
        let node = render_try_question(&question, &Map::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&FormsConfig::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(r#""type":"image""#));
        assert!(json.contains("https://example.com/picture.png"));
    }

    #[test]
    fn extension_question_emits_external_slot_when_contribution_registered() {
        let node = render_try_question(&building_component_question(), &Map::new(), &building_component_contributions(), None, crate::editor::forms::terminology::forms_play_labels(&FormsConfig::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("externalSlot"));
        assert!(json.contains("forms-module-procedural"));
    }

    #[test]
    fn extension_question_falls_back_without_contribution() {
        let node = render_try_question(&building_component_question(), &Map::new(), &[], None, crate::editor::forms::terminology::forms_play_labels(&FormsConfig::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Extension unavailable"));
    }

    #[test]
    fn definition_declares_the_canvas2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, FORMS_PLAY_BODY_TRY);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    }
}
//#endregion 🧪️Tests
