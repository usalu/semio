//! 🔌 Declarative app plugin SDK — build fully declarative Rust apps bundled into hot-swappable WASM components.

pub mod app {
// #region app
//! 🧩 Declarative app builder and plugin trait.

use semio_framework_core::{
    AppDefinition, CommandDescriptor, ExampleDefinition, Keybinding, ModeDefinition,
    NamedLayout, PanelTabDefinition, PluginManifest, ProgramDefinition, ToolNode, UiNode, ViewState,
    WindowEngagement, WindowKindDefinition, WindowLayout, WindowMeasure,
};
use serde_json::Value;
use std::collections::HashMap;

pub struct ModeSpec {
    pub id: String,
    pub label: String,
    pub tools: Vec<ToolNode>,
}

pub struct WindowKindSpec {
    pub id: String,
    pub label: String,
    pub body_key: String,
    pub icon_id: Option<String>,
    pub measures: Vec<WindowMeasure>,
    pub engagement: Option<WindowEngagement>,
}

pub struct PanelTabSpec {
    pub id: String,
    pub label: String,
    pub group: String,
    pub body_key: String,
}

pub struct KeybindingSpec {
    pub keys: String,
    pub controller_id: String,
    pub command: String,
}

pub struct AppBuilder {
    id: String,
    label: String,
    document: Vec<String>,
    icon_id: Option<String>,
    controller_id: String,
    modes: Vec<ModeSpec>,
    default_mode_id: Option<String>,
    window_kinds: Vec<WindowKindSpec>,
    panel_tabs: Vec<PanelTabSpec>,
    keybindings: Vec<KeybindingSpec>,
    named_layouts: Vec<NamedLayout>,
    default_layout: Option<WindowLayout>,
}

impl AppBuilder {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            controller_id: id.clone(),
            id,
            label: label.into(),
            document: Vec::new(),
            icon_id: None,
            modes: Vec::new(),
            default_mode_id: None,
            window_kinds: Vec::new(),
            panel_tabs: Vec::new(),
            keybindings: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
        }
    }

    pub fn icon_id(mut self, icon_id: impl Into<String>) -> Self {
        self.icon_id = Some(icon_id.into());
        self
    }

    pub fn document<I, S>(mut self, document: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.document = document.into_iter().map(Into::into).collect();
        self
    }

    pub fn mode(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.modes.push(ModeSpec {
            id: id.into(),
            label: label.into(),
            tools: Vec::new(),
        });
        self
    }

    pub fn mode_tools(mut self, mode_id: impl AsRef<str>, tools: Vec<ToolNode>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.tools = tools;
        }
        self
    }

    pub fn default_mode_id(mut self, id: impl Into<String>) -> Self {
        self.default_mode_id = Some(id.into());
        self
    }

    pub fn window_kind(mut self, id: impl Into<String>, label: impl Into<String>, body_key: impl Into<String>) -> Self {
        self.window_kinds.push(WindowKindSpec {
            id: id.into(),
            label: label.into(),
            body_key: body_key.into(),
            icon_id: None,
            measures: Vec::new(),
            engagement: None,
        });
        self
    }

    pub fn window_kind_with_engagement(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        body_key: impl Into<String>,
        engagement: WindowEngagement,
    ) -> Self {
        self.window_kinds.push(WindowKindSpec {
            id: id.into(),
            label: label.into(),
            body_key: body_key.into(),
            icon_id: None,
            measures: Vec::new(),
            engagement: Some(engagement),
        });
        self
    }

    pub fn named_layout(mut self, layout: NamedLayout) -> Self {
        self.named_layouts.push(layout);
        self
    }

    pub fn default_layout(mut self, layout: WindowLayout) -> Self {
        self.default_layout = Some(layout);
        self
    }

    pub fn panel_tab(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        group: impl Into<String>,
        body_key: impl Into<String>,
    ) -> Self {
        self.panel_tabs.push(PanelTabSpec {
            id: id.into(),
            label: label.into(),
            group: group.into(),
            body_key: body_key.into(),
        });
        self
    }

    pub fn keybinding(mut self, keys: impl Into<String>, command: impl Into<String>) -> Self {
        self.keybindings.push(KeybindingSpec {
            keys: keys.into(),
            controller_id: self.controller_id.clone(),
            command: command.into(),
        });
        self
    }

    pub fn build_definition(self) -> AppDefinition {
        assert!(
            !self.document.is_empty() && self.document.iter().all(|segment| !segment.trim().is_empty()),
            "app document must contain non-empty segments"
        );
        let default_mode_id = self
            .default_mode_id
            .or_else(|| self.modes.first().map(|mode| mode.id.clone()));
        AppDefinition {
            id: self.id,
            label: self.label,
            document: self.document,
            icon_id: self.icon_id,
            controller_id: self.controller_id,
            modes: self
                .modes
                .into_iter()
                .map(|mode| ModeDefinition {
                    id: mode.id,
                    label: mode.label,
                    tools: mode.tools,
                })
                .collect(),
            default_mode_id,
            window_kinds: self
                .window_kinds
                .into_iter()
                .map(|window| WindowKindDefinition {
                    id: window.id,
                    label: window.label,
                    body_key: window.body_key,
                    icon_id: window.icon_id,
                    measures: window.measures,
                    engagement: window.engagement,
                })
                .collect(),
            panel_tabs: self
                .panel_tabs
                .into_iter()
                .map(|tab| PanelTabDefinition {
                    id: tab.id,
                    label: tab.label,
                    group: tab.group,
                    body_key: tab.body_key,
                })
                .collect(),
            keybindings: self
                .keybindings
                .into_iter()
                .map(|binding| Keybinding {
                    keys: binding.keys,
                    command: CommandDescriptor {
                        controller_id: binding.controller_id,
                        command: binding.command,
                        args: None,
                    },
                })
                .collect(),
            named_layouts: self.named_layouts,
            default_layout: self.default_layout,
        }
    }
}

