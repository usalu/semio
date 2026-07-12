//! 🌊 Flow plugin — declarative flow play app bundled as a hot-swappable WASM component.

use flow_core::{
    dag::{dag_lod_scale_json, DagDrawLod, DagFixture},
    flow_backed_node_graph_extras, flow_neuron_kind_infos_json, flow_operator_catalogue_json, FLOW_LOD_MODE_AUTOMATIC,
    forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec},
    CameraJson, FlowFixture, FlowHost, Widget,
};
use semio_framework_plugin::{SurfaceKind, PanelGroup, 
    build_node_graph_scene, build_text_editor_scene, create_default_layout, create_named_layout,
    handle_generation_action, render_generation_form_body, render_generation_preview_text, render_generations_tree,
    selected_generation, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, App, ActionDescriptor, GenerationPlayState,
    NodeGraphScene, PluginApp, PluginBundle, TextEditorScene, UiControlNode, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiSelectItem, UiSelectNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID,     FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::LazyLock;

//#region 🔖Constants
const FLOW_PLAY_APP_ID: &str = "flow-play";
const FLOW_PLAY_SURFACE_MAIN: &str = "flow.play.main";
const FLOW_PLAY_SURFACE_COMPILED: &str = "flow.play.compiled-dag";
const FLOW_PLAY_BODY_MAIN: &str = "flow.play.main";
const FLOW_PLAY_BODY_COMPILED: &str = "flow.play.compiled-dag";
const FLOW_PLAY_BODY_DOCUMENT: &str = "flow.play.document";
const FLOW_PLAY_BODY_CATALOGUE: &str = "flow.play.catalogue";
const FLOW_PLAY_BODY_INSPECTOR: &str = "flow.play.inspection";
const FLOW_PLAY_WINDOW_MAIN: &str = "flow-main";
const FLOW_PLAY_WINDOW_COMPILED: &str = "flow-compiled-dag";
const FLOW_PLAY_WINDOW_GENERATIONS: &str = "flow-generations";
const FLOW_PLAY_WINDOW_GENERATE_FORM: &str = "flow-generate-form";
const FLOW_PLAY_WINDOW_GENERATE_PREVIEW: &str = "flow-generate-preview";
const FLOW_PLAY_BODY_GENERATIONS: &str = "flow.play.generations";
const FLOW_PLAY_BODY_GENERATE_FORM: &str = "flow.play.generate-form";
const FLOW_PLAY_BODY_GENERATE_PREVIEW: &str = "flow.play.generate-preview";
const FLOW_PLAY_SURFACE_GENERATE_PREVIEW: &str = "flow.play.generate-preview";
const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";

/// 🧩 Built-in flow extensions: (id, name, actionId, actionTitle, effect).
const FLOW_EXTENSIONS: &[(&str, &str, &str, &str, &str)] = &[
    ("auto-layout", "Auto Layout", "flow.extension.reorganize", "Reorganize Canvas", "reorganize"),
    ("auto-evaluate", "Auto Evaluate", "flow.extension.evaluate", "Evaluate Fixture", "evaluate"),
];
//#endregion 🔖Constants

//#region 🔖Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FlowPlayRuntime {
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default)]
    last_eval_json: String,
    #[serde(default = "default_flow_lod_mode")]
    lod_mode: String,
    #[serde(default)]
    proximity_distance: f64,
    #[serde(default = "default_catalogue_sections_json")]
    catalogue_sections_json: String,
    #[serde(default)]
    extension_enabled: HashMap<String, bool>,
}

fn default_flow_lod_mode() -> String {
    FLOW_LOD_MODE_AUTOMATIC.into()
}

fn default_catalogue_sections_json() -> String {
    "[]".into()
}

impl Default for FlowPlayRuntime {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            last_eval_json: String::new(),
            lod_mode: default_flow_lod_mode(),
            proximity_distance: 0.0,
            catalogue_sections_json: default_catalogue_sections_json(),
            extension_enabled: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FlowPlayEnvelope {
    fixture: FlowFixture,
    #[serde(default)]
    runtime: FlowPlayRuntime,
    #[serde(default)]
    generation: GenerationPlayState,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphPortRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphNodeRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    inputs: Vec<MediaGraphPortRecord>,
    outputs: Vec<MediaGraphPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn default_envelope() -> FlowPlayEnvelope {
    FlowPlayEnvelope {
        fixture: FlowFixture::default(),
        runtime: FlowPlayRuntime::default(),
        generation: GenerationPlayState::default(),
    }
}

fn parse_envelope(document_json: &str) -> FlowPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &FlowPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn flow_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: FLOW_PLAY_APP_ID.into(),
        action: action.into(),
        args,
    }
}

fn seed_host_catalogue(host: &mut FlowHost, extra_sections_json: &str) {
    let mut sections: Vec<Value> = serde_json::from_str(&flow_operator_catalogue_json()).unwrap_or_default();
    if let Ok(extra) = serde_json::from_str::<Vec<Value>>(extra_sections_json) {
        sections.extend(extra);
    }
    host.set_host_catalogue_json(&serde_json::to_string(&sections).unwrap_or_else(|_| "[]".into()));
}

fn apply_lod_and_proximity(host: &mut FlowHost, runtime: &FlowPlayRuntime) {
    if runtime.lod_mode != FLOW_LOD_MODE_AUTOMATIC && DagDrawLod::from_id(&runtime.lod_mode).is_some() {
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label(&runtime.lod_mode);
    } else {
        host.dag.set_automatic_lod(true);
    }
    host.dag.set_proximity_distance(runtime.proximity_distance);
}

fn host_from_envelope(envelope: &FlowPlayEnvelope) -> FlowHost {
    let mut host = FlowHost::from_fixture(envelope.fixture.clone());
    seed_host_catalogue(&mut host, &envelope.runtime.catalogue_sections_json);
    apply_lod_and_proximity(&mut host, &envelope.runtime);
    host
}

