//! 🧬 Shared Generate mode state, CRUD, and declarative UI helpers.

use forms::{default_value_for_question, flatten_form_questions, is_question_visible, FormQuestion, FormSpec};
use semio_framework_core::{
    build_text_editor_scene, ui_stack_vertical, ui_text, CommandDescriptor, TextEditorScene, UiControlNode,
    UiFieldNode, UiInputNode, UiNode, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, UiTreeItemAction,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//#region 🔖Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormGeneration {
    pub id: String,
    pub name: String,
    pub values: Map<String, Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationPlayState {
    #[serde(default)]
    pub generations: Vec<FormGeneration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_generation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
}
//#endregion 🔖Types

//#region 🔖Crud
fn next_generation_id(generations: &[FormGeneration]) -> String {
    format!("generation-{}", generations.len() + 1)
}

fn next_generation_name(generations: &[FormGeneration]) -> String {
    format!("Generation {}", generations.len() + 1)
}

pub fn initial_generation_values(spec: &FormSpec) -> Map<String, Value> {
    let mut values = Map::new();
    for question in flatten_form_questions(spec) {
        values.insert(question.id.clone(), default_value_for_question(question));
    }
    values
}

pub fn add_generation(state: &mut GenerationPlayState, spec: &FormSpec) -> String {
    let id = next_generation_id(&state.generations);
    let name = next_generation_name(&state.generations);
    state.generations.push(FormGeneration {
        id: id.clone(),
        name,
        values: initial_generation_values(spec),
    });
    state.selected_generation_id = Some(id.clone());
    id
}

pub fn remove_generation(state: &mut GenerationPlayState, generation_id: &str) {
    state.generations.retain(|entry| entry.id != generation_id);
    if state.selected_generation_id.as_deref() == Some(generation_id) {
        state.selected_generation_id = state.generations.first().map(|entry| entry.id.clone());
    }
}

pub fn rename_generation(state: &mut GenerationPlayState, generation_id: &str, name: &str) {
    if let Some(entry) = state.generations.iter_mut().find(|entry| entry.id == generation_id) {
        entry.name = name.to_string();
    }
}

pub fn select_generation(state: &mut GenerationPlayState, generation_id: &str) {
    if state.generations.iter().any(|entry| entry.id == generation_id) {
        state.selected_generation_id = Some(generation_id.to_string());
    }
}

pub fn selected_generation<'a>(state: &'a GenerationPlayState) -> Option<&'a FormGeneration> {
    let selected_id = state.selected_generation_id.as_deref()?;
    state.generations.iter().find(|entry| entry.id == selected_id)
}

pub fn selected_generation_mut<'a>(state: &'a mut GenerationPlayState) -> Option<&'a mut FormGeneration> {
    let selected_id = state.selected_generation_id.clone()?;
    state.generations.iter_mut().find(|entry| entry.id == selected_id)
}

pub fn update_generation_values(
    state: &mut GenerationPlayState,
    generation_id: &str,
    question_id: &str,
    value: Value,
) {
    if let Some(entry) = state.generations.iter_mut().find(|entry| entry.id == generation_id) {
        entry.values.insert(question_id.to_string(), value);
    }
}

pub fn handle_generation_command(
    command: &str,
    args: Option<&Value>,
    state: &mut GenerationPlayState,
    spec: &FormSpec,
    controller_id: &str,
) -> bool {
    match command {
        "addGeneration" => {
            add_generation(state, spec);
            true
        }
        "removeGeneration" => {
            if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                remove_generation(state, id);
            }
            true
        }
        "selectGeneration" => {
            if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                select_generation(state, id);
            }
            true
        }
        "renameGeneration" => {
            let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
            let name = args.and_then(|value| value.get("name")).and_then(|value| value.as_str());
            if let (Some(id), Some(name)) = (id, name) {
                rename_generation(state, id, name);
            }
            true
        }
        "updateGenerationValues" => {
            let generation_id = args
                .and_then(|value| value.get("generationId"))
                .and_then(|value| value.as_str())
                .map(str::to_string)
                .or_else(|| state.selected_generation_id.clone());
            let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str());
            let value = args.and_then(|value| value.get("value"));
            if let (Some(generation_id), Some(question_id), Some(value)) = (generation_id, question_id, value) {
                update_generation_values(state, &generation_id, question_id, value.clone());
            }
            let _ = controller_id;
            true
        }
        _ => false,
    }
}
//#endregion 🔖Crud

//#region 🔖Render
fn generation_cmd(controller_id: &str, command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: controller_id.into(),
        command: command.into(),
        args,
    }
}