pub struct App {
    pub definition: AppDefinition,
    pub examples: Vec<ExampleDefinition>,
    pub program: Option<ProgramDefinition>,
}

impl App {
    pub fn builder(id: impl Into<String>, label: impl Into<String>) -> AppBuilder {
        AppBuilder::new(id, label)
    }

    pub fn from_builder(builder: AppBuilder) -> Self {
        Self {
            definition: builder.build_definition(),
            examples: Vec::new(),
            program: None,
        }
    }

    pub fn example(mut self, id: impl Into<String>, label: impl Into<String>, document_json: impl Into<String>) -> Self {
        self.examples.push(ExampleDefinition {
            id: id.into(),
            label: label.into(),
            document_json: document_json.into(),
        });
        self
    }

    pub fn program(mut self, program_id: impl Into<String>, label: impl Into<String>, yields: impl Into<String>) -> Self {
        self.program = Some(ProgramDefinition {
            program_id: program_id.into(),
            app_id: self.definition.id.clone(),
            label: label.into(),
            document: self.definition.document.clone(),
            yields: yields.into(),
        });
        self
    }
}

pub trait PluginApp: Send {
    fn app_id(&self) -> &str;
    fn initial_document_json(&self) -> String;
    fn handle_command(&mut self, command: &str, args: Option<&Value>, document_json: &str, view_state: &ViewState) -> Vec<String>;
    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode;
    fn tools(&self, _document_json: &str, _view_state: &ViewState) -> Vec<ToolNode> {
        Vec::new()
    }
    fn window_engagements(
        &self,
        _document_json: &str,
        _view_state: &ViewState,
    ) -> std::collections::HashMap<String, semio_framework_core::layout::WindowEngagement> {
        std::collections::HashMap::new()
    }
    fn window_measures(
        &self,
        _document_json: &str,
        _view_state: &ViewState,
    ) -> std::collections::HashMap<String, Vec<semio_framework_core::WindowMeasure>> {
        std::collections::HashMap::new()
    }
}

pub struct AppInstance {
    pub id: u32,
    pub app: Box<dyn PluginApp>,
    pub document_json: String,
}

pub trait Plugin: Send {
    fn manifest(&self) -> PluginManifest;
    fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>>;
}

pub struct PluginBundle {
    pub manifest: PluginManifest,
    apps: HashMap<String, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>>,
}

impl PluginBundle {
    pub fn new(plugin_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            manifest: PluginManifest {
                plugin_id: plugin_id.into(),
                label: label.into(),
                version: version.into(),
                apps: Vec::new(),
                programs: Vec::new(),
                examples: Vec::new(),
            },
            apps: HashMap::new(),
        }
    }

    pub fn register_app(
        mut self,
        app: App,
        factory: impl Fn() -> Box<dyn PluginApp> + Send + 'static,
    ) -> Self {
        self.manifest.apps.push(app.definition);
        self.manifest.examples.extend(app.examples);
        if let Some(program) = app.program {
            self.manifest.programs.push(program);
        }
        self.apps
            .insert(self.manifest.apps.last().unwrap().id.clone(), Box::new(factory));
        self
    }

    pub fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {
        self.apps.get(app_id).map(|factory| factory())
    }
}

impl Plugin for PluginBundle {
    fn manifest(&self) -> PluginManifest {
        self.manifest.clone()
    }

    fn create_app(&self, app_id: &str) -> Option<Box<dyn PluginApp>> {
        PluginBundle::create_app(self, app_id)
    }
}
// #endregion app
}

pub mod generate_mode {
// #region generate_mode
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
// #endregion generate_mode
}

pub mod plugin_runtime {
// #region plugin_runtime
//! 📤 WASM component export glue for plugin bundles.

use crate::app::{AppInstance, Plugin, PluginBundle};
use semio_framework_core::{PluginManifest, UiNode, ViewState};
use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};

thread_local! {
    static PLUGIN: RefCell<Option<PluginBundle>> = const { RefCell::new(None) };
    static INSTANCES: RefCell<Vec<AppInstance>> = const { RefCell::new(Vec::new()) };
}

static NEXT_INSTANCE_ID: AtomicU32 = AtomicU32::new(1);

pub fn install_plugin_bundle(bundle: PluginBundle) {
    PLUGIN.with(|slot| {
        *slot.borrow_mut() = Some(bundle);
    });
}

pub fn plugin_manifest() -> PluginManifest {
    PLUGIN.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|plugin| plugin.manifest())
            .unwrap_or_else(|| PluginManifest {
                plugin_id: "empty".into(),
                label: "Empty".into(),
                version: "0.0.0".into(),
                apps: vec![],
                programs: vec![],
                examples: vec![],
            })
    })
}

pub fn plugin_create_app(app_id: &str) -> Result<u32, String> {
    PLUGIN.with(|slot| {
        let plugin = slot.borrow();
        let plugin = plugin.as_ref().ok_or_else(|| "plugin not initialized".to_string())?;
        let app = plugin
            .create_app(app_id)
            .ok_or_else(|| format!("unknown app: {app_id}"))?;
        let id = NEXT_INSTANCE_ID.fetch_add(1, Ordering::SeqCst);
        let document_json = app.initial_document_json();
        INSTANCES.with(|instances| {
            instances.borrow_mut().push(AppInstance {
                id,
                app,
                document_json,
            });
        });
        Ok(id)
    })
}