fn sync_host_selection(host: &mut FlowHost, selected: &[String]) {
    if selected.is_empty() {
        let _ = host.dag.cancel_area_select();
    } else {
        host.dag.set_selection(selected);
    }
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

fn fixture_to_media_graph(fixture: &DagFixture) -> (String, String) {
    let nodes: Vec<MediaGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| MediaGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node
                .inputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| MediaGraphPortRecord {
                    id: format!("{}:{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
            outputs: node
                .outputs()
                .iter()
                .filter(|port| port.visible)
                .map(|port| MediaGraphPortRecord {
                    id: format!("{}:{}", node.id, port.id),
                    label: Some(port.label.clone()),
                })
                .collect(),
        })
        .collect();
    let edges: Vec<MediaGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            MediaGraphEdgeRecord {
                id: edge.id.clone(),
                source_node_id,
                source_port_id,
                target_node_id,
                target_port_id,
            }
        })
        .collect();
    (
        serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
    )
}

fn widget_kind_label(widget: &Widget) -> &'static str {
    match widget {
        Widget::Neuron { .. } => "neuron",
        Widget::InputSlider { .. } => "inputSlider",
        Widget::InputStepper { .. } => "inputStepper",
        Widget::InputNote { .. } => "inputNote",
        Widget::InputImage { .. } => "inputImage",
        Widget::Variable { .. } => "variable",
        Widget::OutputPreview { .. } => "outputPreview",
        Widget::OutputAction { .. } => "outputAction",
        Widget::OutputExport { .. } => "outputExport",
        Widget::Cluster { .. } => "cluster",
    }
}

fn widget_tree_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { id, neuronKind, .. } => format!("{id} ({neuronKind})"),
        Widget::InputSlider { id, .. } => format!("{id} (slider)"),
        Widget::InputNote { id, .. } => format!("{id} (note)"),
        Widget::OutputPreview { id, .. } => format!("{id} (preview)"),
        Widget::Variable { id, name, .. } => format!("{id} ({name})"),
        widget => format!("{} ({})", widget_id(widget), widget_kind_label(widget)),
    }
}

fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputStepper { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn flow_widget_descriptor(kind: &str, neuron_kind: Option<&str>) -> Value {
    if kind == "neuron" {
        json!({ "kind": "neuron", "neuronKind": neuron_kind.unwrap_or(kind) })
    } else {
        json!({ "kind": kind })
    }
}

fn flow_widget_drag_data(descriptor: &Value) -> HashMap<String, String> {
    let mut drag_data = HashMap::new();
    drag_data.insert(FLOW_WIDGET_DRAG_MIME.into(), descriptor.to_string());
    drag_data
}

fn tree_item_with_action_draggable(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    action: ActionDescriptor,
    descriptor: &Value,
) -> UiTreeItemNode {
    let mut item = tree_item_with_action(id, label, description, action);
    item.draggable = Some(true);
    item.drag_data = Some(flow_widget_drag_data(descriptor));
    item
}

fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the flow app; one field per label makes every locale combination compile-checked.
struct FlowPlayLabels {
    widgets: &'static str,
    synapses: &'static str,
    extensions: &'static str,
    extension_actions: &'static str,
    sources: &'static str,
    components: &'static str,
    sinks: &'static str,
    catalogue_slider: &'static str,
    catalogue_stepper: &'static str,
    catalogue_note: &'static str,
    catalogue_add: &'static str,
    catalogue_and: &'static str,
    catalogue_concat: &'static str,
    catalogue_preview: &'static str,
    catalogue_export: &'static str,
    extension_auto_layout: &'static str,
    extension_auto_evaluate: &'static str,
    extension_action_reorganize_canvas: &'static str,
    extension_action_evaluate_fixture: &'static str,
    canvas: &'static str,
    widget: &'static str,
    delete_selection: &'static str,
    window_main: &'static str,
    window_compiled: &'static str,
    window_generations: &'static str,
    window_generate_form: &'static str,
    window_generate_preview: &'static str,
}

const FLOW_LABELS_NATIVE_EN: FlowPlayLabels = FlowPlayLabels {
    widgets: "Widgets",
    synapses: "Synapses",
    extensions: "Extensions",
    extension_actions: "Extension Actions",
    sources: "Sources",
    components: "Components",
    sinks: "Sinks",
    catalogue_slider: "Slider",
    catalogue_stepper: "Stepper",
    catalogue_note: "Note",
    catalogue_add: "Add",
    catalogue_and: "And",
    catalogue_concat: "Concat",
    catalogue_preview: "Preview",
    catalogue_export: "Export",
    extension_auto_layout: "Auto Layout",
    extension_auto_evaluate: "Auto Evaluate",
    extension_action_reorganize_canvas: "Reorganize Canvas",
    extension_action_evaluate_fixture: "Evaluate Fixture",
    canvas: "Canvas",
    widget: "Widget",
    delete_selection: "Delete selection",
    window_main: "Flow",
    window_compiled: "DSL",
    window_generations: "Generations",
    window_generate_form: "Form",
    window_generate_preview: "Preview",
};

const FLOW_LABELS_NATIVE_DE: FlowPlayLabels = FlowPlayLabels {
    widgets: "Widgets",
    synapses: "Synapsen",
    extensions: "Erweiterungen",
    extension_actions: "Erweiterungsaktionen",
    sources: "Quellen",
    components: "Komponenten",
    sinks: "Senken",
    catalogue_slider: "Schieberegler",
    catalogue_stepper: "Schrittregler",
    catalogue_note: "Notiz",
    catalogue_add: "Addieren",
    catalogue_and: "Und",
    catalogue_concat: "Verketten",
    catalogue_preview: "Vorschau",
    catalogue_export: "Exportieren",
    extension_auto_layout: "Automatisches Layout",
    extension_auto_evaluate: "Automatisch Auswerten",
    extension_action_reorganize_canvas: "Leinwand neu anordnen",
    extension_action_evaluate_fixture: "Fixture auswerten",
    canvas: "Leinwand",
    widget: "Widget",
    delete_selection: "Auswahl löschen",
    window_main: "Flow",
    window_compiled: "DSL",
    window_generations: "Generationen",
    window_generate_form: "Formular",
    window_generate_preview: "Vorschau",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown locales fall back to native English.
fn flow_labels(view_state: &ViewState) -> &'static FlowPlayLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &FLOW_LABELS_NATIVE_DE
    } else {
        &FLOW_LABELS_NATIVE_EN
    }
}