pub fn render_generations_tree(
    controller_id: &str,
    surface_prefix: &str,
    generations: &[FormGeneration],
    selected_id: Option<&str>,
) -> UiNode {
    let items: Vec<UiTreeItemNode> = generations
        .iter()
        .map(|generation| {
            let mut actions = vec![UiTreeItemAction {
                icon_id: "trash-2".into(),
                label: Some("Remove".into()),
                command: generation_cmd(
                    controller_id,
                    "removeGeneration",
                    Some(json!({ "id": generation.id })),
                ),
                reveal_on_hover: Some(true),
            }];
            actions.insert(
                0,
                UiTreeItemAction {
                    icon_id: "pencil".into(),
                    label: Some("Rename".into()),
                    command: generation_cmd(
                        controller_id,
                        "renameGeneration",
                        Some(json!({ "id": generation.id, "name": format!("{} copy", generation.name) })),
                    ),
                    reveal_on_hover: Some(true),
                },
            );
            UiTreeItemNode {
                id: format!("{surface_prefix}.generation.{}", generation.id),
                label: generation.name.clone(),
                description: Some(format!("{} values", generation.values.len())),
                icon_id: Some("layers".into()),
                selected: Some(selected_id == Some(generation.id.as_str())),
                default_open: None,
                command: Some(generation_cmd(
                    controller_id,
                    "selectGeneration",
                    Some(json!({ "id": generation.id })),
                )),
                hover_command: None,
                unhover_command: None,
                actions: Some(actions),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
            }
        })
        .collect();
    let mut sections = vec![UiTreeSectionNode {
        id: format!("{surface_prefix}.generations"),
        label: Some("Generations".into()),
        default_open: Some(true),
        items: if items.is_empty() {
            vec![UiTreeItemNode {
                id: format!("{surface_prefix}.generations.empty"),
                label: "(no generations)".into(),
                description: None,
                icon_id: None,
                selected: None,
                default_open: None,
                command: None,
                hover_command: None,
                unhover_command: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
            }]
        } else {
            items
        },
    }];
    sections.push(UiTreeSectionNode {
        id: format!("{surface_prefix}.actions"),
        label: Some("Actions".into()),
        default_open: Some(true),
        items: vec![UiTreeItemNode {
            id: format!("{surface_prefix}.add-generation"),
            label: "Add Generation".into(),
            description: None,
            icon_id: Some("plus".into()),
            selected: None,
            default_open: None,
            command: Some(generation_cmd(controller_id, "addGeneration", None)),
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        }],
    });
    UiNode::Tree(UiTreeNode {
        sections,
        selected_ids: selected_id.map(|id| vec![format!("{surface_prefix}.generation.{id}")]),
        highlighted_ids: None,
        selection_change: Some(generation_cmd(controller_id, "selectGeneration", None)),
    })
}

