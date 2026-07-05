//! 📋 Forms plugin — declarative forms play app bundled as a hot-swappable WASM component.

use forms::{
    empty_forms_projection, FormOp, FormQuestion, FormQuestionOption, FormSpec, FormStep,
    FormsEnvelope, FormsStore, FORMS_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{
    build_table_scene, create_default_layout, ui_inspector_groups_to_tree, ui_inspector_mixed_text,
    ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App,
    CommandDescriptor, PluginApp, PluginBundle, TableScene, UiControlNode, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiSelectItem, UiSelectNode, UiToggleNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use vcs::{create_document_vcs_envelope, materialize_document_projection, DocumentVcsCommand};

//#region 🔖Constants
const FORMS_PLAY_APP_ID: &str = "forms-play";
const FORMS_PLAY_CONTROLLER_ID: &str = "forms-play";
const FORMS_PLAY_SURFACE_EDIT: &str = "forms.play.edit";
const FORMS_PLAY_SURFACE_TRY: &str = "forms.play.try";
const FORMS_PLAY_BODY_EDIT: &str = "forms.play.edit";
const FORMS_PLAY_BODY_TRY: &str = "forms.play.try";
const FORMS_PLAY_BODY_HIERARCHY: &str = "forms.play.hierarchy";
const FORMS_PLAY_BODY_CATALOGUE: &str = "forms.play.catalogue";
const FORMS_PLAY_BODY_INSPECTION: &str = "forms.play.inspection";
const FORMS_PLAY_WINDOW_EDIT: &str = "forms-edit";
const FORMS_PLAY_WINDOW_TRY: &str = "forms-try";
const FORMS_QUESTION_DRAG_MIME: &str = "application/x-semio-forms-question-kind";
const BUILDING_COMPONENT_EXAMPLE_JSON: &str = include_str!("../../example/building-component.forms.json");

static FORM_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormsPlayEnvelope {
    envelope: FormsEnvelope,
    #[serde(default)]
    applied_edit_ids: Vec<String>,
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default)]
    try_values: HashMap<String, Value>,
}

fn default_envelope() -> FormsPlayEnvelope {
    let store = FormsStore::new(create_document_vcs_envelope(
        FORMS_DOCUMENT_SCHEMA,
        "forms-play",
        empty_forms_projection(),
        None,
    ));
    FormsPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_ids: Vec::new(),
        try_values: HashMap::new(),
    }
}

fn parse_envelope(document_json: &str) -> FormsPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &FormsPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn forms_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: FORMS_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn store_from_envelope(play: &FormsPlayEnvelope) -> FormsStore {
    let mut store = FormsStore::new(play.envelope.clone());
    store.set_envelope(play.envelope.clone(), play.applied_edit_ids.clone());
    store
}

fn sync_store_to_envelope(store: &FormsStore, play: &FormsPlayEnvelope) -> FormsPlayEnvelope {
    FormsPlayEnvelope {
        envelope: store.envelope().clone(),
        applied_edit_ids: store.applied_edit_ids().to_vec(),
        selected_ids: play.selected_ids.clone(),
        try_values: play.try_values.clone(),
    }
}

fn materialized_projection(play: &FormsPlayEnvelope) -> FormSpec {
    materialize_document_projection(&play.envelope, &play.applied_edit_ids)
        .unwrap_or_else(|_| play.envelope.vcs.initial_projection.clone())
}
//#endregion 🔖Envelope

//#region 🔖Helpers
struct QuestionLocation {
    step_id: String,
    question: FormQuestion,
}

fn create_form_id(prefix: &str) -> String {
    let serial = FORM_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{serial}")
}

fn forms_play_step_tree_id(step_id: &str) -> String {
    format!("step:{step_id}")
}

fn find_question_location(spec: &FormSpec, question_id: &str) -> Option<QuestionLocation> {
    for step in &spec.steps {
        if let Some(question) = step.questions.iter().find(|question| question.id == question_id) {
            return Some(QuestionLocation {
                step_id: step.id.clone(),
                question: question.clone(),
            });
        }
    }
    None
}

