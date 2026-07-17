//! 🌊 Flow plugin — declarative flow play app bundled as a hot-swappable WASM component.

use flow_core::{
    dag::{dag_lod_scale_json, DagDrawLod, DagFixture},
    flow_backed_node_graph_extras, flow_fixture_ops, flow_operator_catalogue_json,
    FLOW_DOCUMENT_SCHEMA, FLOW_LOD_MODE_AUTOMATIC,
    forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec},
    CameraJson, FlowFixture, FlowHost, FlowOp, Widget,
};
use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_node_graph_scene, build_text_editor_scene, create_default_layout, create_named_layout,
    handle_generation_action, render_generation_form_body, render_generation_preview_text, render_generations_tree,
    selected_generation, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, ActionDescriptor, DocumentApp,
    DocumentView, GenerationPlayState,
    NodeGraphScene, TextEditorScene, UiFieldNode, UiInputNode,
    UiInspectorFieldGroup, UiNode, UiSelectItem, UiSelectNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
    ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID,     FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

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
/// 🎛️ Ephemeral view/config state — selection, camera, live eval preview, LOD/catalogue/extension
/// config, and the generate-mode exploration surface — lives in the app struct, never the document,
/// so panning, selecting, and previewing never pollute undo history.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct FlowPlayRuntime {
    selected_node_ids: Vec<String>,
    camera: CameraJson,
    last_eval_json: String,
    lod_mode: String,
    proximity_distance: f64,
    catalogue_sections_json: String,
    extension_enabled: HashMap<String, bool>,
    generation: GenerationPlayState,
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
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            last_eval_json: String::new(),
            lod_mode: default_flow_lod_mode(),
            proximity_distance: 0.0,
            catalogue_sections_json: default_catalogue_sections_json(),
            extension_enabled: HashMap::new(),
            generation: GenerationPlayState::default(),
        }
    }
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

fn host_from_fixture(fixture: &FlowFixture, runtime: &FlowPlayRuntime) -> FlowHost {
    let mut host = FlowHost::from_fixture(fixture.clone());
    seed_host_catalogue(&mut host, &runtime.catalogue_sections_json);
    apply_lod_and_proximity(&mut host, runtime);
    host
}