pub fn plugin_destroy_app(instance_id: u32) -> Result<(), String> {
    INSTANCES.with(|instances| {
        let mut list = instances.borrow_mut();
        let index = list
            .iter()
            .position(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        list.remove(index);
        Ok(())
    })
}

pub fn plugin_handle_command(
    instance_id: u32,
    command_json: &str,
    view_state_json: &str,
) -> Result<Vec<String>, String> {
    let command: serde_json::Value =
        serde_json::from_str(command_json).map_err(|error| error.to_string())?;
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    let command_name = command
        .get("command")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    let args = command.get("args").cloned();
    INSTANCES.with(|instances| {
        let mut list = instances.borrow_mut();
        let instance = list
            .iter_mut()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        let ops = instance.app.handle_command(
            command_name,
            args.as_ref(),
            &instance.document_json,
            &view_state,
        );
        for op in &ops {
            if let Ok(next) = apply_document_op(&instance.document_json, op) {
                instance.document_json = next;
            }
        }
        Ok(ops)
    })
}

pub fn plugin_render(instance_id: u32, body_key: &str, view_state_json: &str) -> Result<UiNode, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    INSTANCES.with(|instances| {
        let list = instances.borrow();
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .render(body_key, &instance.document_json, &view_state))
    })
}

pub fn plugin_tools(instance_id: u32, view_state_json: &str) -> Result<Vec<semio_framework_core::ToolNode>, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    INSTANCES.with(|instances| {
        let list = instances.borrow();
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .tools(&instance.document_json, &view_state))
    })
}

pub fn plugin_window_engagements(
    instance_id: u32,
    view_state_json: &str,
) -> Result<std::collections::HashMap<String, semio_framework_core::layout::WindowEngagement>, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    INSTANCES.with(|instances| {
        let list = instances.borrow();
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .window_engagements(&instance.document_json, &view_state))
    })
}

pub fn plugin_window_measures(
    instance_id: u32,
    view_state_json: &str,
) -> Result<std::collections::HashMap<String, Vec<semio_framework_core::WindowMeasure>>, String> {
    let view_state: ViewState =
        serde_json::from_str(view_state_json).map_err(|error| error.to_string())?;
    INSTANCES.with(|instances| {
        let list = instances.borrow();
        let instance = list
            .iter()
            .find(|instance| instance.id == instance_id)
            .ok_or_else(|| format!("unknown instance: {instance_id}"))?;
        Ok(instance
            .app
            .window_measures(&instance.document_json, &view_state))
    })
}

fn apply_document_op(document_json: &str, op_json: &str) -> Result<String, String> {
    let mut document: serde_json::Value =
        serde_json::from_str(document_json).map_err(|error| error.to_string())?;
    let op: serde_json::Value = serde_json::from_str(op_json).map_err(|error| error.to_string())?;
    match op.get("op").and_then(|value| value.as_str()) {
        Some("setDocument") => {
            if let Some(next) = op.get("document") {
                document = next.clone();
            }
        }
        Some("patch") => {
            if let Some(patch) = op.get("patch") {
                merge_json(&mut document, patch);
            }
        }
        _ => {}
    }
    serde_json::to_string(&document).map_err(|error| error.to_string())
}

fn merge_json(target: &mut serde_json::Value, patch: &serde_json::Value) {
    match (target, patch) {
        (serde_json::Value::Object(target_map), serde_json::Value::Object(patch_map)) => {
            for (key, value) in patch_map {
                if value.is_null() {
                    target_map.remove(key);
                } else {
                    let entry = target_map
                        .entry(key.clone())
                        .or_insert(serde_json::Value::Null);
                    merge_json(entry, value);
                }
            }
        }
        (target_slot, patch_value) => {
            *target_slot = patch_value.clone();
        }
    }
}