fn flatten_questions(spec: &FormSpec) -> Vec<(String, FormQuestion)> {
    spec.steps
        .iter()
        .flat_map(|step| step.questions.iter().map(|question| (step.title.clone(), question.clone())))
        .collect()
}

fn question_required(question_id: &str, play: &FormsPlayEnvelope) -> bool {
    play.try_values
        .get(&format!("__required:{question_id}"))
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

fn set_question_required(play: &mut FormsPlayEnvelope, question_id: &str, required: bool) {
    play.try_values
        .insert(format!("__required:{question_id}"), json!(required));
}

fn default_question_for_kind(kind: &str, id: String) -> FormQuestion {
    match kind {
        "text" => FormQuestion {
            id,
            label: "Text".into(),
            kind: "text".into(),
            default: None,
            text: None,
            options: None,
        },
        "longText" => FormQuestion {
            id,
            label: "Long Text".into(),
            kind: "longText".into(),
            default: None,
            text: None,
            options: None,
        },
        "number" => FormQuestion {
            id,
            label: "Number".into(),
            kind: "number".into(),
            default: Some(json!(0)),
            text: None,
            options: None,
        },
        "slider" => FormQuestion {
            id,
            label: "Slider".into(),
            kind: "slider".into(),
            default: Some(json!(50)),
            text: None,
            options: None,
        },
        "boolean" => FormQuestion {
            id,
            label: "Boolean".into(),
            kind: "boolean".into(),
            default: Some(json!(false)),
            text: None,
            options: None,
        },
        "single" | "multi" => FormQuestion {
            id,
            label: if kind == "single" { "Single Select" } else { "Multi Select" }.into(),
            kind: kind.into(),
            default: if kind == "multi" { Some(json!([])) } else { None },
            text: None,
            options: Some(vec![
                FormQuestionOption {
                    id: "a".into(),
                    label: "Option A".into(),
                },
                FormQuestionOption {
                    id: "b".into(),
                    label: "Option B".into(),
                },
            ]),
        },
        "note" => FormQuestion {
            id,
            label: "Note".into(),
            kind: "note".into(),
            default: None,
            text: Some("Informational note".into()),
            options: None,
        },
        "date" => FormQuestion {
            id,
            label: "Date".into(),
            kind: "date".into(),
            default: Some(json!("2026-01-01")),
            text: None,
            options: None,
        },
        "color" => FormQuestion {
            id,
            label: "Color".into(),
            kind: "color".into(),
            default: Some(json!("#336699")),
            text: None,
            options: None,
        },
        "image" => FormQuestion {
            id,
            label: "Image".into(),
            kind: "image".into(),
            default: None,
            text: None,
            options: None,
        },
        "file" => FormQuestion {
            id,
            label: "File".into(),
            kind: "file".into(),
            default: None,
            text: None,
            options: None,
        },
        "vector" => FormQuestion {
            id,
            label: "Vector".into(),
            kind: "vector".into(),
            default: Some(json!([0.0, 0.0, 0.0])),
            text: None,
            options: None,
        },
        "buildingComponent" => FormQuestion {
            id,
            label: "Building Component".into(),
            kind: "buildingComponent".into(),
            default: Some(json!({ "fixtureSlug": "hexagonal-mushroom-column", "params": { "height": 6 } })),
            text: None,
            options: None,
        },
        _ => FormQuestion {
            id,
            label: kind.into(),
            kind: kind.into(),
            default: None,
            text: None,
            options: None,
        },
    }
}

fn resolve_step_id_from_tree_target(spec: &FormSpec, target_id: &str) -> Option<String> {
    if target_id.starts_with("step:") {
        return Some(target_id[5..].to_string());
    }
    find_question_location(spec, target_id).map(|location| location.step_id)
}

fn resolve_question_insert_index(spec: &FormSpec, step_id: &str, target_id: &str, drop_position: &str) -> Option<usize> {
    let step = spec.steps.iter().find(|step| step.id == step_id)?;
    if target_id.starts_with("step:") {
        return Some(if drop_position == "before" { 0 } else { step.questions.len() });
    }
    let target_index = step.questions.iter().position(|question| question.id == target_id)?;
    Some(match drop_position {
        "before" => target_index,
        "after" => target_index + 1,
        _ => step.questions.len(),
    })
}

fn patch_question_field(play: &mut FormsPlayEnvelope, store: &mut FormsStore, question_id: &str, field: &str, raw_value: &Value) {
    let spec = store.projection().unwrap_or_else(|_| materialized_projection(play));
    let Some(location) = find_question_location(&spec, question_id) else {
        return;
    };
    if field == "required" {
        set_question_required(play, question_id, raw_value.as_bool().unwrap_or(false));
        return;
    }
    let mut question = location.question;
    match field {
        "label" => question.label = raw_value.as_str().unwrap_or("").to_string(),
        "kind" => question.kind = raw_value.as_str().unwrap_or("text").to_string(),
        "text" => question.text = raw_value.as_str().map(str::to_string),
        "default" => question.default = Some(raw_value.clone()),
        _ => {}
    }
    let _ = store.dispatch(DocumentVcsCommand::Apply {
        operations: vec![FormOp::UpdateQuestion {
            step_id: location.step_id,
            question,
        }],
        description: None,
    });
    play.try_values.clear();
}

fn apply_store_command(play: &mut FormsPlayEnvelope, store: &mut FormsStore) -> Vec<String> {
    *play = sync_store_to_envelope(store, play);
    vec![set_document_op(play)]
}

fn catalogue_kinds() -> [(&'static str, &'static str, &'static str); 14] {
    [
        ("text", "Text", "type"),
        ("number", "Number", "hash"),
        ("boolean", "Boolean", "toggle-left"),
        ("single", "Single Select", "circle-dot"),
        ("multi", "Multi Select", "list-checks"),
        ("slider", "Slider", "sliders-horizontal"),
        ("longText", "Long Text", "align-left"),
        ("note", "Note", "sticky-note"),
        ("date", "Date", "calendar"),
        ("color", "Color", "palette"),
        ("image", "Image", "image"),
        ("file", "File", "file"),
        ("vector", "Vector", "move-3d"),
        ("buildingComponent", "Building Component", "building"),
    ]
}
//#endregion 🔖Helpers

//#region 🔖Tables
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EditQuestionRow {
    id: String,
    label: String,
    kind: String,
    step: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TryQuestionRow {
    label: String,
}

fn edit_table_rows(spec: &FormSpec) -> String {
    let rows: Vec<EditQuestionRow> = flatten_questions(spec)
        .into_iter()
        .map(|(step, question)| EditQuestionRow {
            id: question.id.clone(),
            label: question.label.clone(),
            kind: question.kind.clone(),
            step,
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

fn try_table_rows(spec: &FormSpec) -> String {
    let rows: Vec<TryQuestionRow> = flatten_questions(spec)
        .into_iter()
        .map(|(_, question)| TryQuestionRow {
            label: question.label.clone(),
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

fn render_edit_table(spec: &FormSpec) -> UiNode {
    build_table_scene(
        FORMS_PLAY_SURFACE_EDIT,
        FORMS_PLAY_CONTROLLER_ID,
        TableScene {
            columns_json: json!([
                {"id":"id","label":"Id"},
                {"id":"label","label":"Label"},
                {"id":"kind","label":"Kind"},
                {"id":"step","label":"Step"}
            ])
            .to_string(),
            rows_json: edit_table_rows(spec),
        },
    )
}

fn render_try_table(spec: &FormSpec) -> UiNode {
    build_table_scene(
        FORMS_PLAY_SURFACE_TRY,
        FORMS_PLAY_CONTROLLER_ID,
        TableScene {
            columns_json: json!([{"id":"label","label":"Label"}]).to_string(),
            rows_json: try_table_rows(spec),
        },
    )
}
//#endregion 🔖Tables

//#region 🔖Panels
fn build_hierarchy_tree(spec: &FormSpec, selected_ids: &[String]) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = spec
        .steps
        .iter()
        .map(|step| UiTreeItemNode {
            id: forms_play_step_tree_id(&step.id),
            label: step.title.clone(),
            description: Some(format!("{} questions", step.questions.len())),
            icon_id: Some("list-tree".into()),
            selected: None,
            default_open: Some(true),
            command: Some(forms_cmd("setSelection", Some(json!({ "ids": [] })))),
            hover_command: None,
            unhover_command: None,
            actions: None,
            draggable: Some(true),
            drag_data: None,
            items: Some(
                step.questions
                    .iter()
                    .map(|question| UiTreeItemNode {
                        id: question.id.clone(),
                        label: question.label.clone(),
                        description: Some(question.kind.clone()),
                        icon_id: Some("help-circle".into()),
                        selected: None,
                        default_open: None,
                        command: Some(forms_cmd("setSelection", Some(json!({ "ids": [question.id.clone()] })))),
                        hover_command: None,
                        unhover_command: None,
                        actions: None,
                        draggable: Some(true),
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    })
                    .collect(),
            ),
            control: None,
            is_hidden: None,
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "forms-play-hierarchy.steps".into(),
            label: Some(FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL.into()),
            default_open: Some(true),
            items: if step_items.is_empty() {
                vec![UiTreeItemNode {
                    id: "forms-play-hierarchy.empty".into(),
                    label: "(no steps)".into(),
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
                step_items
            },
        }],
        selected_ids: Some(selected_ids.to_vec()),
        highlighted_ids: None,
        selection_change: Some(forms_cmd("setSelection", None)),
    })
}

fn build_catalogue_tree() -> UiNode {
    let kind_items: Vec<UiTreeItemNode> = catalogue_kinds()
        .into_iter()
        .map(|(kind, label, icon)| {
            let mut drag_data = HashMap::new();
            drag_data.insert(FORMS_QUESTION_DRAG_MIME.into(), json!({ "kind": kind }).to_string());
            UiTreeItemNode {
                id: format!("forms-play-catalogue.{kind}"),
                label: label.into(),
                description: Some(kind.into()),
                icon_id: Some(icon.into()),
                selected: None,
                default_open: None,
                command: Some(forms_cmd("addQuestion", Some(json!({ "kind": kind })))),
                hover_command: None,
                unhover_command: None,
                actions: None,
                draggable: Some(true),
                drag_data: Some(drag_data),
                items: None,
                control: None,
                is_hidden: None,
            }
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "forms-play-catalogue.kinds".into(),
                label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
                default_open: Some(true),
                items: kind_items,
            },
            UiTreeSectionNode {
                id: "forms-play-catalogue.actions".into(),
                label: Some("Actions".into()),
                default_open: Some(true),
                items: vec![
                    UiTreeItemNode {
                        id: "forms-play-catalogue.add-step".into(),
                        label: "Add Step".into(),
                        description: None,
                        icon_id: Some("plus".into()),
                        selected: None,
                        default_open: None,
                        command: Some(forms_cmd("addStep", None)),
                        hover_command: None,
                        unhover_command: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    },
                    UiTreeItemNode {
                        id: "forms-play-catalogue.add-question".into(),
                        label: "Add Text Question".into(),
                        description: None,
                        icon_id: Some("type".into()),
                        selected: None,
                        default_open: None,
                        command: Some(forms_cmd("addQuestion", Some(json!({ "kind": "text" })))),
                        hover_command: None,
                        unhover_command: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    },
                ],
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn inspector_patch(question_ids: &[String], field: &str) -> CommandDescriptor {
    forms_cmd("patchQuestions", Some(json!({ "questionIds": question_ids, "field": field })))
}

fn inspector_text_field(question_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: UiControlNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: None,
            on_change: inspector_patch(question_ids, field),
        }),
    })
}

fn build_inspector_tree(spec: &FormSpec, play: &FormsPlayEnvelope) -> UiNode {
    let questions: Vec<FormQuestion> = play
        .selected_ids
        .iter()
        .filter_map(|id| find_question_location(spec, id).map(|location| location.question))
        .collect();
    if questions.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {FORMS_DOCUMENT_SCHEMA}")),
            ui_text(format!("Steps: {}", spec.steps.len())),
            ui_text(format!("Questions: {}", flatten_questions(spec).len())),
        ]);
    }
    let question_ids: Vec<String> = questions.iter().map(|question| question.id.clone()).collect();
    let labels: Vec<String> = questions.iter().map(|question| question.label.clone()).collect();
    let kinds: Vec<String> = questions.iter().map(|question| question.kind.clone()).collect();
    let required: Vec<bool> = questions
        .iter()
        .map(|question| question_required(&question.id, play))
        .collect();
    let kind_mixed = ui_inspector_mixed_text(&kinds);
    let required_mixed = ui_inspector_mixed_toggle(&required);
    let kind_items: Vec<UiSelectItem> = catalogue_kinds()
        .into_iter()
        .map(|(kind, label, _)| UiSelectItem {
            value: kind.into(),
            label: label.into(),
        })
        .collect();
    let groups = vec![UiInspectorFieldGroup {
        id: "forms-play-inspector.base".into(),
        label: "Question".into(),
        default_open: None,
        fields: vec![
            inspector_text_field(&question_ids, "forms-play-inspector.label", "Label", &labels, "label"),
            UiNode::Field(UiFieldNode {
                id: "forms-play-inspector.kind".into(),
                label: "Kind".into(),
                child: UiControlNode::Select(UiSelectNode {
                    id: "forms-play-inspector.kind.select".into(),
                    value: kind_mixed.value,
                    placeholder: kind_mixed.placeholder,
                    items: kind_items,
                    on_change: inspector_patch(&question_ids, "kind"),
                }),
            }),
            ui_inspector_readonly_field(
                "forms-play-inspector.id",
                "Id",
                if question_ids.len() == 1 {
                    question_ids[0].clone()
                } else {
                    format!("{} selected", question_ids.len())
                },
            ),
            UiNode::Field(UiFieldNode {
                id: "forms-play-inspector.required".into(),
                label: "Required".into(),
                child: UiControlNode::Toggle(UiToggleNode {
                    id: "forms-play-inspector.required.toggle".into(),
                    icon_id: "check".into(),
                    pressed: required_mixed.uniform && required_mixed.pressed,
                    text: if required_mixed.uniform {
                        Some(if required_mixed.pressed { "Yes".into() } else { "No".into() })
                    } else {
                        Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                    },
                    on_change: inspector_patch(&question_ids, "required"),
                }),
            }),
        ],
    }];
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖Panels

//#region 🔖FormsPlayApp
struct FormsPlayApp;

impl PluginApp for FormsPlayApp {
    fn app_id(&self) -> &str {
        FORMS_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("forms envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut play = parse_envelope(document_json);
        let mut store = store_from_envelope(&play);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value::<FormsPlayEnvelope>(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()) {
                    play.selected_ids = ids
                        .iter()
                        .filter_map(|value| value.as_str().map(str::to_string))
                        .collect();
                    return vec![set_document_op(&play)];
                }
            }
            "addStep" => {
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let step = FormStep {
                    id: create_form_id("step"),
                    title: format!("Step {}", projection.steps.len() + 1),
                    description: None,
                    questions: Vec::new(),
                };
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::AddStep { step, index: None }],
                    description: None,
                });
                play.try_values.clear();
                return apply_store_command(&mut play, &mut store);
            }
            "addQuestion" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("text");
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let step_id = args
                    .and_then(|value| value.get("stepId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| projection.steps.first().map(|step| step.id.clone()));
                let Some(step_id) = step_id else {
                    return Vec::new();
                };
                let question = default_question_for_kind(kind, create_form_id("q"));
                let select_id = question.id.clone();
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::AddQuestion {
                        step_id,
                        question,
                        index: None,
                    }],
                    description: None,
                });
                play.try_values.clear();
                play.selected_ids = vec![select_id];
                return apply_store_command(&mut play, &mut store);
            }
            "removeQuestion" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str()).unwrap_or("");
                if question_id.is_empty() {
                    return Vec::new();
                }
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let Some(location) = find_question_location(&projection, question_id) else {
                    return Vec::new();
                };
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::RemoveQuestion {
                        step_id: location.step_id,
                        question_id: question_id.into(),
                    }],
                    description: None,
                });
                play.selected_ids.retain(|id| id != question_id);
                play.try_values.clear();
                return apply_store_command(&mut play, &mut store);
            }
            "patchQuestions" => {
                let question_ids: Vec<String> = args
                    .and_then(|value| value.get("questionIds"))
                    .and_then(|value| value.as_array())
                    .map(|values| values.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("pressed")))
                    .cloned()
                    .unwrap_or(Value::Null);
                if question_ids.is_empty() || field.is_empty() {
                    return Vec::new();
                }
                for question_id in question_ids {
                    patch_question_field(&mut play, &mut store, &question_id, field, &raw_value);
                }
                return apply_store_command(&mut play, &mut store);
            }
            "moveQuestion" => {
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str());
                let to_step_id = args.and_then(|value| value.get("toStepId")).and_then(|value| value.as_str());
                let target_id = args
                    .and_then(|value| value.get("targetId"))
                    .and_then(|value| value.as_str());
                let position = args
                    .and_then(|value| value.get("position"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("inside");
                let (Some(question_id), Some(to_step_id)) = (question_id, to_step_id) else {
                    return Vec::new();
                };
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let Some(source) = find_question_location(&projection, question_id) else {
                    return Vec::new();
                };
                let target_id = target_id.unwrap_or(question_id);
                let index = resolve_question_insert_index(&projection, to_step_id, target_id, position).unwrap_or(0);
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::MoveQuestion {
                        question_id: question_id.into(),
                        from_step_id: source.step_id,
                        to_step_id: to_step_id.into(),
                        index,
                    }],
                    description: None,
                });
                play.try_values.clear();
                return apply_store_command(&mut play, &mut store);
            }
            "dropQuestionKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
                let target_id = args.and_then(|value| value.get("targetId")).and_then(|value| value.as_str());
                let drop_position = args
                    .and_then(|value| value.get("dropPosition"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("inside");
                let (Some(kind), Some(target_id)) = (kind, target_id) else {
                    return Vec::new();
                };
                let projection = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let Some(step_id) = resolve_step_id_from_tree_target(&projection, target_id) else {
                    return Vec::new();
                };
                let index = resolve_question_insert_index(&projection, &step_id, target_id, drop_position);
                let question = default_question_for_kind(kind, create_form_id("q"));
                let select_id = question.id.clone();
                let _ = store.dispatch(DocumentVcsCommand::Apply {
                    operations: vec![FormOp::AddQuestion {
                        step_id,
                        question,
                        index,
                    }],
                    description: None,
                });
                play.try_values.clear();
                play.selected_ids = vec![select_id];
                return apply_store_command(&mut play, &mut store);
            }
            "undo" => {
                let _ = store.dispatch(DocumentVcsCommand::Undo);
                play.try_values.clear();
                return apply_store_command(&mut play, &mut store);
            }
            "redo" => {
                let _ = store.dispatch(DocumentVcsCommand::Redo);
                play.try_values.clear();
                return apply_store_command(&mut play, &mut store);
            }
            "exportFixture" => {
                let spec = store.projection().unwrap_or_else(|_| materialized_projection(&play));
                let _ = serde_json::to_string_pretty(&spec).unwrap_or_default();
                return Vec::new();
            }
            "setSpecJson" | "editEngagementInput" | "tryEngagementInput" => {}
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let json_text = match example_id {
                    "building-component" => BUILDING_COMPONENT_EXAMPLE_JSON,
                    _ => return Vec::new(),
                };
                let spec: FormSpec = serde_json::from_str(json_text).unwrap_or_else(|_| materialized_projection(&play));
                let document_id = spec.id.clone();
                let envelope = create_document_vcs_envelope(FORMS_DOCUMENT_SCHEMA, &document_id, spec, None);
                store = FormsStore::new(envelope);
                play.try_values.clear();
                play.selected_ids.clear();
                return apply_store_command(&mut play, &mut store);
            }
            "setTryValues" => {
                if let Some(values) = args.and_then(|value| value.get("values")).and_then(|value| value.as_object()) {
                    for (key, value) in values {
                        play.try_values.insert(key.clone(), value.clone());
                    }
                    return vec![set_document_op(&play)];
                }
            }
            "resetTry" => {
                play.try_values.clear();
                return vec![set_document_op(&play)];
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let play = parse_envelope(document_json);
        let spec = materialized_projection(&play);
        match body_key {
            FORMS_PLAY_BODY_EDIT => render_edit_table(&spec),
            FORMS_PLAY_BODY_TRY => render_try_table(&spec),
            FORMS_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&spec, &play.selected_ids),
            FORMS_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            FORMS_PLAY_BODY_INSPECTION => build_inspector_tree(&spec, &play),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖FormsPlayApp

//#region 🔖AppFactory
fn create_forms_app() -> App {
    App::from_builder(
        App::builder(FORMS_PLAY_APP_ID, "Forms")
            .icon_id("forms")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(FORMS_PLAY_WINDOW_EDIT, "Edit", FORMS_PLAY_BODY_EDIT)
            .window_kind(FORMS_PLAY_WINDOW_TRY, "Try", FORMS_PLAY_BODY_TRY)
            .panel_tab("framework.panel.hierarchy", "Hierarchy", "workbench", FORMS_PLAY_BODY_HIERARCHY)
            .panel_tab("framework.panel.catalogue", "Catalogue", "workbench", FORMS_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", "Inspection", "details", FORMS_PLAY_BODY_INSPECTION)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .default_layout(create_default_layout(
                &[FORMS_PLAY_WINDOW_EDIT.into(), FORMS_PLAY_WINDOW_TRY.into()],
                "row",
                Some(&[58.0, 42.0]),
                Some(&["Edit".into(), "Try".into()]),
            )),
    )
    .example("empty", "Empty", serde_json::to_string(&default_envelope()).unwrap())
    .example("building-component", "Building Component", BUILDING_COMPONENT_EXAMPLE_JSON)
    .program("forms", "Forms", "data")
}