/// 🌉 Runs a `FlowHost` mutation over the current document fixture and diffs the result into granular
/// `FlowOp`s. `mutate` returns `true` if it changed the fixture; a non-mutating call yields no ops.
fn host_ops(
    fixture: &FlowFixture,
    runtime: &FlowPlayRuntime,
    mutate: impl FnOnce(&mut FlowHost) -> bool,
) -> Vec<FlowOp> {
    let mut host = host_from_fixture(fixture, runtime);
    if !mutate(&mut host) {
        return Vec::new();
    }
    flow_fixture_ops(fixture, &host.fixture)
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
    lod_mode: &'static str,
    automatic: &'static str,
    proximity_distance: &'static str,
    value: &'static str,
    text: &'static str,
    kind: &'static str,
    id: &'static str,
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
    lod_mode: "LOD Mode",
    automatic: "Automatic",
    proximity_distance: "Proximity Distance",
    value: "Value",
    text: "Text",
    kind: "Kind",
    id: "Id",
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
    lod_mode: "LOD-Modus",
    automatic: "Automatisch",
    proximity_distance: "Naeheabstand",
    value: "Wert",
    text: "Text",
    kind: "Art",
    id: "Id",
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

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_flow_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn flow_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("addWidget", "Add Widget", "Widget hinzufuegen"),
        ("removeWidget", "Remove Widget", "Widget entfernen"),
        ("deleteSelection", "Delete Selection", "Auswahl loeschen"),
        ("disconnect", "Disconnect", "Trennen"),
        ("connectMediaPorts", "Connect Ports", "Anschluesse verbinden"),
        ("moveMediaNode", "Move Node", "Knoten verschieben"),
        ("reorganize", "Reorganize", "Neu anordnen"),
        ("patchFlowWidgets", "Patch Widgets", "Widgets aktualisieren"),
        ("renameFlowWidget", "Rename Widget", "Widget umbenennen"),
        ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
        ("spotlightCommit", "Spotlight Commit", "Spotlight bestaetigen"),
        ("runExtensionAction", "Run Extension Action", "Erweiterungsaktion ausfuehren"),
        ("evaluate", "Evaluate", "Auswerten"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("selectNode", "Select Node", "Knoten auswaehlen"),
        ("nodeGraphSelect", "Node Graph Select", "Knotengraph auswaehlen"),
        ("nodeGraphHover", "Node Graph Hover", "Knotengraph-Hover"),
        ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrueckt"),
        ("nodeGraphViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
        ("setLodMode", "Set LOD Mode", "LOD-Modus festlegen"),
        ("setProximityDistance", "Set Proximity Distance", "Naeheabstand festlegen"),
        ("setCatalogueSections", "Set Catalogue Sections", "Katalogabschnitte festlegen"),
        ("toggleExtension", "Toggle Extension", "Erweiterung umschalten"),
        ("addGeneration", "Add Generation", "Generation hinzufuegen"),
        ("removeGeneration", "Remove Generation", "Generation entfernen"),
        ("selectGeneration", "Select Generation", "Generation auswaehlen"),
        ("renameGeneration", "Rename Generation", "Generation umbenennen"),
        ("updateGenerationValues", "Update Generation Values", "Generationswerte aktualisieren"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}
//#endregion 🔖CommandLabels

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

fn build_catalogue_tree(fixture: &FlowFixture, runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> UiNode {
    let host = host_from_fixture(fixture, runtime);
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
    tree_sections.extend(flow_extensions_tree_sections(runtime, labels));
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
    let lod_items: Vec<UiSelectItem> = std::iter::once(UiSelectItem { value: FLOW_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() })
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
                label: labels.lod_mode.into(),
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
                label: labels.proximity_distance.into(),
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
                label: labels.value.into(),
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
                label: labels.text.into(),
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
        labels.kind,
        if kind_mixed.placeholder.is_none() { widget_kind_label(widgets[0]).to_string() } else { "—".into() },
    )];
    if widget_ids.len() == 1 {
        base_fields.insert(
            0,
            UiNode::Field(UiFieldNode {
                id: "flow-play-inspector.id".into(),
                label: labels.id.into(),
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
fn render_main_graph(fixture: &FlowFixture, runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> UiNode {
    let host = host_from_fixture(fixture, runtime);
    let (nodes_json, edges_json) = fixture_to_media_graph(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&runtime.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let fixture_json = serde_json::to_string(fixture).ok();
    let selection_json = if runtime.selected_node_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&runtime.selected_node_ids).ok()
    };
    let flow_extras = flow_backed_node_graph_extras(fixture, &runtime.lod_mode, runtime.proximity_distance);
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

fn render_compiled_dag(fixture: &FlowFixture, runtime: &FlowPlayRuntime) -> UiNode {
    let host = host_from_fixture(fixture, runtime);
    build_text_editor_scene(
        FLOW_PLAY_SURFACE_COMPILED,
        FLOW_PLAY_APP_ID,
        TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None),
    )
}

fn evaluate_generation_preview(fixture: &FlowFixture, runtime: &FlowPlayRuntime, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    seed_host_catalogue(&mut host, &runtime.catalogue_sections_json);
    host.evaluate().unwrap_or_default()
}

/// 👁️ Re-evaluates the selected generation into the runtime preview text (ephemeral view state — never
/// a document op).
fn refresh_generation_preview(fixture: &FlowFixture, runtime: &mut FlowPlayRuntime) {
    let Some(generation) = selected_generation(&runtime.generation) else {
        runtime.generation.preview_text = None;
        return;
    };
    let preview = evaluate_generation_preview(fixture, runtime, &generation.values.clone());
    runtime.generation.preview_text = Some(preview.clone());
    runtime.last_eval_json = preview;
}

fn render_generate_generations(runtime: &FlowPlayRuntime) -> UiNode {
    render_generations_tree(
        FLOW_PLAY_APP_ID,
        "flow-play-generate",
        &runtime.generation.generations,
        runtime.generation.selected_generation_id.as_deref(),
    )
}

fn render_generate_form(fixture: &FlowFixture, runtime: &FlowPlayRuntime) -> UiNode {
    let spec = flow_fixture_to_form_spec(fixture);
    let Some(generation) = selected_generation(&runtime.generation) else {
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

fn render_generate_preview(runtime: &FlowPlayRuntime) -> UiNode {
    let text = runtime
        .generation
        .preview_text
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("(evaluate a generation to preview output)");
    render_generation_preview_text(FLOW_PLAY_SURFACE_GENERATE_PREVIEW, FLOW_PLAY_APP_ID, text)
}
//#endregion 🔖Render

//#region 🔖FlowPlayApp
#[derive(Default)]
struct FlowPlayApp {
    runtime: FlowPlayRuntime,
}

impl FlowPlayApp {
    /// 👁️ Parses the many selection-arg shapes (`ids`/`nodeIds` arrays or a single `nodeId`) into ids.
    fn parse_selection(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("ids").or_else(|| value.get("nodeIds")))
            .and_then(|value| {
                if value.is_array() {
                    serde_json::from_value(value.clone()).ok()
                } else {
                    value.as_str().map(|id| vec![id.to_string()])
                }
            })
            .or_else(|| {
                args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()])
            })
            .unwrap_or_default()
    }

    /// ✏️ Renames a widget id (rewiring synapses and layout) purely in the fixture; `None` if the target
    /// id is blank, unchanged, or already taken.
    fn renamed_fixture(fixture: &FlowFixture, old_id: &str, new_id: &str) -> Option<FlowFixture> {
        let trimmed = new_id.trim();
        if trimmed.is_empty() || trimmed == old_id || fixture.widgets.iter().any(|widget| widget_id(widget) == trimmed) {
            return None;
        }
        let mut next = fixture.clone();
        for widget in next.widgets.iter_mut() {
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
        for synapse in next.synapses.iter_mut() {
            if synapse.from == old_id {
                synapse.from = trimmed.into();
            }
            if synapse.to == old_id {
                synapse.to = trimmed.into();
            }
        }
        if let Some(layout) = next.layout.remove(old_id) {
            next.layout.insert(trimmed.into(), layout);
        }
        Some(next)
    }

    /// ✏️ Patches slider values / note text on the selected widgets in the fixture, returning the clone.
    fn patched_widgets_fixture(fixture: &FlowFixture, widget_ids: &[String], field: &str, raw_value: Option<&Value>) -> FlowFixture {
        let mut next = fixture.clone();
        for widget in next.widgets.iter_mut() {
            if !widget_ids.iter().any(|id| id == widget_id(widget)) {
                continue;
            }
            match (field, widget) {
                ("value", Widget::InputSlider { value, .. }) => {
                    if let Some(v) = raw_value.and_then(|value| value.as_f64()) {
                        *value = v;
                    }
                }
                ("text", Widget::InputNote { text, .. }) => {
                    if let Some(v) = raw_value.and_then(|value| value.as_str()) {
                        *text = v.into();
                    }
                }
                _ => {}
            }
        }
        next
    }
}

impl DocumentApp for FlowPlayApp {
    type Projection = FlowFixture;
    type Op = FlowOp;

    fn app_id(&self) -> &str {
        FLOW_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> FlowFixture {
        FlowFixture::default()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, FlowFixture>,
        _view_state: &ViewState,
    ) -> ActionEmit<FlowOp> {
        let fixture = doc.projection;
        match action {
            // 👁️ View/config actions — mutate runtime, emit no ops (never pollute undo).
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                self.runtime.selected_node_ids = Self::parse_selection(args);
                ActionEmit::default()
            }
            "nodeGraphHover" => ActionEmit::default(),
            "graphPointerDown" => {
                self.runtime.selected_node_ids.clear();
                ActionEmit::default()
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<CameraJson>(viewport_json) {
                        self.runtime.camera = camera;
                    }
                }
                ActionEmit::default()
            }
            "evaluate" => {
                let mut host = host_from_fixture(fixture, &self.runtime);
                host.clear_computing_widget_ids();
                if let Ok(eval_json) = host.evaluate() {
                    host.apply_eval_outputs_json(&eval_json);
                    self.runtime.last_eval_json = eval_json;
                }
                ActionEmit::default()
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                    if mode == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(mode).is_some() {
                        self.runtime.lod_mode = mode.into();
                    }
                }
                ActionEmit::default()
            }
            "setProximityDistance" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    self.runtime.proximity_distance = value.max(0.0);
                }
                ActionEmit::default()
            }
            "setCatalogueSections" => {
                if let Some(sections) = args.and_then(|value| value.get("sections")) {
                    self.runtime.catalogue_sections_json = sections.to_string();
                }
                ActionEmit::default()
            }
            "toggleExtension" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool());
                if let (Some(id), Some(enabled)) = (id, enabled) {
                    self.runtime.extension_enabled.insert(id.into(), enabled);
                }
                ActionEmit::default()
            }
            "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                let spec = flow_fixture_to_form_spec(fixture);
                let mut generation = self.runtime.generation.clone();
                if handle_generation_action(action, args, &mut generation, &spec, FLOW_PLAY_APP_ID) {
                    self.runtime.generation = generation;
                    if matches!(action, "addGeneration" | "selectGeneration" | "updateGenerationValues") {
                        refresh_generation_preview(fixture, &mut self.runtime);
                    }
                }
                ActionEmit::default()
            }
            // ✏️ Operation actions — run the stateful `FlowHost` mutation, diff into granular ops.
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
                let mut new_id = None;
                let ops = host_ops(fixture, &self.runtime, |host| match host.add_widget(&descriptor, x, y) {
                    Ok(id) => {
                        new_id = Some(id);
                        true
                    }
                    Err(_) => false,
                });
                if let Some(id) = new_id {
                    self.runtime.selected_node_ids = vec![id];
                }
                ActionEmit::ops(ops)
            }
            "removeWidget" => {
                let widget_id = args
                    .and_then(|value| value.get("widgetId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let Some(widget_id) = widget_id else {
                    return ActionEmit::default();
                };
                let ops = host_ops(fixture, &self.runtime, |host| host.remove_widget(&widget_id).is_ok());
                if !ops.is_empty() {
                    self.runtime.selected_node_ids.retain(|id| id != &widget_id);
                }
                ActionEmit::ops(ops)
            }
            "deleteSelection" => {
                let selected = self.runtime.selected_node_ids.clone();
                let ops = host_ops(fixture, &self.runtime, |host| {
                    sync_host_selection(host, &selected);
                    host.delete_selection().is_ok()
                });
                if !ops.is_empty() {
                    self.runtime.selected_node_ids.clear();
                }
                ActionEmit::ops(ops)
            }
            "disconnect" => {
                let synapse_id = args
                    .and_then(|value| value.get("synapseId"))
                    .or_else(|| args.and_then(|value| value.get("edgeId")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                let Some(synapse_id) = synapse_id else {
                    return ActionEmit::default();
                };
                ActionEmit::ops(host_ops(fixture, &self.runtime, |host| host.disconnect(&synapse_id).is_ok()))
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let from_port = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str()).map(str::to_string);
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let to_port = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str()).map(str::to_string);
                let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) else {
                    return ActionEmit::default();
                };
                ActionEmit::ops(host_ops(fixture, &self.runtime, |host| host.connect_ports(&from, &from_port, &to, &to_port).is_ok()))
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) else {
                    return ActionEmit::default();
                };
                let ops = host_ops(fixture, &self.runtime, |host| {
                    host.begin_change();
                    host.move_widget(&node_id, x, y).is_ok()
                });
                if ops.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::amend(ops, format!("move-{node_id}"))
            }
            "reorganize" => ActionEmit::ops(host_ops(fixture, &self.runtime, |host| host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok())),
            "patchFlowWidgets" => {
                let widget_ids: Vec<String> = args
                    .and_then(|value| value.get("widgetIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let raw_value = args.and_then(|value| value.get("value")).cloned();
                let next = Self::patched_widgets_fixture(fixture, &widget_ids, &field, raw_value.as_ref());
                let ops = flow_fixture_ops(fixture, &next);
                if ops.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::amend(ops, format!("patch-{field}-{}", widget_ids.join(",")))
            }
            "renameFlowWidget" => {
                let old_id = args.and_then(|value| value.get("oldId")).and_then(|value| value.as_str());
                let new_id = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                let (Some(old_id), Some(new_id)) = (old_id, new_id) else {
                    return ActionEmit::default();
                };
                let Some(next) = Self::renamed_fixture(fixture, old_id, new_id) else {
                    return ActionEmit::default();
                };
                self.runtime.selected_node_ids = vec![new_id.trim().into()];
                ActionEmit::ops(flow_fixture_ops(fixture, &next))
            }
            "nodeGraphEdit" | "spotlightCommit" => {
                let raw_ops = args.and_then(|value| value.get("ops")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let selected = self.runtime.selected_node_ids.clone();
                let mut clear_selection = false;
                let ops = host_ops(fixture, &self.runtime, |host| {
                    let mut changed = false;
                    for op in &raw_ops {
                        match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                    if let Ok(parsed) = serde_json::from_str::<FlowFixture>(fixture_json) {
                                        host.begin_change();
                                        host.set_fixture_preserving_history(parsed);
                                        changed = true;
                                    }
                                }
                            }
                            "deleteSelection" => {
                                sync_host_selection(host, &selected);
                                if host.delete_selection().is_ok() {
                                    clear_selection = true;
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
                    changed
                });
                if clear_selection {
                    self.runtime.selected_node_ids.clear();
                }
                ActionEmit::ops(ops)
            }
            "runExtensionAction" => {
                let action_id = args.and_then(|value| value.get("actionId")).and_then(|value| value.as_str());
                let Some(action_id) = action_id else {
                    return ActionEmit::default();
                };
                let entry = FLOW_EXTENSIONS.iter().find(|(_, _, entry_action_id, ..)| *entry_action_id == action_id);
                let Some((id, _, _, _, effect)) = entry else {
                    return ActionEmit::default();
                };
                if !self.runtime.extension_enabled.get(*id).copied().unwrap_or(false) {
                    return ActionEmit::default();
                }
                match *effect {
                    "reorganize" => ActionEmit::ops(host_ops(fixture, &self.runtime, |host| host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok())),
                    "evaluate" => {
                        let mut host = host_from_fixture(fixture, &self.runtime);
                        if let Ok(eval_json) = host.evaluate() {
                            self.runtime.last_eval_json = eval_json;
                        }
                        ActionEmit::default()
                    }
                    _ => ActionEmit::default(),
                }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, FlowFixture>, view_state: &ViewState) -> UiNode {
        let fixture = doc.projection;
        let labels = flow_labels(view_state);
        match body_key {
            FLOW_PLAY_BODY_MAIN => render_main_graph(fixture, &self.runtime, labels),
            FLOW_PLAY_BODY_COMPILED => render_compiled_dag(fixture, &self.runtime),
            FLOW_PLAY_BODY_GENERATIONS => render_generate_generations(&self.runtime),
            FLOW_PLAY_BODY_GENERATE_FORM => render_generate_form(fixture, &self.runtime),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&self.runtime),
            FLOW_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &self.runtime.selected_node_ids, labels),
            FLOW_PLAY_BODY_CATALOGUE => build_catalogue_tree(fixture, &self.runtime, labels),
            FLOW_PLAY_BODY_INSPECTOR => build_inspector_tree(fixture, &self.runtime.selected_node_ids, &self.runtime, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = flow_labels(view_state);
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
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
            mode_labels: std::collections::HashMap::from([
                ("edit".to_string(), (if is_de { "Bearbeiten" } else { "Edit" }).to_string()),
                ("generate".to_string(), (if is_de { "Generieren" } else { "Generate" }).to_string()),
            ]),
            action_labels: flow_action_labels(is_de),
            utility_labels: HashMap::new(),
            example_labels: std::collections::HashMap::from([
                ("demo".to_string(), (if is_de { "Demo" } else { "Demo" }).to_string()),
            ]),
            action_arg_labels: HashMap::new(),
            dialog_labels: HashMap::new(),
            introduction_labels: HashMap::new(),
        }
    }
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
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .operation("addWidget", "Add Widget")
            .operation("removeWidget", "Remove Widget")
            .operation("deleteSelection", "Delete Selection")
            .operation("disconnect", "Disconnect")
            .operation("connectMediaPorts", "Connect Ports")
            .operation("moveMediaNode", "Move Node")
            .operation("reorganize", "Reorganize")
            .operation("patchFlowWidgets", "Patch Widgets")
            .operation("renameFlowWidget", "Rename Widget")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .operation("spotlightCommit", "Spotlight Commit")
            // 🧩 Dynamic extension-provided action — id resolved at runtime, kept out of the palette.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("runExtensionAction", "Run Extension Action", ActionKind::Operation) })
            // 👁️ Ephemeral view/config actions — mutate runtime, emit no ops.
            .view_action("evaluate", "Evaluate")
            .view_action("setSelection", "Set Selection")
            .view_action("selectNode", "Select Node")
            .view_action("nodeGraphSelect", "Node Graph Select")
            .view_action("nodeGraphHover", "Node Graph Hover")
            .view_action("graphPointerDown", "Graph Pointer Down")
            .view_action("nodeGraphViewport", "Node Graph Viewport")
            .view_action("setLodMode", "Set LOD Mode")
            .view_action("setProximityDistance", "Set Proximity Distance")
            .view_action("setCatalogueSections", "Set Catalogue Sections")
            .view_action("toggleExtension", "Toggle Extension")
            .view_action("addGeneration", "Add Generation")
            .view_action("removeGeneration", "Remove Generation")
            .view_action("selectGeneration", "Select Generation")
            .view_action("renameGeneration", "Rename Generation")
            .view_action("updateGenerationValues", "Update Generation Values")
            // 📝 Staged argument form for the panel-visible create action (module operators stay catalogue-driven).
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("inputSlider", "Slider"),
                    ActionArgOption::new("inputStepper", "Stepper"),
                    ActionArgOption::new("inputNote", "Note"),
                ]).default_value("inputSlider"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("demo", "Demo", serde_json::to_string(&FlowFixture::default()).unwrap())
    .program("flow", "Flow", "graph")
}

fn register_flow_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "flow", label: "Flow", version: "0.1.0",
    setup: register_flow_exports,
    apps: [ create_flow_app => FlowPlayApp ],
}
//#endregion 🔖Manifest

#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::FlowFixture;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};
    use vcs::MemoryBackbone;

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<FlowPlayApp> {
        VcsDocumentApp::new(FlowPlayApp::default())
    }

    fn render(app: &mut VcsDocumentApp<FlowPlayApp>, body_key: &str, view_state: &ViewState) -> String {
        serde_json::to_string(&app.render(body_key, None, view_state).expect("render")).unwrap()
    }

    #[test]
    fn renders_node_graph_scene() {
        let mut app = new_app();
        assert!(render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default()).contains("node-graph"));
    }

    #[test]
    fn renders_compiled_wire_editor() {
        let mut app = new_app();
        assert!(render(&mut app, FLOW_PLAY_BODY_COMPILED, &ViewState::default()).contains("text-editor"));
    }

    #[test]
    fn default_fixture_has_widgets() {
        assert!(!FlowFixture::default().widgets.is_empty());
    }

    #[test]
    fn document_lists_widgets() {
        let mut app = new_app();
        assert!(render(&mut app, FLOW_PLAY_BODY_DOCUMENT, &ViewState::default()).contains("flow-play-document.widgets"));
    }

    #[test]
    fn catalogue_lists_module_operators() {
        let mut app = new_app();
        let json = render(&mut app, FLOW_PLAY_BODY_CATALOGUE, &ViewState::default());
        assert!(json.contains("flow-play-catalogue.math"), "expected math module section: {json}");
        assert!(json.contains("math.add"), "expected math.add operator: {json}");
    }

    #[test]
    fn catalogue_items_export_flow_widget_drag_payload() {
        let mut app = new_app();
        let json = render(&mut app, FLOW_PLAY_BODY_CATALOGUE, &ViewState::default());
        assert!(json.contains(FLOW_WIDGET_DRAG_MIME), "missing drag mime: {json}");
        assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
    }

    #[test]
    fn add_widget_emits_ops_and_grows_the_document() {
        let mut app = new_app();
        let before = app.projection().expect("projection").widgets.len();
        let result = app
            .handle_action("addWidget", Some(&json!({ "kind": "inputNote", "x": 40.0, "y": 40.0 })), &ViewState::default(), &meta("local"))
            .expect("addWidget");
        assert!(!result.operations.is_empty(), "addWidget must emit ops");
        assert_eq!(app.projection().expect("projection").widgets.len(), before + 1);
    }

    #[test]
    fn undo_restores_fixture_after_add_widget() {
        let mut app = new_app();
        let before = app.projection().expect("projection").widgets.len();
        app.handle_action("addWidget", Some(&json!({ "kind": "inputNote", "x": 40.0, "y": 40.0 })), &ViewState::default(), &meta("local")).expect("addWidget");
        assert_eq!(app.projection().expect("projection").widgets.len(), before + 1);
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").widgets.len(), before);
    }

    #[test]
    fn selection_is_view_state_and_emits_no_ops() {
        let mut app = new_app();
        let result = app
            .handle_action("setSelection", Some(&json!({ "ids": ["slider"] })), &ViewState::default(), &meta("local"))
            .expect("setSelection");
        assert!(result.operations.is_empty(), "selection must not produce document ops");
    }

    #[test]
    fn evaluate_updates_preview_state_without_ops() {
        let mut app = new_app();
        let result = app.handle_action("evaluate", None, &ViewState::default(), &meta("local")).expect("evaluate");
        assert!(result.operations.is_empty(), "evaluate is a view action");
    }

    #[test]
    fn generate_mode_renders_three_surfaces() {
        let mut app = new_app();
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATIONS, &ViewState::default()).contains("addGeneration"));
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_FORM, &ViewState::default()).contains("Add a generation"));
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW, &ViewState::default()).contains("text-editor"));
    }

    #[test]
    fn set_lod_mode_rejects_unknown_and_accepts_known() {
        let mut app = new_app();
        app.handle_action("setLodMode", Some(&json!({ "mode": "bogus" })), &ViewState::default(), &meta("local")).expect("bogus");
        app.handle_action("setLodMode", Some(&json!({ "mode": "micro" })), &ViewState::default(), &meta("local")).expect("micro");
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert!(json.contains("\\\"forcedLabel\\\":\\\"micro\\\"") || json.contains("\"forcedLabel\":\"micro\""));
    }

    #[test]
    fn toggle_extension_and_run_action_reorganizes_fixture() {
        let mut app = new_app();
        let before = app.projection().expect("projection").widgets.len();
        let ignored = app.handle_action("runExtensionAction", Some(&json!({ "actionId": "flow.extension.reorganize" })), &ViewState::default(), &meta("local")).expect("ignored");
        assert!(ignored.operations.is_empty(), "disabled extension action must be a no-op");
        app.handle_action("toggleExtension", Some(&json!({ "id": "auto-layout", "enabled": true })), &ViewState::default(), &meta("local")).expect("toggle");
        app.handle_action("runExtensionAction", Some(&json!({ "actionId": "flow.extension.reorganize" })), &ViewState::default(), &meta("local")).expect("reorganize");
        assert_eq!(app.projection().expect("projection").widgets.len(), before, "reorganize keeps every widget");
    }

    #[test]
    fn flow_labels_resolve_native_english_and_german() {
        let mut app = new_app();
        let english = render(&mut app, FLOW_PLAY_BODY_DOCUMENT, &ViewState::default());
        assert!(english.contains("Widgets") && english.contains("Synapses"), "english labels: {english}");
        let german = render(&mut app, FLOW_PLAY_BODY_DOCUMENT, &ViewState { locale: Some("de".into()), ..ViewState::default() });
        assert!(german.contains("Synapsen"), "german labels: {german}");
    }

    /// 🤝 Definitional merge proof: two instances on one backbone make DISJOINT edits (one renames a
    /// widget, the other adds a widget); after exchanging ops both converge — impossible under
    /// whole-fixture `setDocument` snapshots, which would clobber one side.
    #[test]
    fn two_instances_converge_on_disjoint_edits() {
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://flow-convergence", "mem://flow-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a
            .handle_action("renameFlowWidget", Some(&json!({ "oldId": "slider", "value": "input" })), &ViewState::default(), &meta("actor-a"))
            .expect("a renames slider");
        instance_b
            .handle_action("addWidget", Some(&json!({ "kind": "inputNote", "x": 10.0, "y": 10.0 })), &ViewState::default(), &meta("actor-b"))
            .expect("b adds a note");

        // A neutral history action always dispatches through the store, which pumps inbound ops first.
        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        assert!(projection_a.widgets.iter().any(|widget| widget_id(widget) == "input"), "A keeps its rename");
        assert!(projection_a.widgets.iter().any(|widget| matches!(widget, Widget::InputNote { .. })), "A absorbs B's note");
        assert_eq!(projection_a.widgets.len(), projection_b.widgets.len(), "both instances converge to the same widget set");
    }
}