fn render_question_field(
    question: &FormQuestion,
    values: &Map<String, Value>,
    controller_id: &str,
    patch_command: &str,
    generation_id: &str,
) -> Option<UiNode> {
    if !is_question_visible(question, values) {
        return None;
    }
    let value = values
        .get(&question.id)
        .cloned()
        .unwrap_or_else(|| default_value_for_question(question));
    let field_id = format!("generate.form.{}", question.id);
    let on_change = || {
        generation_cmd(
            controller_id,
            patch_command,
            Some(json!({
                "generationId": generation_id,
                "questionId": question.id,
            })),
        )
    };
    let child = match question.kind.as_str() {
        "text" | "longText" => UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: if question.kind == "longText" { "textarea".into() } else { "text".into() },
            value: value.as_str().unwrap_or_default().to_string(),
            placeholder: question.placeholder.clone(),
            commit: None,
            on_change: on_change(),
        }),
        "number" => UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: value.as_f64().map(|number| number.to_string()).unwrap_or_default(),
            placeholder: question.placeholder.clone(),
            commit: None,
            on_change: on_change(),
        }),
        "slider" => UiControlNode::Slider(UiSliderNode {
            id: format!("{field_id}.slider"),
            value: value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0)),
            min: question.min.unwrap_or(0.0),
            max: question.max.unwrap_or(100.0),
            step: question.step.unwrap_or(1.0),
            on_change: on_change(),
        }),
        "boolean" => UiControlNode::Toggle(UiToggleNode {
            id: format!("{field_id}.toggle"),
            icon_id: "toggle-left".into(),
            pressed: value.as_bool().unwrap_or(false),
            text: Some(question.label.clone()),
            on_change: on_change(),
        }),
        "single" => {
            let items = question
                .options
                .as_ref()
                .map(|options| {
                    options
                        .iter()
                        .map(|option| UiSelectItem {
                            value: option.value.clone(),
                            label: option.label.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default();
            UiControlNode::Select(UiSelectNode {
                id: format!("{field_id}.select"),
                value: value.as_str().unwrap_or_default().to_string(),
                items,
                placeholder: question.placeholder.clone(),
                on_change: on_change(),
            })
        }
        "vector" => {
            let numbers = value
                .as_array()
                .cloned()
                .unwrap_or_else(|| {
                    question
                        .fields
                        .as_ref()
                        .map(|fields| fields.iter().map(|field| json!(field.value.unwrap_or(0.0))).collect())
                        .unwrap_or_default()
                });
            let labels: Vec<String> = question
                .fields
                .as_ref()
                .map(|fields| {
                    fields
                        .iter()
                        .map(|field| field.label.clone().unwrap_or_else(|| field.key.clone()))
                        .collect()
                })
                .unwrap_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect());
            let children: Vec<UiNode> = numbers
                .iter()
                .enumerate()
                .map(|(index, number)| {
                    let label = labels.get(index).cloned().unwrap_or_else(|| format!("Field {}", index + 1));
                    UiNode::Field(UiFieldNode {
                        id: format!("{field_id}.vector.{index}"),
                        label,
                        child: UiControlNode::Input(UiInputNode {
                            id: format!("{field_id}.vector.{index}.input"),
                            input_kind: "number".into(),
                            value: number.as_f64().map(|entry| entry.to_string()).unwrap_or_default(),
                            placeholder: None,
                            commit: None,
                            on_change: generation_cmd(
                                controller_id,
                                patch_command,
                                Some(json!({
                                    "generationId": generation_id,
                                    "questionId": question.id,
                                    "fieldIndex": index,
                                })),
                            ),
                        }),
                    })
                })
                .collect();
            return Some(ui_stack_vertical(children));
        }
        "note" => return Some(ui_text(question.text.clone().unwrap_or_default())),
        "image" => return Some(ui_text(question.src.clone().unwrap_or_else(|| "(no image)".into()))),
        _ => UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: value.to_string(),
            placeholder: question.placeholder.clone(),
            commit: None,
            on_change: on_change(),
        }),
    };
    Some(UiNode::Field(UiFieldNode {
        id: field_id,
        label: question.label.clone(),
        child,
    }))
}

pub fn render_generation_form_body(
    form_spec: &FormSpec,
    values: &Map<String, Value>,
    controller_id: &str,
    patch_command: &str,
    generation_id: &str,
) -> UiNode {
    let mut children = Vec::new();
    for step in &form_spec.steps {
        if !step.questions.is_empty() {
            children.push(ui_text(step.title.clone()));
        }
        for question in &step.questions {
            if let Some(field) = render_question_field(question, values, controller_id, patch_command, generation_id) {
                children.push(field);
            }
        }
    }
    if children.is_empty() {
        return ui_text("No input widgets to generate from.");
    }
    ui_stack_vertical(children)
}

pub fn render_generation_preview_text(surface: &str, controller_id: &str, text: &str) -> UiNode {
    build_text_editor_scene(
        surface,
        controller_id,
        TextEditorScene::base(text.to_string(), Some("json".into()), None),
    )
}
//#endregion 🔖Render

#[cfg(test)]
mod tests {
    use super::*;
    use forms::{FormQuestion, FormStep, FORMS_DOCUMENT_SCHEMA};

    fn sample_spec() -> FormSpec {
        FormSpec {
            schema: FORMS_DOCUMENT_SCHEMA.into(),
            id: "sample".into(),
            version: "1".into(),
            title: None,
            steps: vec![FormStep {
                id: "s".into(),
                title: "Inputs".into(),
                description: None,
                questions: vec![FormQuestion {
                    id: "width".into(),
                    label: "Width".into(),
                    kind: "slider".into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(json!(1.0)),
                    min: Some(0.0),
                    max: Some(10.0),
                    step: Some(0.5),
                    unit: None,
                    text: None,
                    options: None,
                    fields: None,
                    schema: None,
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: None,
                }],
            }],
        }
    }

    #[test]
    fn generation_crud_round_trip() {
        let spec = sample_spec();
        let mut state = GenerationPlayState::default();
        let id = add_generation(&mut state, &spec);
        assert_eq!(state.generations.len(), 1);
        rename_generation(&mut state, &id, "Variant A");
        update_generation_values(&mut state, &id, "width", json!(4.0));
        assert_eq!(selected_generation(&state).unwrap().name, "Variant A");
        remove_generation(&mut state, &id);
        assert!(state.generations.is_empty());
    }

    #[test]
    fn render_generations_tree_contains_add_action() {
        let json = serde_json::to_string(&render_generations_tree(
            "flow-play",
            "flow-generate",
            &[],
            None,
        ))
        .unwrap();
        assert!(json.contains("addGeneration"));
    }
}