fn forms_bundle() -> PluginBundle {
    PluginBundle::new("forms", "Forms", "0.1.0").register_app(create_forms_app(), || Box::new(FormsPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(forms_bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use forms::apply_form_edit_op;
    use semio_framework_plugin::PluginApp;

    fn apply_ops(document_json: &str, ops: &[String]) -> FormsPlayEnvelope {
        let mut play = parse_envelope(document_json);
        for op_json in ops {
            if let Ok(op) = serde_json::from_str::<Value>(op_json) {
                if op.get("op").and_then(|value| value.as_str()) == Some("setDocument") {
                    if let Some(document) = op.get("document") {
                        if let Ok(parsed) = serde_json::from_value(document.clone()) {
                            play = parsed;
                        }
                    }
                }
            }
        }
        play
    }

    #[test]
    fn renders_edit_table() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(FORMS_PLAY_BODY_EDIT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("table"));
    }

    #[test]
    fn hierarchy_lists_steps() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(FORMS_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("forms-play-hierarchy.steps"));
        assert!(json.contains("Inputs"));
    }

    #[test]
    fn catalogue_lists_question_kinds() {
        let app = FormsPlayApp;
        let document = app.initial_document_json();
        let node = app.render(FORMS_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("forms-play-catalogue.text"));
        assert!(json.contains("forms-play-catalogue.add-step"));
    }

    #[test]
    fn add_step_command_appends_step() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let before = materialized_projection(&parse_envelope(&document)).steps.len();
        let ops = app.handle_command("addStep", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let next = apply_ops(&document, &ops);
        assert_eq!(materialized_projection(&next).steps.len(), before + 1);
    }

    #[test]
    fn add_question_command_appends_question() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "addQuestion",
            Some(&json!({ "kind": "text" })),
            &document,
            &ViewState::default(),
        );
        let next = apply_ops(&document, &ops);
        assert!(flatten_questions(&materialized_projection(&next))
            .iter()
            .any(|(_, question)| question.kind == "text"));
    }

    #[test]
    fn set_try_values_updates_runtime() {
        let mut app = FormsPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command(
            "setTryValues",
            Some(&json!({ "values": { "q-text": "Ada" } })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let next = apply_ops(&document, &ops);
        assert_eq!(next.try_values.get("q-text").and_then(|v| v.as_str()), Some("Ada"));
    }

    #[test]
    fn apply_form_edit_op_roundtrip() {
        let spec = empty_forms_projection();
        let step = FormStep {
            id: "step-test".into(),
            title: "Review".into(),
            description: None,
            questions: Vec::new(),
        };
        let next = apply_form_edit_op(&spec, &FormOp::AddStep { step, index: None });
        assert_eq!(next.steps.len(), 2);
    }
}
//#endregion 🧪Tests