#[macro_export]
macro_rules! wasm_plugin_exports {
    () => {
        #[cfg(target_arch = "wasm32")]
        mod semio_wasm_exports {
            use super::_PLUGIN_INIT;
            use semio_framework_plugin::plugin_runtime::{
                plugin_create_app, plugin_destroy_app, plugin_handle_command, plugin_manifest, plugin_render,
                plugin_tools, plugin_window_engagements, plugin_window_measures,
            };
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(start)]
            pub fn semio_plugin_start() {
                let _ = &*_PLUGIN_INIT;
            }

            #[wasm_bindgen]
            pub fn semio_plugin_manifest() -> String {
                serde_json::to_string(&plugin_manifest()).unwrap_or_else(|_| "{}".into())
            }

            #[wasm_bindgen]
            pub fn semio_plugin_create_app(app_id: &str) -> Result<u32, JsValue> {
                plugin_create_app(app_id).map_err(|error| JsValue::from_str(&error))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_destroy_app(instance_id: u32) -> Result<(), JsValue> {
                plugin_destroy_app(instance_id).map_err(|error| JsValue::from_str(&error))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_handle_command(
                instance_id: u32,
                command_json: &str,
                view_state_json: &str,
            ) -> Result<String, JsValue> {
                let ops = plugin_handle_command(instance_id, command_json, view_state_json)
                    .map_err(|error| JsValue::from_str(&error))?;
                serde_json::to_string(&ops).map_err(|error| JsValue::from_str(&error.to_string()))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_render(
                instance_id: u32,
                body_key: &str,
                view_state_json: &str,
            ) -> Result<String, JsValue> {
                let node = plugin_render(instance_id, body_key, view_state_json)
                    .map_err(|error| JsValue::from_str(&error))?;
                serde_json::to_string(&node).map_err(|error| JsValue::from_str(&error.to_string()))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_tools(instance_id: u32, view_state_json: &str) -> Result<String, JsValue> {
                let tools = plugin_tools(instance_id, view_state_json)
                    .map_err(|error| JsValue::from_str(&error))?;
                serde_json::to_string(&tools).map_err(|error| JsValue::from_str(&error.to_string()))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_window_engagements(
                instance_id: u32,
                view_state_json: &str,
            ) -> Result<String, JsValue> {
                let engagements = plugin_window_engagements(instance_id, view_state_json)
                    .map_err(|error| JsValue::from_str(&error))?;
                serde_json::to_string(&engagements).map_err(|error| JsValue::from_str(&error.to_string()))
            }

            #[wasm_bindgen]
            pub fn semio_plugin_window_measures(
                instance_id: u32,
                view_state_json: &str,
            ) -> Result<String, JsValue> {
                let measures = plugin_window_measures(instance_id, view_state_json)
                    .map_err(|error| JsValue::from_str(&error))?;
                serde_json::to_string(&measures).map_err(|error| JsValue::from_str(&error.to_string()))
            }
        }
    };
}

#[macro_export]
macro_rules! native_plugin_exports {
    () => {
        #[cfg(not(target_arch = "wasm32"))]
        mod semio_native_exports {
            use super::_PLUGIN_INIT;
            use semio_framework_plugin::plugin_runtime::{
                plugin_create_app, plugin_destroy_app, plugin_handle_command, plugin_manifest, plugin_render,
                plugin_tools, plugin_window_engagements, plugin_window_measures,
            };
            use std::ffi::{c_char, CStr, CString};
            use std::sync::LazyLock;

            static START: LazyLock<()> = LazyLock::new(|| {
                let _ = &*_PLUGIN_INIT;
            });

            fn to_c_string(value: String) -> *mut c_char {
                CString::new(value)
                    .map(|string| string.into_raw())
                    .unwrap_or(std::ptr::null_mut())
            }

            unsafe fn read_c_str(ptr: *const c_char) -> Result<String, String> {
                if ptr.is_null() {
                    return Err("null c string".into());
                }
                CStr::from_ptr(ptr)
                    .to_str()
                    .map(|value| value.to_string())
                    .map_err(|error| error.to_string())
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_manifest() -> *mut c_char {
                let _ = &*START;
                to_c_string(serde_json::to_string(&plugin_manifest()).unwrap_or_else(|_| "{}".into()))
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_create_app(app_id: *const c_char) -> u32 {
                let _ = &*START;
                let Ok(app_id) = (unsafe { read_c_str(app_id) }) else {
                    return u32::MAX;
                };
                plugin_create_app(&app_id).unwrap_or(u32::MAX)
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_destroy_app(instance_id: u32) {
                let _ = plugin_destroy_app(instance_id);
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_handle_command(
                instance_id: u32,
                command_json: *const c_char,
                view_state_json: *const c_char,
            ) -> *mut c_char {
                let Ok(command_json) = (unsafe { read_c_str(command_json) }) else {
                    return std::ptr::null_mut();
                };
                let Ok(view_state_json) = (unsafe { read_c_str(view_state_json) }) else {
                    return std::ptr::null_mut();
                };
                let ops = plugin_handle_command(instance_id, &command_json, &view_state_json)
                    .unwrap_or_default();
                to_c_string(serde_json::to_string(&ops).unwrap_or_else(|_| "[]".into()))
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_render(
                instance_id: u32,
                body_key: *const c_char,
                view_state_json: *const c_char,
            ) -> *mut c_char {
                let Ok(body_key) = (unsafe { read_c_str(body_key) }) else {
                    return std::ptr::null_mut();
                };
                let Ok(view_state_json) = (unsafe { read_c_str(view_state_json) }) else {
                    return std::ptr::null_mut();
                };
                let node = plugin_render(instance_id, &body_key, &view_state_json).unwrap_or_default();
                to_c_string(serde_json::to_string(&node).unwrap_or_else(|_| "{}".into()))
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_tools(instance_id: u32, view_state_json: *const c_char) -> *mut c_char {
                let Ok(view_state_json) = (unsafe { read_c_str(view_state_json) }) else {
                    return std::ptr::null_mut();
                };
                let tools = plugin_tools(instance_id, &view_state_json).unwrap_or_default();
                to_c_string(serde_json::to_string(&tools).unwrap_or_else(|_| "[]".into()))
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_window_engagements(
                instance_id: u32,
                view_state_json: *const c_char,
            ) -> *mut c_char {
                let Ok(view_state_json) = (unsafe { read_c_str(view_state_json) }) else {
                    return std::ptr::null_mut();
                };
                let engagements = plugin_window_engagements(instance_id, &view_state_json).unwrap_or_default();
                to_c_string(serde_json::to_string(&engagements).unwrap_or_else(|_| "{}".into()))
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_window_measures(
                instance_id: u32,
                view_state_json: *const c_char,
            ) -> *mut c_char {
                let Ok(view_state_json) = (unsafe { read_c_str(view_state_json) }) else {
                    return std::ptr::null_mut();
                };
                let measures = plugin_window_measures(instance_id, &view_state_json).unwrap_or_default();
                to_c_string(serde_json::to_string(&measures).unwrap_or_else(|_| "{}".into()))
            }

            #[no_mangle]
            pub extern "C" fn semio_plugin_free_string(ptr: *mut c_char) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    drop(CString::from_raw(ptr));
                }
            }
        }
    };
}

#[macro_export]
macro_rules! plugin_exports {
    () => {
        semio_framework_plugin::wasm_plugin_exports!();
        semio_framework_plugin::native_plugin_exports!();
    };
}
// #endregion plugin_runtime
}

pub mod native_host {
// #region native_host
//! 🔌 Native dylib plugin loader with hot-swap support.

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use libloading::{Library, Symbol};
    use semio_framework_core::{PluginManifest, ToolNode, UiNode, ViewState, WindowEngagement, WindowMeasure};
    use std::ffi::{c_char, CStr, CString};
    use std::path::{Path, PathBuf};

    type ManifestFn = unsafe extern "C" fn() -> *mut c_char;
    type CreateAppFn = unsafe extern "C" fn(*const c_char) -> u32;
    type DestroyAppFn = unsafe extern "C" fn(u32);
    type CommandFn = unsafe extern "C" fn(u32, *const c_char, *const c_char) -> *mut c_char;
    type RenderFn = unsafe extern "C" fn(u32, *const c_char, *const c_char) -> *mut c_char;
    type ToolsFn = unsafe extern "C" fn(u32, *const c_char) -> *mut c_char;
    type EngagementsFn = unsafe extern "C" fn(u32, *const c_char) -> *mut c_char;
    type MeasuresFn = unsafe extern "C" fn(u32, *const c_char) -> *mut c_char;
    type FreeStringFn = unsafe extern "C" fn(*mut c_char);

    pub struct NativePluginLibrary {
        library: Library,
        pub path: PathBuf,
    }

    impl NativePluginLibrary {
        pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
            let path = path.as_ref().to_path_buf();
            let library = unsafe { Library::new(&path).map_err(|error| error.to_string())? };
            Ok(Self { library, path })
        }

        pub fn manifest(&self) -> Result<PluginManifest, String> {
            let json = self.call_string(b"semio_plugin_manifest", |symbol| unsafe { symbol() })?;
            serde_json::from_str(&json).map_err(|error| error.to_string())
        }

        pub fn create_app(&self, app_id: &str) -> Result<u32, String> {
            let c_app = CString::new(app_id).map_err(|error| error.to_string())?;
            let create: Symbol<CreateAppFn> = unsafe {
                self.library
                    .get(b"semio_plugin_create_app")
                    .map_err(|error| error.to_string())?
            };
            let id = unsafe { create(c_app.as_ptr()) };
            if id == u32::MAX {
                return Err("create_app failed".into());
            }
            Ok(id)
        }

        pub fn destroy_app(&self, instance_id: u32) {
            if let Ok(destroy) = unsafe { self.library.get::<DestroyAppFn>(b"semio_plugin_destroy_app") } {
                unsafe { destroy(instance_id) };
            }
        }

        pub fn handle_command(
            &self,
            instance_id: u32,
            command_json: &str,
            view_state: &ViewState,
        ) -> Result<Vec<String>, String> {
            let command = CString::new(command_json).map_err(|error| error.to_string())?;
            let view = CString::new(serde_json::to_string(view_state).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
            let handle: Symbol<CommandFn> = unsafe {
                self.library
                    .get(b"semio_plugin_handle_command")
                    .map_err(|error| error.to_string())?
            };
            let json = self.call_string_ptr(unsafe {
                handle(instance_id, command.as_ptr(), view.as_ptr())
            })?;
            serde_json::from_str(&json).map_err(|error| error.to_string())
        }

        pub fn render(&self, instance_id: u32, body_key: &str, view_state: &ViewState) -> Result<UiNode, String> {
            let body = CString::new(body_key).map_err(|error| error.to_string())?;
            let view = CString::new(serde_json::to_string(view_state).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
            let render: Symbol<RenderFn> = unsafe {
                self.library
                    .get(b"semio_plugin_render")
                    .map_err(|error| error.to_string())?
            };
            let json = self.call_string_ptr(unsafe {
                render(instance_id, body.as_ptr(), view.as_ptr())
            })?;
            serde_json::from_str(&json).map_err(|error| error.to_string())
        }

        pub fn tools(&self, instance_id: u32, view_state: &ViewState) -> Result<Vec<ToolNode>, String> {
            let view = CString::new(serde_json::to_string(view_state).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
            let tools: Symbol<ToolsFn> = unsafe {
                self.library
                    .get(b"semio_plugin_tools")
                    .map_err(|error| error.to_string())?
            };
            let json = self.call_string_ptr(unsafe { tools(instance_id, view.as_ptr()) })?;
            serde_json::from_str(&json).map_err(|error| error.to_string())
        }

        pub fn window_engagements(
            &self,
            instance_id: u32,
            view_state: &ViewState,
        ) -> Result<std::collections::HashMap<String, WindowEngagement>, String> {
            let view = CString::new(serde_json::to_string(view_state).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
            let engagements: Symbol<EngagementsFn> = unsafe {
                self.library
                    .get(b"semio_plugin_window_engagements")
                    .map_err(|error| error.to_string())?
            };
            let json = self.call_string_ptr(unsafe { engagements(instance_id, view.as_ptr()) })?;
            serde_json::from_str(&json).map_err(|error| error.to_string())
        }

        pub fn window_measures(
            &self,
            instance_id: u32,
            view_state: &ViewState,
        ) -> Result<std::collections::HashMap<String, Vec<WindowMeasure>>, String> {
            let view = CString::new(serde_json::to_string(view_state).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
            let measures: Symbol<MeasuresFn> = unsafe {
                self.library
                    .get(b"semio_plugin_window_measures")
                    .map_err(|error| error.to_string())?
            };
            let json = self.call_string_ptr(unsafe { measures(instance_id, view.as_ptr()) })?;
            serde_json::from_str(&json).map_err(|error| error.to_string())
        }

        fn call_string(
            &self,
            symbol_name: &[u8],
            invoke: impl FnOnce(Symbol<ManifestFn>) -> *mut c_char,
        ) -> Result<String, String> {
            let symbol: Symbol<ManifestFn> = unsafe {
                self.library.get(symbol_name).map_err(|error| error.to_string())?
            };
            self.call_string_ptr(unsafe { invoke(symbol) })
        }

        fn call_string_ptr(&self, ptr: *mut c_char) -> Result<String, String> {
            if ptr.is_null() {
                return Err("native plugin returned null".into());
            }
            let value = unsafe { CStr::from_ptr(ptr) }
                .to_str()
                .map_err(|error| error.to_string())?
                .to_string();
            if let Ok(free) = unsafe { self.library.get::<FreeStringFn>(b"semio_plugin_free_string") } {
                unsafe { free(ptr) };
            }
            Ok(value)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::NativePluginLibrary;
// #endregion native_host
}

pub mod scaffold {
// #region scaffold
//! 🧰 Helpers for scaffolding standard technology plugins.

use crate::{
    build_canvas_2d_scene, build_node_graph_scene, build_raster_scene, build_table_scene,
    build_text_editor_scene, build_world_3d_scene, default_world3d_selection, ui_stack_vertical,
    ui_text, world3d_default_meshes_json, App, Canvas2dScene, NodeGraphScene, PluginApp,
    RasterScene, TableScene, TextEditorScene, UiNode, ViewState, World3dScene,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneKind {
    Canvas2d,
    World3d,
    NodeGraph,
    TextEditor,
    Table,
    Raster,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StandardApp {
    pub app_id: &'static str,
    pub label: &'static str,
    pub document: &'static [&'static str],
    pub program_id: Option<&'static str>,
    pub yields: Option<&'static str>,
    pub surface_id: &'static str,
    pub body_key: &'static str,
    pub scene_kind: SceneKind,
    pub initial_document_json: &'static str,
}

pub struct StandardPluginApp {
    pub spec: StandardApp,
}

fn document_body_key(body_key: &str) -> String {
    body_key.replace(".composite", ".document")
}

fn properties_body_key(body_key: &str) -> String {
    body_key.replace(".composite", ".properties")
}

fn json_field(document: &Value, keys: &[&str]) -> Option<Value> {
    keys.iter().find_map(|key| document.get(*key)).cloned()
}

fn canvas_layers_json(document: &Value, fallback: &str) -> String {
    json_field(
        document,
        &["layers", "tiles", "blocks", "features", "cells", "nodes"],
    )
    .map(|value| value.to_string())
    .unwrap_or_else(|| fallback.to_string())
}

fn world_instances_json(document: &Value, fallback: &str) -> String {
    json_field(
        document,
        &["instances", "entities", "meshes", "tiles", "cells", "parts"],
    )
    .map(|value| value.to_string())
    .unwrap_or_else(|| fallback.to_string())
}

fn node_graph_payload(document: &Value, fallback: &str) -> (String, String) {
    if let Some(nodes) = document.get("nodes") {
        let edges = document
            .get("edges")
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        return (nodes.to_string(), edges.to_string());
    }
    if let Some(flow) = document.get("flow") {
        let nodes = flow
            .get("components")
            .or_else(|| flow.get("nodes"))
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        let edges = flow
            .get("edges")
            .cloned()
            .unwrap_or(Value::Array(vec![]));
        return (nodes.to_string(), edges.to_string());
    }
    if let Some(steps) = document.get("steps") {
        return (steps.to_string(), "[]".into());
    }
    (fallback.into(), "[]".into())
}

fn text_editor_payload(document: &Value, fallback: &str) -> (String, Option<String>) {
    if let Some(text) = document
        .get("text")
        .or_else(|| document.get("source"))
        .and_then(|value| value.as_str())
    {
        let language = document
            .get("language")
            .and_then(|value| value.as_str())
            .map(str::to_string);
        return (text.into(), language);
    }
    if document.is_string() {
        return (
            document.as_str().unwrap_or(fallback).into(),
            Some("plain".into()),
        );
    }
    (fallback.into(), Some("plain".into()))
}

fn table_payload(document: &Value, fallback: &str) -> (String, String) {
    let rows = json_field(document, &["rows", "edits", "records"])
        .map(|value| value.to_string())
        .unwrap_or_else(|| fallback.to_string());
    let columns = document
        .get("columns")
        .map(|value| value.to_string())
        .unwrap_or_else(|| r#"[{"id":"id","label":"Id"}]"#.into());
    (columns, rows)
}

fn raster_payload(document: &Value, fallback: &str) -> RasterScene {
    if let Ok(scene) = serde_json::from_value::<RasterScene>(document.clone()) {
        return scene;
    }
    let parsed: Value = serde_json::from_str(fallback).unwrap_or(Value::Null);
    RasterScene {
        width: document
            .get("width")
            .or_else(|| parsed.get("width"))
            .and_then(|value| value.as_u64())
            .unwrap_or(256) as u32,
        height: document
            .get("height")
            .or_else(|| parsed.get("height"))
            .and_then(|value| value.as_u64())
            .unwrap_or(256) as u32,
        pixels_base64: document
            .get("pixelsBase64")
            .or_else(|| document.get("pixels_base64"))
            .and_then(|value| value.as_str())
            .unwrap_or("")
            .into(),
    }
}

pub fn scene_kind_component_tag(kind: SceneKind) -> &'static str {
    match kind {
        SceneKind::Canvas2d => "canvas-2d",
        SceneKind::World3d => "world-3d",
        SceneKind::NodeGraph => "node-graph",
        SceneKind::TextEditor => "text-editor",
        SceneKind::Table => "table",
        SceneKind::Raster => "raster",
    }
}

pub fn assert_standard_app_renders(spec: StandardApp) {
    let app = StandardPluginApp { spec };
    let node = app.render(spec.body_key, spec.initial_document_json, &ViewState::default());
    let json = serde_json::to_string(&node).expect("ui json");
    let tag = scene_kind_component_tag(spec.scene_kind);
    assert!(json.contains(tag), "expected {tag} in {json}");
}

impl PluginApp for StandardPluginApp {
    fn app_id(&self) -> &str {
        self.spec.app_id
    }

    fn initial_document_json(&self) -> String {
        self.spec.initial_document_json.to_string()
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        if command == "setDocument" {
            if let Some(document) = args.and_then(|value| value.get("document")) {
                return vec![serde_json::json!({ "op": "setDocument", "document": document }).to_string()];
            }
        }
        if command == "patch" {
            if let Some(patch) = args.and_then(|value| value.get("patch")) {
                return vec![serde_json::json!({ "op": "patch", "patch": patch }).to_string()];
            }
        }
        let _ = document_json;
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let document_key = document_body_key(self.spec.body_key);
        let properties_key = properties_body_key(self.spec.body_key);
        if body_key == document_key {
            return render_document_panel(self.spec.label, document_json);
        }
        if body_key == properties_key {
            return render_properties_panel(self.spec.label, document_json);
        }
        if body_key != self.spec.body_key {
            return ui_text(format!("Unknown body: {body_key}"));
        }
        let document: Value =
            serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
        match self.spec.scene_kind {
            SceneKind::Canvas2d => build_canvas_2d_scene(
                self.spec.surface_id,
                self.spec.app_id,
                Canvas2dScene {
                    camera_x: document
                        .get("camera")
                        .and_then(|camera| camera.get("x"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(0.0),
                    camera_y: document
                        .get("camera")
                        .and_then(|camera| camera.get("y"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(0.0),
                    zoom: document
                        .get("camera")
                        .and_then(|camera| camera.get("zoom"))
                        .and_then(|value| value.as_f64())
                        .unwrap_or(1.0),
                    layers_json: canvas_layers_json(&document, document_json),
                },
            ),
            SceneKind::World3d => build_world_3d_scene(
                self.spec.surface_id,
                self.spec.app_id,
                World3dScene {
                    camera_json: document
                        .get("camera")
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| r#"{"x":0,"y":0,"z":5}"#.into()),
                    meshes_json: document
                        .get("meshes")
                        .map(|value| value.to_string())
                        .unwrap_or_else(world3d_default_meshes_json),
                    instances_json: world_instances_json(&document, document_json),
                    selection_json: document
                        .get("selection")
                        .map(|value| value.to_string())
                        .unwrap_or_else(default_world3d_selection),
                    vortices_json: None,
                    attractions_json: None,
                    target_volumes_json: None,
                    references_json: None,
                    brush_preview_json: None,
                    interaction_json: None,
                },
            ),
            SceneKind::NodeGraph => {
                let (nodes_json, edges_json) = node_graph_payload(&document, document_json);
                build_node_graph_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    NodeGraphScene::base(
                        nodes_json,
                        edges_json,
                        document
                            .get("viewport")
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| r#"{"x":0,"y":0,"zoom":1}"#.into()),
                    ),
                )
            }
            SceneKind::TextEditor => {
                let (buffer, language) = text_editor_payload(&document, document_json);
                build_text_editor_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    TextEditorScene::base(buffer, language, None),
                )
            }
            SceneKind::Table => {
                let (columns_json, rows_json) = table_payload(&document, document_json);
                build_table_scene(
                    self.spec.surface_id,
                    self.spec.app_id,
                    TableScene {
                        columns_json,
                        rows_json,
                    },
                )
            }
            SceneKind::Raster => build_raster_scene(
                self.spec.surface_id,
                self.spec.app_id,
                raster_payload(&document, document_json),
            ),
        }
    }
}

fn render_document_panel(label: &str, document_json: &str) -> UiNode {
    let document: Value =
        serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
    let schema = document
        .get("schema")
        .and_then(|value| value.as_str())
        .unwrap_or(label);
    let count = document
        .get("layers")
        .or_else(|| document.get("nodes"))
        .or_else(|| document.get("rows"))
        .or_else(|| document.get("entities"))
        .and_then(|value| value.as_array())
        .map(|rows| rows.len())
        .unwrap_or(0);
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {schema}")),
        ui_text(format!("Items: {count}")),
    ])
}

fn render_properties_panel(label: &str, document_json: &str) -> UiNode {
    let document: Value =
        serde_json::from_str(document_json).unwrap_or(Value::String(document_json.into()));
    let id = document
        .get("id")
        .and_then(|value| value.as_str())
        .unwrap_or(label);
    ui_stack_vertical(vec![
        ui_text(format!("App: {label}")),
        ui_text(format!("Id: {id}")),
    ])
}

pub fn standard_app(spec: StandardApp) -> App {
    let document_key = document_body_key(spec.body_key);
    let properties_key = properties_body_key(spec.body_key);
    let app = App::from_builder(
        App::builder(spec.app_id, spec.label)
            .document(spec.document.iter().copied())
            .icon_id(spec.app_id)
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind("main", "Main", spec.body_key)
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                "workbench",
                &document_key,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                &properties_key,
            ),
    );
    if let (Some(program_id), Some(yields)) = (spec.program_id, spec.yields) {
        app.program(program_id, spec.label, yields)
    } else {
        app
    }
}

pub fn standard_factory(spec: StandardApp) -> Box<dyn PluginApp> {
    Box::new(StandardPluginApp { spec })
}

pub fn register_standard_app(bundle: crate::PluginBundle, spec: StandardApp) -> crate::PluginBundle {
    let app = standard_app(spec);
    bundle.register_app(app, move || standard_factory(spec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-canvas",
            label: "Canvas",
            document: &["semio", "test", "canvas"],
            program_id: None,
            yields: None,
            surface_id: "test.canvas",
            body_key: "test.canvas.composite",
            scene_kind: SceneKind::Canvas2d,
            initial_document_json: r#"{"schema":"test","id":"test","layers":[]}"#,
        });
    }

    #[test]
    fn node_graph_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-graph",
            label: "Graph",
            document: &["semio", "test", "graph"],
            program_id: None,
            yields: None,
            surface_id: "test.graph",
            body_key: "test.graph.composite",
            scene_kind: SceneKind::NodeGraph,
            initial_document_json: r#"{"nodes":[],"edges":[]}"#,
        });
    }

    #[test]
    fn world_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-world",
            label: "World",
            document: &["semio", "test", "world"],
            program_id: None,
            yields: None,
            surface_id: "test.world",
            body_key: "test.world.composite",
            scene_kind: SceneKind::World3d,
            initial_document_json: r#"{"schema":"test","id":"test","entities":[]}"#,
        });
    }

    #[test]
    fn text_editor_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-text",
            label: "Text",
            document: &["semio", "test", "text"],
            program_id: None,
            yields: None,
            surface_id: "test.text",
            body_key: "test.text.composite",
            scene_kind: SceneKind::TextEditor,
            initial_document_json: r#"{"schema":"test","id":"test","source":""}"#,
        });
    }

    #[test]
    fn table_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-table",
            label: "Table",
            document: &["semio", "test", "table"],
            program_id: None,
            yields: None,
            surface_id: "test.table",
            body_key: "test.table.composite",
            scene_kind: SceneKind::Table,
            initial_document_json: r#"{"schema":"test","id":"test","rows":[]}"#,
        });
    }

    #[test]
    fn raster_scene_renders() {
        assert_standard_app_renders(StandardApp {
            app_id: "test-raster",
            label: "Raster",
            document: &["semio", "test", "raster"],
            program_id: None,
            yields: None,
            surface_id: "test.raster",
            body_key: "test.raster.composite",
            scene_kind: SceneKind::Raster,
            initial_document_json: r#"{"schema":"raster.document","id":"raster","width":64,"height":64,"pixelsBase64":""}"#,
        });
    }
}
// #endregion scaffold
}

pub mod world3d_host {
// #region world3d_host
//! 🌐 Shared world-3d scene payload builders for plugin apps.

use semio_framework_core::{
    mesh_from_kind, mesh_to_glb, mesh_to_obj, MeshData, World3dScene, world3d_camera_json,
    world3d_default_selection_json,
};
use serde_json::{json, Value};

pub fn mesh_kind_from_json(mesh_json: &str) -> String {
    serde_json::from_str::<Value>(mesh_json)
        .ok()
        .and_then(|value| value.get("kind").and_then(|v| v.as_str()).map(str::to_string))
        .unwrap_or_else(|| "box".into())
}

pub fn world3d_meshes_json_from_kinds(kinds: &[String]) -> String {
    let meshes: Vec<Value> = kinds
        .iter()
        .map(|kind| {
            let data = mesh_from_kind(kind);
            json!({ "id": kind, "data": data })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn world3d_mesh_id_from_url(url: &str) -> String {
    let slug = url
        .trim_start_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(url)
        .trim_end_matches(".glb")
        .trim_end_matches(".gltf");
    format!("mesh:{slug}")
}

pub fn world3d_meshes_json_from_urls(urls: &[String]) -> String {
    let meshes: Vec<Value> = urls
        .iter()
        .map(|url| {
            json!({
                "id": world3d_mesh_id_from_url(url),
                "url": url,
            })
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn world3d_meshes_json_from_kinds_and_urls(kinds: &[String], urls: &[String]) -> String {
    let mut meshes: Vec<Value> = kinds
        .iter()
        .map(|kind| {
            let data = mesh_from_kind(kind);
            json!({ "id": kind, "data": data })
        })
        .collect();
    for url in urls {
        let id = world3d_mesh_id_from_url(url);
        if meshes.iter().any(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(id.as_str())) {
            continue;
        }
        meshes.push(json!({ "id": id, "url": url }));
    }
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn world3d_selection_json(method: &str, ids: &[String], hovered_id: Option<&str>) -> String {
    world3d_selection_json_with_granularity(method, ids, hovered_id, None)
}

pub fn world3d_selection_json_with_granularity(
    method: &str,
    ids: &[String],
    hovered_id: Option<&str>,
    granularity: Option<&str>,
) -> String {
    let mut value = json!({
        "method": method,
        "mode": "replace",
        "ids": ids,
        "hoveredId": hovered_id,
    });
    if let Some(entry) = granularity {
        if let Some(object) = value.as_object_mut() {
            object.insert("granularity".into(), json!(entry));
        }
    }
    value.to_string()
}

pub fn world3d_scene(
    camera_json: String,
    meshes_json: String,
    instances_json: String,
    selection_json: String,
) -> World3dScene {
    world3d_scene_extended(
        camera_json,
        meshes_json,
        instances_json,
        selection_json,
        None,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn world3d_scene_extended(
    camera_json: String,
    meshes_json: String,
    instances_json: String,
    selection_json: String,
    vortices_json: Option<String>,
    attractions_json: Option<String>,
    target_volumes_json: Option<String>,
    references_json: Option<String>,
    brush_preview_json: Option<String>,
    interaction_json: Option<String>,
) -> World3dScene {
    World3dScene {
        camera_json,
        meshes_json,
        instances_json,
        selection_json,
        vortices_json,
        attractions_json,
        target_volumes_json,
        references_json,
        brush_preview_json,
        interaction_json,
    }
}

pub fn world3d_default_camera() -> String {
    world3d_camera_json([4.0, -4.0, 3.0], [0.0, 0.0, 0.0], 45.0)
}

pub fn export_mesh_obj(mesh: &MeshData, name: &str) -> (String, String) {
    (mesh_to_obj(mesh, name), "text/plain".into())
}

pub fn export_mesh_glb_bytes(mesh: &MeshData) -> (Vec<u8>, String) {
    (mesh_to_glb(mesh), "model/gltf-binary".into())
}

pub fn merge_world_selection_ids(existing: &[String], incoming: &[String], merge: &str) -> Vec<String> {
    match merge {
        "add" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(id.clone());
                }
            }
            merged
        }
        "toggle" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id.clone());
                }
            }
            merged
        }
        _ => incoming.to_vec(),
    }
}

pub fn default_world3d_selection() -> String {
    world3d_default_selection_json()
}
// #endregion world3d_host
}


pub use app::{
    App, AppBuilder, AppInstance, KeybindingSpec, ModeSpec, PanelTabSpec, Plugin, PluginApp, PluginBundle,
    WindowKindSpec,
};
pub use generate_mode::{
    add_generation, handle_generation_command, initial_generation_values, remove_generation,
    rename_generation, render_generation_form_body, render_generation_preview_text,
    render_generations_tree, select_generation, selected_generation, selected_generation_mut,
    update_generation_values, FormGeneration, GenerationPlayState,
};
pub use plugin_runtime::install_plugin_bundle;
pub use scaffold::{
    assert_standard_app_renders, register_standard_app, scene_kind_component_tag, standard_app,
    standard_factory, SceneKind, StandardApp, StandardPluginApp,
};
pub use world3d_host::{
    default_world3d_selection, export_mesh_glb_bytes, export_mesh_obj, merge_world_selection_ids,
    mesh_kind_from_json, world3d_default_camera, world3d_mesh_id_from_url,
    world3d_meshes_json_from_kinds, world3d_meshes_json_from_kinds_and_urls,
    world3d_meshes_json_from_urls, world3d_scene, world3d_scene_extended,
    world3d_selection_json,
};
pub use semio_framework_core::*;

#[macro_export]
macro_rules! register_plugin {
    ($bundle:expr) => {
        $crate::plugin_runtime::install_plugin_bundle($bundle);
    };
}