/// 🗣️ Resolves a built-in extension's display name from its stable id; unknown ids fall back to the extension's native English name.
fn flow_extension_label(id: &str, name: &'static str, labels: &FlowPlayLabels) -> &'static str {
    match id {
        "auto-layout" => labels.extension_auto_layout,
        "auto-evaluate" => labels.extension_auto_evaluate,
        _ => name,
    }
}

/// 🗣️ Resolves a built-in extension action's display title from its stable action id; unknown ids fall back to the action's native English title.
fn flow_extension_action_title_label(action_id: &str, title: &'static str, labels: &FlowPlayLabels) -> &'static str {
    match action_id {
        "flow.extension.reorganize" => labels.extension_action_reorganize_canvas,
        "flow.extension.evaluate" => labels.extension_action_evaluate_fixture,
        _ => title,
    }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn build_document_tree(fixture: &FlowFixture, selected: &[String], labels: &FlowPlayLabels) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            tree_item_with_action(
                format!("flow-play-document.widget.{}", widget_id(widget)),
                widget_tree_label(widget),
                Some(widget_kind_label(widget).into()),
                flow_action("setSelection", Some(json!({ "ids": [widget_id(widget)] }))),
            )
        })
        .collect();
    let synapse_items: Vec<UiTreeItemNode> = fixture
        .synapses
        .iter()
        .map(|synapse| {
            UiTreeItemNode {
                id: format!("flow-play-document.synapse.{}", synapse.id),
                label: format!("{} → {}", synapse.from, synapse.to),
                description: Some(format!("{} → {}", synapse.from_port, synapse.to_port)),
                icon_id: None,
                selected: None,
                default_open: None,
                action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
            }
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "flow-play-document.widgets".into(),
                label: Some(labels.widgets.into()),
                default_open: Some(true),
                items: if widget_items.is_empty() {
                    vec![tree_item("flow-play-document.widgets.empty", "(none)")]
                } else {
                    widget_items
                },
            },
            UiTreeSectionNode {
                id: "flow-play-document.synapses".into(),
                label: Some(labels.synapses.into()),
                default_open: Some(false),
                items: if synapse_items.is_empty() {
                    vec![tree_item("flow-play-document.synapses.empty", "(none)")]
                } else {
                    synapse_items
                },
            },
        ],
        selected_ids: Some(selected.iter().map(|id| format!("flow-play-document.widget.{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn build_catalogue_tree(envelope: &FlowPlayEnvelope, labels: &FlowPlayLabels) -> UiNode {
    let host = host_from_envelope(envelope);
    let sections: Vec<Value> = host
        .catalogue_json()
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default();
    let tree_sections: Vec<UiTreeSectionNode> = sections
        .iter()
        .filter_map(|section| {
            let id = section.get("id")?.as_str()?.to_string();
            let title = section
                .get("title")
                .and_then(|value| value.as_str())
                .unwrap_or(&id)
                .to_string();
            let items: Vec<UiTreeItemNode> = section
                .get("items")
                .and_then(|value| value.as_array())
                .map(|entries| {
                    entries
                        .iter()
                        .filter_map(|entry| {
                            let kind = entry.get("kind")?.as_str()?;
                            let label = entry
                                .get("name")
                                .or_else(|| entry.get("abbreviation"))
                                .and_then(|value| value.as_str())
                                .unwrap_or(kind);
                            let descriptor = if kind == "neuron" {
                                flow_widget_descriptor(
                                    "neuron",
                                    entry.get("neuronKind").and_then(|value| value.as_str()),
                                )
                            } else {
                                flow_widget_descriptor(kind, None)
                            };
                            let action = flow_action("addWidget", Some(descriptor.clone()));
                            Some(tree_item_with_action_draggable(
                                format!("flow-play-catalogue.{id}.{kind}.{label}"),
                                label,
                                Some(kind.to_string()),
                                action,
                                &descriptor,
                            ))
                        })
                        .collect()
                })
                .unwrap_or_default();
            Some(UiTreeSectionNode {
                id: format!("flow-play-catalogue.{id}"),
                label: Some(title),
                default_open: Some(true),
                items,
            })
        })
        .collect();
    let mut tree_sections = if tree_sections.is_empty() { catalogue_tree_sections_fallback(labels) } else { tree_sections };
    tree_sections.extend(flow_extensions_tree_sections(&envelope.runtime, labels));
    UiNode::Tree(UiTreeNode {
        sections: tree_sections,
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

/// 🧩 Installed/enabled extension palette plus actions surfaced by active extensions.
fn flow_extensions_tree_sections(runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> Vec<UiTreeSectionNode> {
    let installed: Vec<UiTreeItemNode> = FLOW_EXTENSIONS
        .iter()
        .map(|(id, name, _, _, _)| {
            let enabled = runtime.extension_enabled.get(*id).copied().unwrap_or(false);
            tree_item_with_action(
                format!("flow-play-extensions.{id}"),
                flow_extension_label(id, name, labels),
                Some(if enabled { "enabled".into() } else { "disabled".into() }),
                flow_action("toggleExtension", Some(json!({ "id": id, "enabled": !enabled }))),
            )
        })
        .collect();
    let actions: Vec<UiTreeItemNode> = FLOW_EXTENSIONS
        .iter()
        .filter(|(id, ..)| runtime.extension_enabled.get(*id).copied().unwrap_or(false))
        .map(|(_, _, action_id, title, _)| {
            tree_item_with_action(
                format!("flow-play-extensions.action.{action_id}"),
                flow_extension_action_title_label(action_id, title, labels),
                Some((*action_id).into()),
                flow_action("runExtensionAction", Some(json!({ "actionId": action_id }))),
            )
        })
        .collect();
    let mut sections = vec![UiTreeSectionNode {
        id: "flow-play-extensions.installed".into(),
        label: Some(labels.extensions.into()),
        default_open: Some(false),
        items: installed,
    }];
    if !actions.is_empty() {
        sections.push(UiTreeSectionNode {
            id: "flow-play-extensions.actions".into(),
            label: Some(labels.extension_actions.into()),
            default_open: Some(false),
            items: actions,
        });
    }
    sections
}

fn catalogue_tree_sections_fallback(labels: &FlowPlayLabels) -> Vec<UiTreeSectionNode> {
    let sources = [("inputSlider", labels.catalogue_slider), ("inputStepper", labels.catalogue_stepper), ("inputNote", labels.catalogue_note)];
    let components = [("math.add", labels.catalogue_add), ("logic.and", labels.catalogue_and), ("text.concat", labels.catalogue_concat)];
    let sinks = [("outputPreview", labels.catalogue_preview), ("outputExport", labels.catalogue_export)];
    vec![
        UiTreeSectionNode {
            id: "flow-play-catalogue.sources".into(),
            label: Some(labels.sources.into()),
            default_open: Some(true),
            items: sources
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor(kind, None);
                    tree_item_with_action_draggable(
                        format!("flow-play-catalogue.source.{kind}"),
                        *label,
                        Some((*kind).into()),
                        flow_action("addWidget", Some(descriptor.clone())),
                        &descriptor,
                    )
                })
                .collect(),
        },
        UiTreeSectionNode {
            id: "flow-play-catalogue.components".into(),
            label: Some(labels.components.into()),
            default_open: Some(true),
            items: components
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor("neuron", Some(kind));
                    tree_item_with_action_draggable(
                        format!("flow-play-catalogue.component.{kind}"),
                        *label,
                        Some((*kind).into()),
                        flow_action("addWidget", Some(descriptor.clone())),
                        &descriptor,
                    )
                })
                .collect(),
        },
        UiTreeSectionNode {
            id: "flow-play-catalogue.sinks".into(),
            label: Some(labels.sinks.into()),
            default_open: Some(false),
            items: sinks
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor(kind, None);
                    tree_item_with_action_draggable(
                        format!("flow-play-catalogue.sink.{kind}"),
                        *label,
                        Some((*kind).into()),
                        flow_action("addWidget", Some(descriptor.clone())),
                        &descriptor,
                    )
                })
                .collect(),
        },
    ]
}

fn canvas_settings_field_group(runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> UiInspectorFieldGroup {
    let lod_items: Vec<UiSelectItem> = std::iter::once(UiSelectItem { value: FLOW_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() })
        .chain(
            serde_json::from_str::<Vec<Value>>(&dag_lod_scale_json())
                .unwrap_or_default()
                .into_iter()
                .filter_map(|lod| {
                    let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
                    let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
                    Some(UiSelectItem { value: id, label: name })
                }),
        )
        .collect();
    UiInspectorFieldGroup {
        id: "flow-play-inspector.canvas".into(),
        label: labels.canvas.into(),
        default_open: Some(true),
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.lod-mode".into(),
                label: "LOD Mode".into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "flow-play-inspector.lod-mode.select".into(),
                    value: runtime.lod_mode.clone(),
                    items: lod_items,
                    placeholder: None,
                    on_change: flow_action("setLodMode", None),
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.proximity-distance".into(),
                label: "Proximity Distance".into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "flow-play-inspector.proximity-distance.input".into(),
                    input_kind: "number".into(),
                    value: runtime.proximity_distance.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: flow_action("setProximityDistance", None),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
        ],
    }
}

fn build_inspector_tree(fixture: &FlowFixture, selected: &[String], runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> UiNode {
    if selected.is_empty() {
        return ui_inspector_groups_to_tree(&[canvas_settings_field_group(runtime, labels)]);
    }
    let widgets: Vec<&Widget> = selected
        .iter()
        .filter_map(|id| fixture.widgets.iter().find(|widget| widget_id(widget) == id))
        .collect();
    if widgets.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "flow-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Widget not found")],
        }]);
    }
    let widget_ids: Vec<String> = widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if widgets.iter().all(|widget| matches!(widget, Widget::InputSlider { .. })) {
        let mixed = ui_inspector_mixed_number(&widgets.iter().map(|widget| match widget { Widget::InputSlider { value, .. } => *value, _ => 0.0 }).collect::<Vec<_>>());
        groups.push(UiInspectorFieldGroup {
            id: "flow-play-inspector.kind.inputSlider".into(),
            label: "inputSlider".into(),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.slider-value".into(),
                label: "Value".into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "flow-play-inspector.slider-value.input".into(),
                    input_kind: "number".into(),
                    value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
                    placeholder: if mixed.uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                    commit: None,
                    on_change: flow_action("patchFlowWidgets", Some(json!({ "widgetIds": widget_ids, "field": "value" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            })],
        });
    }
    if widgets.iter().all(|widget| matches!(widget, Widget::InputNote { .. })) {
        let mixed = ui_inspector_mixed_text(&widgets.iter().map(|widget| match widget { Widget::InputNote { text, .. } => text.clone(), _ => String::new() }).collect::<Vec<_>>());
        groups.push(UiInspectorFieldGroup {
            id: "flow-play-inspector.kind.inputNote".into(),
            label: "inputNote".into(),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.note-text".into(),
                label: "Text".into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "flow-play-inspector.note-text.input".into(),
                    input_kind: "text".into(),
                    value: mixed.value,
                    placeholder: mixed.placeholder,
                    commit: Some("blur".into()),
                    on_change: flow_action("patchFlowWidgets", Some(json!({ "widgetIds": widget_ids, "field": "text" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            })],
        });
    }
    let kind_mixed = ui_inspector_mixed_text(&widgets.iter().map(|widget| widget_kind_label(widget).to_string()).collect::<Vec<_>>());
    let mut base_fields = vec![ui_inspector_readonly_field(
        "flow-play-inspector.kind",
        "Kind",
        if kind_mixed.placeholder.is_none() { widget_kind_label(widgets[0]).to_string() } else { "—".into() },
    )];
    if widget_ids.len() == 1 {
        base_fields.insert(
            0,
            UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.id".into(),
                label: "Id".into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "flow-play-inspector.id.input".into(),
                    input_kind: "text".into(),
                    value: widget_ids[0].clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: flow_action("renameFlowWidget", Some(json!({ "oldId": widget_ids[0] }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
        );
    }
    groups.push(UiInspectorFieldGroup {
        id: "flow-play-inspector.base".into(),
        label: labels.widget.into(),
        default_open: None,
        fields: base_fields,
    });
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_graph(envelope: &FlowPlayEnvelope, labels: &FlowPlayLabels) -> UiNode {
    let host = host_from_envelope(envelope);
    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&envelope.fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let fixture_json = serde_json::to_string(&envelope.fixture).ok();
    let selection_json = if envelope.runtime.selected_node_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&envelope.runtime.selected_node_ids).ok()
    };
    let flow_extras = flow_backed_node_graph_extras(&envelope.fixture, &envelope.runtime.lod_mode, envelope.runtime.proximity_distance);
    let context_menu_json = json!([{
        "id": "delete-selection",
        "label": labels.delete_selection,
        "action": "nodeGraphEdit",
        "args": { "ops": [{ "op": "deleteSelection" }] },
    }])
    .to_string();
    build_node_graph_scene(
        FLOW_PLAY_SURFACE_MAIN,
        FLOW_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators_json: flow_extras.operators_json,
            context_menu_json: Some(context_menu_json),
            find_items_json: None,
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json.or(fixture_json),
            selection_json,
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_compiled_dag(envelope: &FlowPlayEnvelope) -> UiNode {
    let host = host_from_envelope(envelope);
    build_text_editor_scene(
        FLOW_PLAY_SURFACE_COMPILED,
        FLOW_PLAY_APP_ID,
        TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None),
    )
}

fn evaluate_generation_preview(envelope: &FlowPlayEnvelope, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(&envelope.fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| envelope.fixture.clone());
    let mut host = FlowHost::from_fixture(fixture);
    seed_host_catalogue(&mut host, &envelope.runtime.catalogue_sections_json);
    host.evaluate().unwrap_or_default()
}

fn refresh_generation_preview(envelope: &mut FlowPlayEnvelope) {
    let Some(generation) = selected_generation(&envelope.generation) else {
        envelope.generation.preview_text = None;
        return;
    };
    let preview = evaluate_generation_preview(envelope, &generation.values);
    envelope.generation.preview_text = Some(preview.clone());
    envelope.runtime.last_eval_json = preview;
}

fn render_generate_generations(envelope: &FlowPlayEnvelope) -> UiNode {
    render_generations_tree(
        FLOW_PLAY_APP_ID,
        "flow-play-generate",
        &envelope.generation.generations,
        envelope.generation.selected_generation_id.as_deref(),
    )
}

fn render_generate_form(envelope: &FlowPlayEnvelope) -> UiNode {
    let spec = flow_fixture_to_form_spec(&envelope.fixture);
    let Some(generation) = selected_generation(&envelope.generation) else {
        return ui_text("Add a generation to edit input values.");
    };
    render_generation_form_body(
        &spec,
        &generation.values,
        FLOW_PLAY_APP_ID,
        "updateGenerationValues",
        &generation.id,
    )
}

fn render_generate_preview(envelope: &FlowPlayEnvelope) -> UiNode {
    let text = envelope
        .generation
        .preview_text
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("(evaluate a generation to preview output)");
    render_generation_preview_text(FLOW_PLAY_SURFACE_GENERATE_PREVIEW, FLOW_PLAY_APP_ID, text)
}
//#endregion 🔖Render

//#region 🔖FlowPlayApp
struct FlowPlayApp {
    host: Option<FlowHost>,
}

impl FlowPlayApp {
    fn host_for(&mut self, envelope: &FlowPlayEnvelope) -> &mut FlowHost {
        let replace = self
            .host
            .as_ref()
            .map_or(true, |host| host.fixture != envelope.fixture);
        if replace {
            self.host = Some(host_from_envelope(envelope));
        }
        self.host.as_mut().expect("flow host")
    }
}

impl PluginApp for FlowPlayApp {
    fn app_id(&self) -> &str {
        FLOW_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("flow envelope json")
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        let host = self.host_for(&envelope);
        match action {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                let ids = args
                    .and_then(|value| value.get("nodeIds"))
                    .and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok())
                    .unwrap_or_else(|| selection_ids(args));
                envelope.runtime.selected_node_ids = ids;
                return vec![set_document_op(&envelope)];
            }
            "graphPointerDown" => {
                envelope.runtime.selected_node_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    host.begin_change();
                    if host.move_widget(node_id, x, y).is_ok() {
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "undo" => {
                if host.undo() {
                    envelope.fixture = host.fixture.clone();
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                if host.redo() {
                    envelope.fixture = host.fixture.clone();
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "evaluate" => {
                host.clear_computing_widget_ids();
                if let Ok(eval_json) = host.evaluate() {
                    envelope.fixture = host.fixture.clone();
                    envelope.runtime.last_eval_json = eval_json.clone();
                    host.apply_eval_outputs_json(&eval_json);
                    host.clear_computing_widget_ids();
                    return vec![set_document_op(&envelope)];
                }
            }
            "removeWidget" => {
                let widget_id = args
                    .and_then(|value| value.get("widgetId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str());
                if let Some(widget_id) = widget_id {
                    if host.remove_widget(widget_id).is_ok() {
                        envelope.runtime.selected_node_ids.retain(|id| id != widget_id);
                        if let Ok(eval_json) = host.evaluate() {
                            envelope.runtime.last_eval_json = eval_json;
                        }
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "deleteSelection" => {
                sync_host_selection(host, &envelope.runtime.selected_node_ids);
                if host.delete_selection().is_ok() {
                    envelope.runtime.selected_node_ids.clear();
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    envelope.fixture = host.fixture.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            "disconnect" => {
                let synapse_id = args
                    .and_then(|value| value.get("synapseId"))
                    .or_else(|| args.and_then(|value| value.get("edgeId")))
                    .and_then(|value| value.as_str());
                if let Some(synapse_id) = synapse_id {
                    if host.disconnect(synapse_id).is_ok() {
                        if let Ok(eval_json) = host.evaluate() {
                            envelope.runtime.last_eval_json = eval_json;
                        }
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str());
                let from_port = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str());
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str());
                let to_port = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str());
                if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                    if host.connect_ports(from, from_port, to, to_port).is_ok() {
                        if let Ok(eval_json) = host.evaluate() {
                            envelope.runtime.last_eval_json = eval_json;
                        }
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "addWidget" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                let descriptor = match kind {
                    "neuron" => {
                        let neuron_kind = args.and_then(|value| value.get("neuronKind")).and_then(|value| value.as_str()).unwrap_or("math.add");
                        json!({ "kind": "neuron", "neuronKind": neuron_kind }).to_string()
                    }
                    other => json!({ "kind": other }).to_string(),
                };
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                if let Ok(id) = host.add_widget(&descriptor, x, y) {
                    envelope.runtime.selected_node_ids = vec![id];
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    envelope.fixture = host.fixture.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            "reorganize" => {
                if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                    envelope.fixture = host.fixture.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            "patchFlowWidgets" => {
                host.begin_change();
                let widget_ids: Vec<String> = args
                    .and_then(|value| value.get("widgetIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value"));
                for widget in envelope.fixture.widgets.iter_mut() {
                    if !widget_ids.contains(&widget_id(widget).to_string()) {
                        continue;
                    }
                    match (field, widget) {
                        ("value", Widget::InputSlider { value: ref mut slider_value, .. }) => {
                            if let Some(value) = raw_value.and_then(|value| value.as_f64()) {
                                *slider_value = value;
                            }
                        }
                        ("text", Widget::InputNote { text: ref mut note_text, .. }) => {
                            if let Some(value) = raw_value.and_then(|value| value.as_str()) {
                                *note_text = value.into();
                            }
                        }
                        _ => {}
                    }
                }
                host.set_fixture_preserving_history(envelope.fixture.clone());
                if let Ok(eval_json) = host.evaluate() {
                    envelope.runtime.last_eval_json = eval_json;
                }
                envelope.fixture = host.fixture.clone();
                return vec![set_document_op(&envelope)];
            }
            "renameFlowWidget" => {
                host.begin_change();
                let old_id = args.and_then(|value| value.get("oldId")).and_then(|value| value.as_str());
                let new_id = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                if let (Some(old_id), Some(new_id)) = (old_id, new_id) {
                    let trimmed = new_id.trim();
                    if !trimmed.is_empty() && trimmed != old_id && !envelope.fixture.widgets.iter().any(|widget| widget_id(widget) == trimmed) {
                        for widget in envelope.fixture.widgets.iter_mut() {
                            if widget_id(widget) == old_id {
                                match widget {
                                    Widget::Neuron { id, .. }
                                    | Widget::InputSlider { id, .. }
                                    | Widget::InputStepper { id, .. }
                                    | Widget::InputNote { id, .. }
                                    | Widget::InputImage { id, .. }
                                    | Widget::Variable { id, .. }
                                    | Widget::OutputPreview { id, .. }
                                    | Widget::OutputAction { id, .. }
                                    | Widget::OutputExport { id, .. }
                                    | Widget::Cluster { id, .. } => *id = trimmed.to_string(),
                                }
                            }
                        }
                        for synapse in envelope.fixture.synapses.iter_mut() {
                            if synapse.from == old_id {
                                synapse.from = trimmed.into();
                            }
                            if synapse.to == old_id {
                                synapse.to = trimmed.into();
                            }
                        }
                        envelope.runtime.selected_node_ids = vec![trimmed.into()];
                        host.set_fixture_preserving_history(envelope.fixture.clone());
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "nodeGraphHover" => {
                return Vec::new();
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<CameraJson>(viewport_json) {
                        envelope.fixture.camera = camera;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "nodeGraphEdit" | "spotlightCommit" => {
                let ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                if ops.is_empty() && action == "spotlightCommit" {
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                        envelope.fixture = host.fixture.clone();
                        return vec![set_document_op(&envelope)];
                    }
                }
                let mut changed = false;
                for op in ops {
                    let op_name = op.get("op").and_then(|value| value.as_str()).unwrap_or("");
                    match op_name {
                        "setFixture" => {
                            if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                if let Ok(fixture) = serde_json::from_str::<FlowFixture>(fixture_json) {
                                    host.begin_change();
                                    envelope.fixture = fixture.clone();
                                    host.set_fixture_preserving_history(fixture);
                                    changed = true;
                                }
                            }
                        }
                        "deleteSelection" => {
                            sync_host_selection(host, &envelope.runtime.selected_node_ids);
                            if host.delete_selection().is_ok() {
                                envelope.runtime.selected_node_ids.clear();
                                changed = true;
                            }
                        }
                        "connect" => {
                            let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                            let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                            let to = op.get("targetNodeId").and_then(|value| value.as_str());
                            let to_port = op.get("targetPortId").and_then(|value| value.as_str());
                            if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                if host.connect_ports(from, from_port, to, to_port).is_ok() {
                                    changed = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if changed {
                    if let Ok(eval_json) = host.evaluate() {
                        envelope.runtime.last_eval_json = eval_json;
                    }
                    envelope.fixture = host.fixture.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                let spec = flow_fixture_to_form_spec(&envelope.fixture);
                if handle_generation_action(action, args, &mut envelope.generation, &spec, FLOW_PLAY_APP_ID) {
                    if action == "addGeneration" && envelope.generation.generations.len() == 1 {
                        refresh_generation_preview(&mut envelope);
                    } else if action == "selectGeneration" || action == "updateGenerationValues" {
                        refresh_generation_preview(&mut envelope);
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                    if mode == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(mode).is_some() {
                        envelope.runtime.lod_mode = mode.into();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setProximityDistance" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.proximity_distance = value.max(0.0);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setCatalogueSections" => {
                if let Some(sections) = args.and_then(|value| value.get("sections")) {
                    envelope.runtime.catalogue_sections_json = sections.to_string();
                    return vec![set_document_op(&envelope)];
                }
            }
            "toggleExtension" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool());
                if let (Some(id), Some(enabled)) = (id, enabled) {
                    envelope.runtime.extension_enabled.insert(id.into(), enabled);
                    return vec![set_document_op(&envelope)];
                }
            }
            "runExtensionAction" => {
                let action_id = args.and_then(|value| value.get("actionId")).and_then(|value| value.as_str());
                if let Some(action_id) = action_id {
                    if let Some((id, _, _, _, effect)) = FLOW_EXTENSIONS.iter().find(|(_, _, entry_action_id, ..)| *entry_action_id == action_id) {
                        if envelope.runtime.extension_enabled.get(*id).copied().unwrap_or(false) {
                            match *effect {
                                "reorganize" => {
                                    if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                                        envelope.fixture = host.fixture.clone();
                                    }
                                }
                                "evaluate" => {
                                    if let Ok(eval_json) = host.evaluate() {
                                        envelope.runtime.last_eval_json = eval_json;
                                        envelope.fixture = host.fixture.clone();
                                    }
                                }
                                _ => {}
                            }
                            return vec![set_document_op(&envelope)];
                        }
                    }
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let labels = flow_labels(view_state);
        match body_key {
            FLOW_PLAY_BODY_MAIN => render_main_graph(&envelope, labels),
            FLOW_PLAY_BODY_COMPILED => render_compiled_dag(&envelope),
            FLOW_PLAY_BODY_GENERATIONS => render_generate_generations(&envelope),
            FLOW_PLAY_BODY_GENERATE_FORM => render_generate_form(&envelope),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&envelope),
            FLOW_PLAY_BODY_DOCUMENT => build_document_tree(&envelope.fixture, &envelope.runtime.selected_node_ids, labels),
            FLOW_PLAY_BODY_CATALOGUE => build_catalogue_tree(&envelope, labels),
            FLOW_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_node_ids, &envelope.runtime, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = flow_labels(view_state);
        semio_framework_plugin::AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([
                (FLOW_PLAY_WINDOW_MAIN.to_string(), labels.window_main.to_string()),
                (FLOW_PLAY_WINDOW_COMPILED.to_string(), labels.window_compiled.to_string()),
                (FLOW_PLAY_WINDOW_GENERATIONS.to_string(), labels.window_generations.to_string()),
                (FLOW_PLAY_WINDOW_GENERATE_FORM.to_string(), labels.window_generate_form.to_string()),
                (FLOW_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), labels.window_generate_preview.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::new(),
        }
    }
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| {
            args.and_then(|value| value.get("nodeId"))
                .and_then(|value| value.as_str())
                .map(|id| vec![id.to_string()])
        })
        .unwrap_or_default()
}
//#endregion 🔖FlowPlayApp

//#region 🔖Manifest
fn create_flow_app() -> App {
    App::from_builder(
        App::builder(FLOW_PLAY_APP_ID, "Flow").document(["semio", "flow"])
            .icon_id("flow")
            .mode("edit", "Edit")
            .mode("generate", "Generate")
            .default_mode_id("edit")
            .window_kind(FLOW_PLAY_WINDOW_MAIN, "Flow", FLOW_PLAY_BODY_MAIN, SurfaceKind::NodeGraph)
            .window_kind(FLOW_PLAY_WINDOW_COMPILED, "DSL", FLOW_PLAY_BODY_COMPILED, SurfaceKind::NodeGraph)
            .window_kind(FLOW_PLAY_WINDOW_GENERATIONS, "Generations", FLOW_PLAY_BODY_GENERATIONS, SurfaceKind::Canvas2d)
            .window_kind(FLOW_PLAY_WINDOW_GENERATE_FORM, "Form", FLOW_PLAY_BODY_GENERATE_FORM, SurfaceKind::Canvas2d)
            .window_kind(
                FLOW_PLAY_WINDOW_GENERATE_PREVIEW,
                "Preview",
                FLOW_PLAY_BODY_GENERATE_PREVIEW,
                SurfaceKind::Canvas2d,
            )
            .default_layout(create_default_layout(
                &[FLOW_PLAY_WINDOW_MAIN.into(), FLOW_PLAY_WINDOW_COMPILED.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Flow".into(), "DSL".into()]),
            ))
            .named_layout(create_named_layout(
                "flow-generate",
                "Generate",
                create_default_layout(
                    &[
                        FLOW_PLAY_WINDOW_GENERATIONS.into(),
                        FLOW_PLAY_WINDOW_GENERATE_FORM.into(),
                        FLOW_PLAY_WINDOW_GENERATE_PREVIEW.into(),
                    ],
                    "row",
                    Some(&[22.0, 43.0, 35.0]),
                    Some(&["Generations".into(), "Form".into(), "Preview".into()]),
                ),
                "builtin",
                Some("sparkles".into()),
                None,
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                FLOW_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                FLOW_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                FLOW_PLAY_BODY_INSPECTOR,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&default_envelope()).unwrap())
    .program("flow", "Flow", "graph")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("flow", "Flow", "0.1.0").register_app(create_flow_app(), || Box::new(FlowPlayApp { host: None }))
}

semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Manifest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_node_graph_scene() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let node = app.render(FLOW_PLAY_BODY_MAIN, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_compiled_wire_editor() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let node = app.render(FLOW_PLAY_BODY_COMPILED, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn default_fixture_has_widgets() {
        let envelope = default_envelope();
        assert!(!envelope.fixture.widgets.is_empty());
    }

    #[test]
    fn document_lists_widgets() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let node = app.render(FLOW_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("flow-play-document.widgets"));
    }

    #[test]
    fn catalogue_lists_module_operators() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let node = app.render(FLOW_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("flow-play-catalogue.math"), "expected math module section: {json}");
        assert!(json.contains("math.add"), "expected math.add operator: {json}");
    }

    #[test]
    fn catalogue_items_export_flow_widget_drag_payload() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let node = app.render(FLOW_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(FLOW_WIDGET_DRAG_MIME), "missing drag mime: {json}");
        assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
    }

    #[test]
    fn evaluate_updates_preview_state() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("evaluate", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: FlowPlayEnvelope = serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert!(!updated.runtime.last_eval_json.is_empty());
    }

    #[test]
    fn undo_restores_fixture_after_add_widget() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let before: FlowPlayEnvelope = serde_json::from_str(&document).unwrap();
        let count_before = before.fixture.widgets.len();
        let ops = app.handle_action_patch_ops(
            "addWidget",
            Some(&json!({ "kind": "inputNote", "x": 40.0, "y": 40.0 })),
            &document,
            &ViewState::default(),
        );
        assert_eq!(ops.len(), 1);
        let after_add = serde_json::to_string(&serde_json::from_str::<Value>(&ops[0]).unwrap()["document"]).unwrap();
        let undo_ops = app.handle_action_patch_ops("undo", None, &after_add, &ViewState::default());
        let restored: FlowPlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&undo_ops[0]).unwrap()["document"].clone()).unwrap();
        assert_eq!(restored.fixture.widgets.len(), count_before);
    }

    #[test]
    fn generate_mode_renders_three_surfaces() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let generations = app.render(FLOW_PLAY_BODY_GENERATIONS, &document, &ViewState::default());
        let form = app.render(FLOW_PLAY_BODY_GENERATE_FORM, &document, &ViewState::default());
        let preview = app.render(FLOW_PLAY_BODY_GENERATE_PREVIEW, &document, &ViewState::default());
        assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
        assert!(serde_json::to_string(&form).unwrap().contains("Add a generation"));
        assert!(serde_json::to_string(&preview).unwrap().contains("text-editor"));
    }

    #[test]
    fn add_generation_evaluates_preview() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("addGeneration", None, &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let updated: FlowPlayEnvelope =
            serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap();
        assert_eq!(updated.generation.generations.len(), 1);
        assert!(updated.generation.preview_text.as_deref().unwrap_or("").len() > 2);
    }

    fn document_after(app: &mut FlowPlayApp, document: &str, action: &str, args: Option<Value>) -> FlowPlayEnvelope {
        let ops = app.handle_action_patch_ops(action, args.as_ref(), document, &ViewState::default());
        assert_eq!(ops.len(), 1, "action {action} produced no op");
        serde_json::from_value(serde_json::from_str::<Value>(&ops[0]).unwrap()["document"].clone()).unwrap()
    }

    #[test]
    fn set_lod_mode_rejects_unknown_and_accepts_known() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        assert!(app.handle_action_patch_ops("setLodMode", Some(&json!({ "mode": "bogus" })), &document, &ViewState::default()).is_empty());
        let next = document_after(&mut app, &document, "setLodMode", Some(json!({ "mode": "micro" })));
        assert_eq!(next.runtime.lod_mode, "micro");
        let node_graph = app.render(FLOW_PLAY_BODY_MAIN, &serde_json::to_string(&next).unwrap(), &ViewState::default());
        let json = serde_json::to_string(&node_graph).unwrap();
        assert!(json.contains("\\\"forcedLabel\\\":\\\"micro\\\"") || json.contains("\"forcedLabel\":\"micro\""));
    }

    #[test]
    fn set_proximity_distance_clamps_to_zero() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let next = document_after(&mut app, &document, "setProximityDistance", Some(json!({ "value": -5.0 })));
        assert_eq!(next.runtime.proximity_distance, 0.0);
        let next = document_after(&mut app, &document, "setProximityDistance", Some(json!({ "value": 160.0 })));
        assert_eq!(next.runtime.proximity_distance, 160.0);
    }

    #[test]
    fn set_catalogue_sections_persists_and_merges_into_catalogue() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let sections = json!([{
            "id": "custom",
            "title": "Custom",
            "items": [{
                "kind": "neuron",
                "neuronKind": "math.add",
                "name": "CustomAdd",
                "abbreviation": "Add",
                "icon": "emoji:➕",
                "summary": "Custom add operator",
            }],
        }]);
        let next = document_after(&mut app, &document, "setCatalogueSections", Some(json!({ "sections": sections })));
        assert!(next.runtime.catalogue_sections_json.contains("custom"));
        let catalogue = app.render(FLOW_PLAY_BODY_CATALOGUE, &serde_json::to_string(&next).unwrap(), &ViewState::default());
        assert!(serde_json::to_string(&catalogue).unwrap().contains("CustomAdd"));
    }

    #[test]
    fn toggle_extension_and_run_action_reorganizes_fixture() {
        let mut app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        assert!(app
            .handle_action_patch_ops("runExtensionAction", Some(&json!({ "actionId": "flow.extension.reorganize" })), &document, &ViewState::default())
            .is_empty());
        let toggled = document_after(&mut app, &document, "toggleExtension", Some(json!({ "id": "auto-layout", "enabled": true })));
        assert_eq!(toggled.runtime.extension_enabled.get("auto-layout"), Some(&true));
        let toggled_json = serde_json::to_string(&toggled).unwrap();
        let ran = document_after(&mut app, &toggled_json, "runExtensionAction", Some(json!({ "actionId": "flow.extension.reorganize" })));
        assert_eq!(ran.fixture.widgets.len(), toggled.fixture.widgets.len());
    }

    #[test]
    fn flow_labels_resolve_native_english_by_default() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let document_tree = app.render(FLOW_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Widgets"), "expected native English Widgets section: {document_json}");
        assert!(document_json.contains("Synapses"), "expected native English Synapses section: {document_json}");
        let catalogue = app.render(FLOW_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Extensions"), "expected native English Extensions section: {catalogue_json}");
        assert!(catalogue_json.contains("Auto Layout"), "expected native English extension name: {catalogue_json}");
    }

    #[test]
    fn flow_labels_resolve_german_locale() {
        let app = FlowPlayApp { host: None };
        let document = app.initial_document_json();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let document_tree = app.render(FLOW_PLAY_BODY_DOCUMENT, &document, &view_state);
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Synapsen"), "expected German Synapsen section: {document_json}");
        let catalogue = app.render(FLOW_PLAY_BODY_CATALOGUE, &document, &view_state);
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Erweiterungen"), "expected German Erweiterungen section: {catalogue_json}");
        assert!(catalogue_json.contains("Automatisches Layout"), "expected German extension name: {catalogue_json}");
    }
}
