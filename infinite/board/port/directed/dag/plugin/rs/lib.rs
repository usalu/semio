//! 🔀 DAG plugin — declarative DAG play app bundled as a hot-swappable WASM component.

use infinite_board_port_directed_dag::{
    dag_fixture_from_document, dag_fixture_to_wire_literal, dag_node_kind_tag, default_dag_document, fit_node_size, note_widget_size, preview_widget_size, stepper_widget_height, stepper_widget_width, would_create_cycle, DagCamera, DagDocument,
    DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodePatch, DagNodeSpec, DagOp, DagPreviewContent, DagStepperField, IoPortSpec, DAG_DOCUMENT_SCHEMA,
};
use semio_framework_plugin::{
    build_node_graph_scene, build_text_editor_scene, create_default_layout, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef,
    ActionArgOption, ActionDescriptor, ActionEmit, App, DocumentApp, DocumentView, NodeGraphScene, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, ResourceKindSpec, SurfaceKind, TextEditorScene, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap};
use vcs::CollectionOp;

//#region ⚠️ Errors
/// ⚠️ Errors from DAG play app edge-connection building.
#[derive(Debug, thiserror::Error)]
enum DagPlayError {
    #[error("connection would create cycle")]
    CycleDetected,
}
//#endregion ⚠️ Errors

//#region 🔖Constants
const DAG_PLAY_APP_ID: &str = "dag-play";
const DAG_PLAY_SURFACE_MAIN: &str = "dag.play.main";
const DAG_PLAY_SURFACE_COMPILED: &str = "dag.play.compiled-dag";
const DAG_PLAY_BODY_MAIN: &str = "dag.play.main";
const DAG_PLAY_BODY_COMPILED: &str = "dag.play.compiled-dag";
const DAG_PLAY_BODY_DOCUMENT: &str = "dag.play.document";
const DAG_PLAY_BODY_CATALOGUE: &str = "dag.play.catalogue";
const DAG_PLAY_BODY_INSPECTOR: &str = "dag.play.inspection";
const DAG_PLAY_WINDOW_MAIN: &str = "dag-main";
const DAG_PLAY_WINDOW_COMPILED: &str = "dag-compiled-dag";
//#endregion 🔖Constants

//#region 🔖Runtime
/// 🎛️ Ephemeral view state — selection and camera/viewport — lives in the app struct, not the
/// document, so panning/zooming and selecting never pollute undo history.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct DagPlayRuntime {
    selected_node_ids: Vec<String>,
    camera: DagCamera,
}

impl Default for DagPlayRuntime {
    fn default() -> Self {
        Self { selected_node_ids: Vec::new(), camera: DagFixture::default().camera }
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
//#endregion 🔖Runtime

//#region 🔖DocumentHelpers
fn dag_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: DAG_PLAY_APP_ID.into(), action: action.into(), args }
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once(':').map(|(node, port)| (node.to_string(), port.to_string())).unwrap_or_else(|| (endpoint.to_string(), "out".into()))
}

fn document_to_media_graph(document: &DagDocument) -> (String, String) {
    let nodes: Vec<MediaGraphNodeRecord> = document
        .nodes
        .iter()
        .map(|node| MediaGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| MediaGraphPortRecord { id: format!("{}:{}", node.id, port.id), label: Some(port.label.clone()) }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| MediaGraphPortRecord { id: format!("{}:{}", node.id, port.id), label: Some(port.label.clone()) }).collect(),
        })
        .collect();
    let edges: Vec<MediaGraphEdgeRecord> = document
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            MediaGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id }
        })
        .collect();
    (serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()))
}

fn next_node_id(document: &DagDocument) -> String {
    let max = document.nodes.iter().filter_map(|node| node.id.strip_prefix('n').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0);
    format!("n{}", max + 1)
}

fn default_node_for_kind(kind: &str, id: &str, x: f64, y: f64) -> DagNodeSpec {
    let mut node = match kind {
        "slider" => DagNodeSpec {
            id: id.into(),
            name: "Slider".into(),
            abbreviation: "Sld".into(),
            icon: "emoji:🎚️".into(),
            x,
            y,
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.1, value: 3.0, output: IoPortSpec::named("N", "Num", "number", "Number") },
            ..Default::default()
        },
        "select" => DagNodeSpec {
            id: id.into(),
            name: "Select".into(),
            abbreviation: "Sel".into(),
            icon: "emoji:📋".into(),
            x,
            y,
            kind: DagNodeKind::Select { options: vec!["A".into(), "B".into(), "C".into()], selected: 0, output: IoPortSpec::named("V", "Val", "value", "Value") },
            ..Default::default()
        },
        "screen" => {
            DagNodeSpec { id: id.into(), name: "Screen".into(), abbreviation: "Scr".into(), icon: "emoji:🖥️".into(), x, y, kind: DagNodeKind::Screen { media: None, input: IoPortSpec::named("I", "In", "in", "Input") }, ..Default::default() }
        }
        "note" => {
            let text = String::new();
            let (width, height) = note_widget_size(&text);
            DagNodeSpec { id: id.into(), name: "Note".into(), abbreviation: "Note".into(), icon: "emoji:📝".into(), x, y, width, height, kind: DagNodeKind::Note { text, output: IoPortSpec::named("T", "Txt", "text", "Text") }, ..Default::default() }
        }
        "preview" => {
            let (width, height) = preview_widget_size(&DagPreviewContent::Scalar { text: String::new() }, &BTreeSet::new());
            DagNodeSpec {
                id: id.into(),
                name: "Preview".into(),
                abbreviation: "Prv".into(),
                icon: "emoji:👁️".into(),
                x,
                y,
                width,
                height,
                kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: String::new() }, expanded: BTreeSet::new(), input: IoPortSpec::named("I", "In", "in", "Input") },
                ..Default::default()
            }
        }
        "stepper" => {
            let fields = vec![DagStepperField { key: "value".into(), label: "Value".into(), value: 0.0, step: 1.0 }];
            DagNodeSpec {
                id: id.into(),
                name: "Stepper".into(),
                abbreviation: "Stp".into(),
                icon: "emoji:🎚️".into(),
                x,
                y,
                width: stepper_widget_width(),
                height: stepper_widget_height(fields.len()),
                kind: DagNodeKind::Stepper { fields, output: IoPortSpec::named("N", "Num", "number", "Number") },
                ..Default::default()
            }
        }
        _ => DagNodeSpec {
            id: id.into(),
            name: "Computation".into(),
            abbreviation: "Cmp".into(),
            icon: "emoji:⚙️".into(),
            x,
            y,
            operator_kind: Some("math.add".into()),
            kind: DagNodeKind::Computation {
                inputs: vec![IoPortSpec::named("A", "A", "a", "A"), IoPortSpec::named("B", "B", "b", "B")],
                outputs: vec![IoPortSpec::named("R", "R", "result", "Result")],
                variadic_inputs: false,
                variadic_outputs: false,
            },
            ..Default::default()
        },
    };
    fit_node_size(&mut node);
    node
}

/// 🔗 Builds the `DagFixtureEdge` connecting two ports, or `Err` if it would introduce a cycle.
fn connect_edge(document: &DagDocument, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<DagFixtureEdge, DagPlayError> {
    let existing: Vec<(String, String)> = document
        .edges
        .iter()
        .map(|edge| {
            let (from, _) = split_endpoint(&edge.source);
            let (to, _) = split_endpoint(&edge.target);
            (from, to)
        })
        .collect();
    if would_create_cycle(&existing, source_node_id, target_node_id) {
        return Err(DagPlayError::CycleDetected);
    }
    let edge_id = format!("e{}", document.edges.iter().filter_map(|edge| edge.id.strip_prefix('e').and_then(|suffix| suffix.parse::<u64>().ok())).max().unwrap_or(0) + 1);
    Ok(DagFixtureEdge { id: edge_id, source: format!("{source_node_id}:{source_port_id}"), target: format!("{target_node_id}:{target_port_id}"), ..Default::default() })
}

/// 🗑️ Ops removing `node_ids` and every edge touching them, for delete-node / delete-selection.
fn remove_nodes_ops(document: &DagDocument, node_ids: &[String]) -> Vec<DagOp> {
    let mut ops: Vec<DagOp> = document.nodes.iter().filter(|node| node_ids.contains(&node.id)).map(|node| DagOp::Nodes(CollectionOp::Remove { id: node.id.clone() })).collect();
    ops.extend(
        document
            .edges
            .iter()
            .filter(|edge| {
                let (from, _) = split_endpoint(&edge.source);
                let (to, _) = split_endpoint(&edge.target);
                node_ids.iter().any(|id| id == &from || id == &to)
            })
            .map(|edge| DagOp::Edges(CollectionOp::Remove { id: edge.id.clone() })),
    );
    ops
}

/// 🩹 Builds the `DagNodePatch` for a `patchDagNodes` field write (name, or a slider param that also
/// refits the widget size).
fn node_patch_for_field(node: &DagNodeSpec, field: &str, raw_value: Option<&Value>) -> Option<DagNodePatch> {
    match field {
        "name" => raw_value.and_then(|value| value.as_str()).map(|value| DagNodePatch { name: Some(value.into()), ..Default::default() }),
        "value" | "min" | "max" if matches!(node.kind, DagNodeKind::Slider { .. }) => {
            let value = raw_value.and_then(|value| value.as_f64())?;
            let mut updated = node.clone();
            if let DagNodeKind::Slider { value: ref mut slider_value, min: ref mut slider_min, max: ref mut slider_max, .. } = updated.kind {
                match field {
                    "value" => *slider_value = value,
                    "min" => *slider_min = value,
                    _ => *slider_max = value,
                }
            }
            fit_node_size(&mut updated);
            Some(DagNodePatch { kind: Some(updated.kind.clone()), width: Some(updated.width), height: Some(updated.height), ..Default::default() })
        }
        _ => None,
    }
}

fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        loading: None,
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

fn tree_item_with_description(id: impl Into<String>, label: impl Into<String>, description: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: Some(description.into()),
        icon_id: None,
        selected: None,
        loading: None,
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

fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        selected: None,
        loading: None,
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
/// 🗣️ Complete UI label set for the DAG app; one field per label makes every locale combination compile-checked.
struct DagPlayLabels {
    nodes: &'static str,
    edges: &'static str,
    empty: &'static str,
    kind_computation: &'static str,
    kind_slider: &'static str,
    kind_stepper: &'static str,
    kind_select: &'static str,
    kind_note: &'static str,
    kind_preview: &'static str,
    kind_screen: &'static str,
    select_a_node: &'static str,
    node_not_found: &'static str,
    slider_group: &'static str,
    node_group: &'static str,
    field_value: &'static str,
    field_min: &'static str,
    field_max: &'static str,
    field_name: &'static str,
    field_kind: &'static str,
    field_id: &'static str,
    selected_suffix: &'static str,
    delete_selection: &'static str,
    window_main: &'static str,
    window_compiled: &'static str,
}

const DAG_PLAY_LABELS_NATIVE_EN: DagPlayLabels = DagPlayLabels {
    nodes: "Nodes",
    edges: "Edges",
    empty: "(none)",
    kind_computation: "Computation",
    kind_slider: "Slider",
    kind_stepper: "Stepper",
    kind_select: "Select",
    kind_note: "Note",
    kind_preview: "Preview",
    kind_screen: "Screen",
    select_a_node: "Select a node in the document.",
    node_not_found: "Node not found",
    slider_group: "slider",
    node_group: "Node",
    field_value: "Value",
    field_min: "Min",
    field_max: "Max",
    field_name: "Name",
    field_kind: "Kind",
    field_id: "Id",
    selected_suffix: "selected",
    delete_selection: "Delete selection",
    window_main: "DAG",
    window_compiled: "DSL",
};

const DAG_PLAY_LABELS_NATIVE_DE: DagPlayLabels = DagPlayLabels {
    nodes: "Knoten",
    edges: "Kanten",
    empty: "(keine)",
    kind_computation: "Berechnung",
    kind_slider: "Schieberegler",
    kind_stepper: "Schrittregler",
    kind_select: "Auswahl",
    kind_note: "Notiz",
    kind_preview: "Vorschau",
    kind_screen: "Bildschirm",
    select_a_node: "Wählen Sie einen Knoten im Dokument aus.",
    node_not_found: "Knoten nicht gefunden",
    slider_group: "schieberegler",
    node_group: "Knoten",
    field_value: "Wert",
    field_min: "Min",
    field_max: "Max",
    field_name: "Name",
    field_kind: "Typ",
    field_id: "Id",
    selected_suffix: "ausgewählt",
    delete_selection: "Auswahl löschen",
    window_main: "DAG",
    window_compiled: "DSL",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; this app has no terminology variant.
fn dag_play_labels(view_state: &ViewState) -> &'static DagPlayLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &DAG_PLAY_LABELS_NATIVE_DE
    } else {
        &DAG_PLAY_LABELS_NATIVE_EN
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_dag_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn dag_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("addNode", "Add Node", "Knoten hinzufügen"),
        ("removeNode", "Remove Node", "Knoten entfernen"),
        ("deleteSelection", "Delete Selection", "Auswahl löschen"),
        ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
        ("connectMediaPorts", "Connect Ports", "Anschlüsse verbinden"),
        ("disconnect", "Disconnect", "Trennen"),
        ("moveMediaNode", "Move Node", "Knoten verschieben"),
        ("renameDagNode", "Rename Node", "Knoten umbenennen"),
        ("reorganize", "Reorganize", "Neu anordnen"),
        ("patchDagNodes", "Patch Nodes", "Knoten aktualisieren"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("selectNode", "Select Node", "Knoten auswählen"),
        ("nodeGraphSelect", "Node Graph Select", "Knotengraph auswählen"),
        ("nodeGraphHover", "Node Graph Hover", "Knotengraph-Hover"),
        ("nodeGraphViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
        ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}
//#endregion 🔖CommandLabels

//#region 🔖Panels
fn build_document_tree(document: &DagDocument, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    let node_items: Vec<UiTreeItemNode> = document
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(
                format!("dag-play-document.node.{}", node.id),
                if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
                Some(dag_node_kind_tag(&node.kind).into()),
                dag_action("setSelection", Some(json!({ "ids": [node.id.clone()] }))),
            )
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = document.edges.iter().map(|edge| tree_item_with_description(format!("dag-play-document.edge.{}", edge.id), format!("{} → {}", edge.source, edge.target), edge.id.clone())).collect();
    UiNode::Tree(UiTreeNode {
        loading: None,
        sections: vec![
            UiTreeSectionNode {
                id: "dag-play-document.nodes".into(),
                label: Some(labels.nodes.into()),
                default_open: Some(true),
                loading: None,
                items: if node_items.is_empty() { vec![tree_item("dag-play-document.nodes.empty", labels.empty)] } else { node_items },
            },
            UiTreeSectionNode {
                id: "dag-play-document.edges".into(),
                label: Some(labels.edges.into()),
                default_open: Some(false),
                loading: None,
                items: if edge_items.is_empty() { vec![tree_item("dag-play-document.edges.empty", labels.empty)] } else { edge_items },
            },
        ],
        selected_ids: Some(selected.iter().map(|id| format!("dag-play-document.node.{id}")).collect()),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn build_catalogue_tree(labels: &DagPlayLabels) -> UiNode {
    let kinds =
        [("computation", labels.kind_computation), ("slider", labels.kind_slider), ("stepper", labels.kind_stepper), ("select", labels.kind_select), ("note", labels.kind_note), ("preview", labels.kind_preview), ("screen", labels.kind_screen)];
    UiNode::Tree(UiTreeNode {
        loading: None,
        sections: vec![UiTreeSectionNode {
            id: "dag-play-catalogue.node-kinds".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            loading: None,
            items: kinds.iter().map(|(kind, label)| tree_item_with_action(format!("dag-play-catalogue.kind.{kind}"), *label, Some((*kind).into()), dag_action("addNode", Some(json!({ "kind": kind }))))).collect(),
        }],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn inspector_number_field(node_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
            commit: None,
            on_change: dag_action("patchDagNodes", Some(json!({ "nodeIds": node_ids, "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn inspector_text_field(node_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder,
            commit: Some("blur".into()),
            on_change: dag_action("patchDagNodes", Some(json!({ "nodeIds": node_ids, "field": field }))),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn build_inspector_tree(document: &DagDocument, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "dag-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            loading: None,
            children: vec![ui_text(labels.select_a_node)],
        }]);
    }
    let nodes: Vec<&DagNodeSpec> = selected.iter().filter_map(|id| document.nodes.iter().find(|node| &node.id == id)).collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "dag-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            loading: None,
            children: vec![ui_text(labels.node_not_found)],
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if nodes.iter().all(|node| matches!(node.kind, DagNodeKind::Slider { .. })) {
        groups.push(UiInspectorFieldGroup {
            id: "dag-play-inspector.kind.slider".into(),
            label: labels.slider_group.into(),
            default_open: None,
            fields: vec![
                inspector_number_field(
                    &node_ids,
                    "dag-play-inspector.slider-value",
                    labels.field_value,
                    &nodes
                        .iter()
                        .map(|node| match &node.kind {
                            DagNodeKind::Slider { value, .. } => *value,
                            _ => 0.0,
                        })
                        .collect::<Vec<_>>(),
                    "value",
                ),
                inspector_number_field(
                    &node_ids,
                    "dag-play-inspector.slider-min",
                    labels.field_min,
                    &nodes
                        .iter()
                        .map(|node| match &node.kind {
                            DagNodeKind::Slider { min, .. } => *min,
                            _ => 0.0,
                        })
                        .collect::<Vec<_>>(),
                    "min",
                ),
                inspector_number_field(
                    &node_ids,
                    "dag-play-inspector.slider-max",
                    labels.field_max,
                    &nodes
                        .iter()
                        .map(|node| match &node.kind {
                            DagNodeKind::Slider { max, .. } => *max,
                            _ => 0.0,
                        })
                        .collect::<Vec<_>>(),
                    "max",
                ),
            ],
        });
    }
    let mut base_fields = vec![
        inspector_text_field(&node_ids, "dag-play-inspector.name", labels.field_name, &nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>(), "name"),
        ui_inspector_readonly_field(
            "dag-play-inspector.kind",
            labels.field_kind,
            if nodes.iter().map(|node| dag_node_kind_tag(&node.kind)).collect::<std::collections::HashSet<_>>().len() == 1 { dag_node_kind_tag(&nodes[0].kind).to_string() } else { "—".into() },
        ),
    ];
    if node_ids.len() == 1 {
        base_fields.insert(
            0,
            UiNode::Field(UiFieldNode {
                id: "dag-play-inspector.id".into(),
                label: labels.field_id.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "dag-play-inspector.id.input".into(),
                    input_kind: "text".into(),
                    value: node_ids[0].clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: dag_action("renameDagNode", Some(json!({ "oldId": node_ids[0] }))),
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
    } else {
        base_fields.insert(0, ui_inspector_readonly_field("dag-play-inspector.id", labels.field_id, format!("{} {}", node_ids.len(), labels.selected_suffix)));
    }
    groups.push(UiInspectorFieldGroup { id: "dag-play-inspector.base".into(), label: labels.node_group.into(), default_open: None, fields: base_fields });
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_main_graph(document: &DagDocument, camera: &DagCamera, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    let (nodes_json, edges_json) = document_to_media_graph(document);
    let viewport_json = serde_json::to_string(camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let selection_json = if selected.is_empty() { None } else { serde_json::to_string(selected).ok() };
    let context_menu_json = json!([{
        "id": "delete-selection",
        "label": labels.delete_selection,
        "action": "nodeGraphEdit",
        "args": { "ops": [{ "op": "deleteSelection" }] },
    }])
    .to_string();
    build_node_graph_scene(DAG_PLAY_SURFACE_MAIN, DAG_PLAY_APP_ID, NodeGraphScene { editable: Some(true), selection_json, context_menu_json: Some(context_menu_json), ..NodeGraphScene::base(nodes_json, edges_json, viewport_json) })
}

fn render_compiled_dag(document: &DagDocument, camera: &DagCamera) -> UiNode {
    let fixture = dag_fixture_from_document(document, camera.clone());
    build_text_editor_scene(DAG_PLAY_SURFACE_COMPILED, DAG_PLAY_APP_ID, TextEditorScene::base(dag_fixture_to_wire_literal(&fixture), Some("wire".into()), None))
}
//#endregion 🔖Render

//#region 🔖DagPlayApp
#[derive(Default)]
struct DagPlayApp {
    runtime: DagPlayRuntime,
}

impl DagPlayApp {
    /// 👁️ Parses the many selection-arg shapes (`ids`/`nodeIds` arrays or a single `nodeId`) into ids.
    fn parse_selection(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("nodeIds").or_else(|| value.get("ids")))
            .and_then(|value| if value.is_array() { serde_json::from_value(value.clone()).ok() } else { value.as_str().map(|id| vec![id.to_string()]) })
            .or_else(|| args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()]))
            .unwrap_or_default()
    }
}

impl DocumentApp for DagPlayApp {
    type Projection = DagDocument;
    type Op = DagOp;

    fn app_id(&self) -> &str {
        DAG_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        DAG_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> DagDocument {
        default_dag_document()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, DagDocument>, _view_state: &ViewState) -> ActionEmit<DagOp> {
        let document = doc.projection;
        match action {
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
                    if let Ok(camera) = serde_json::from_str::<DagCamera>(viewport_json) {
                        self.runtime.camera = camera;
                    }
                }
                ActionEmit::default()
            }
            "nodeGraphEdit" => {
                let ops = args.and_then(|value| value.get("ops")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let mut emitted: Vec<DagOp> = Vec::new();
                for op in ops {
                    match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
                        "setFixture" => {
                            if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                                if let Ok(fixture) = serde_json::from_str::<DagFixture>(fixture_json) {
                                    self.runtime.camera = fixture.camera.clone();
                                    emitted.push(DagOp::SetDocument { document: infinite_board_port_directed_dag::dag_document_from_fixture(&fixture) });
                                }
                            }
                        }
                        "deleteSelection" => {
                            let ids = self.runtime.selected_node_ids.clone();
                            let removes = remove_nodes_ops(document, &ids);
                            if !removes.is_empty() {
                                self.runtime.selected_node_ids.clear();
                                emitted.extend(removes);
                            }
                        }
                        "connect" => {
                            let from = op.get("sourceNodeId").and_then(|value| value.as_str());
                            let from_port = op.get("sourcePortId").and_then(|value| value.as_str());
                            let to = op.get("targetNodeId").and_then(|value| value.as_str());
                            let to_port = op.get("targetPortId").and_then(|value| value.as_str());
                            if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                if let Ok(edge) = connect_edge(document, from, from_port, to, to_port) {
                                    emitted.push(DagOp::Edges(CollectionOp::Add { index: document.edges.len(), item: edge }));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                ActionEmit::ops(emitted)
            }
            "deleteSelection" => {
                let ids = self.runtime.selected_node_ids.clone();
                let removes = remove_nodes_ops(document, &ids);
                if removes.is_empty() {
                    return ActionEmit::default();
                }
                self.runtime.selected_node_ids.clear();
                ActionEmit::ops(removes)
            }
            "renameDagNode" => {
                let old_id = args.and_then(|value| value.get("oldId")).and_then(|value| value.as_str());
                let new_id = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                if let (Some(old_id), Some(new_id)) = (old_id, new_id) {
                    let trimmed = new_id.trim();
                    if !trimmed.is_empty() && trimmed != old_id && !document.nodes.iter().any(|node| node.id == trimmed) {
                        let nodes: Vec<DagNodeSpec> = document.nodes.iter().map(|node| if node.id == old_id { DagNodeSpec { id: trimmed.into(), ..node.clone() } } else { node.clone() }).collect();
                        let edges: Vec<DagFixtureEdge> = document
                            .edges
                            .iter()
                            .map(|edge| {
                                let (from_node, from_port) = split_endpoint(&edge.source);
                                let (to_node, to_port) = split_endpoint(&edge.target);
                                DagFixtureEdge {
                                    source: if from_node == old_id { format!("{trimmed}:{from_port}") } else { edge.source.clone() },
                                    target: if to_node == old_id { format!("{trimmed}:{to_port}") } else { edge.target.clone() },
                                    ..edge.clone()
                                }
                            })
                            .collect();
                        self.runtime.selected_node_ids = vec![trimmed.into()];
                        return ActionEmit::ops(vec![DagOp::SetNodes { nodes }, DagOp::SetEdges { edges }]);
                    }
                }
                ActionEmit::default()
            }
            "removeNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).or_else(|| args.and_then(|value| value.get("id"))).and_then(|value| value.as_str());
                if let Some(node_id) = node_id {
                    let removes = remove_nodes_ops(document, &[node_id.to_string()]);
                    if !removes.is_empty() {
                        self.runtime.selected_node_ids.retain(|id| id != node_id);
                        return ActionEmit::ops(removes);
                    }
                }
                ActionEmit::default()
            }
            "disconnect" => {
                let edge_id = args.and_then(|value| value.get("edgeId")).or_else(|| args.and_then(|value| value.get("synapseId"))).and_then(|value| value.as_str());
                match edge_id {
                    Some(edge_id) if document.edges.iter().any(|edge| edge.id == edge_id) => ActionEmit::ops(vec![DagOp::Edges(CollectionOp::Remove { id: edge_id.into() })]),
                    _ => ActionEmit::default(),
                }
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if document.nodes.iter().any(|node| node.id == node_id) {
                        return ActionEmit::amend(vec![DagOp::Nodes(CollectionOp::Patch { id: node_id.into(), patch: DagNodePatch { x: Some(x), y: Some(y), ..Default::default() } })], format!("move-{node_id}"));
                    }
                }
                ActionEmit::default()
            }
            "connectMediaPorts" => {
                let source_node_id = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str());
                let source_port_id = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str());
                let target_node_id = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str());
                let target_port_id = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str());
                if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (source_node_id, source_port_id, target_node_id, target_port_id) {
                    if let Ok(edge) = connect_edge(document, from, from_port, to, to_port) {
                        return ActionEmit::ops(vec![DagOp::Edges(CollectionOp::Add { index: document.edges.len(), item: edge })]);
                    }
                }
                ActionEmit::default()
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("computation");
                let id = next_node_id(document);
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let node = default_node_for_kind(kind, &id, x, y);
                self.runtime.selected_node_ids = vec![id];
                ActionEmit::ops(vec![DagOp::Nodes(CollectionOp::Add { index: document.nodes.len(), item: node })])
            }
            "reorganize" => {
                if let Ok(mut host) = DagHost::load_fixture_json(&serde_json::to_string(&dag_fixture_from_document(document, self.runtime.camera.clone())).unwrap_or_default()) {
                    let _ = host.reorganize(&DagLayoutOptions::default());
                    if let Ok(json) = host.fixture_json() {
                        if let Ok(fixture) = serde_json::from_str::<DagFixture>(&json) {
                            return ActionEmit::ops(vec![DagOp::SetNodes { nodes: fixture.nodes }]);
                        }
                    }
                }
                ActionEmit::default()
            }
            "patchDagNodes" => {
                let node_ids: Vec<String> = args.and_then(|value| value.get("nodeIds")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value"));
                let ops: Vec<DagOp> =
                    document.nodes.iter().filter(|node| node_ids.contains(&node.id)).filter_map(|node| node_patch_for_field(node, field, raw_value).map(|patch| DagOp::Nodes(CollectionOp::Patch { id: node.id.clone(), patch }))).collect();
                if ops.is_empty() {
                    ActionEmit::default()
                } else {
                    ActionEmit::amend(ops, format!("patch-{field}-{}", node_ids.join(",")))
                }
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, DagDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let selected = &self.runtime.selected_node_ids;
        let camera = &self.runtime.camera;
        let labels = dag_play_labels(view_state);
        match body_key {
            DAG_PLAY_BODY_MAIN => render_main_graph(document, camera, selected, labels),
            DAG_PLAY_BODY_COMPILED => render_compiled_dag(document, camera),
            DAG_PLAY_BODY_DOCUMENT => build_document_tree(document, selected, labels),
            DAG_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            DAG_PLAY_BODY_INSPECTOR => build_inspector_tree(document, selected, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = dag_play_labels(view_state);
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        semio_framework_plugin::AppLabelsOverlay {
            window_kind_labels: std::collections::HashMap::from([(DAG_PLAY_WINDOW_MAIN.to_string(), labels.window_main.to_string()), (DAG_PLAY_WINDOW_COMPILED.to_string(), labels.window_compiled.to_string())]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::from([("edit".to_string(), (if is_de { "Bearbeiten" } else { "Edit" }).to_string())]),
            action_labels: dag_action_labels(is_de),
            utility_labels: HashMap::new(),
            example_labels: std::collections::HashMap::from([("demo".to_string(), "Demo".to_string())]),
            action_arg_labels: HashMap::new(),
            dialog_labels: HashMap::new(),
            introduction_labels: HashMap::new(),
            group_labels: HashMap::new(),
        }
    }
}
//#endregion 🔖DagPlayApp

//#region 🔖Manifest
fn create_dag_app() -> App {
    App::from_builder(
        App::builder(DAG_PLAY_APP_ID, "DAG").document(["semio", "mathematical", "graph", "port", "directed", "dag"])
            .resource_kind(ResourceKindSpec {
                id: "graph.dag".into(),
                name: "DAG".into(),
                source_format: "flow.dag".into(),
                component_kind: "dag".into(),
                dimension: "graph".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Dag },
                schema: "flow.dag".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("dag")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(DAG_PLAY_WINDOW_MAIN, "DAG", DAG_PLAY_BODY_MAIN, SurfaceKind::NodeGraph)
            .window_kind(DAG_PLAY_WINDOW_COMPILED, "DSL", DAG_PLAY_BODY_COMPILED, SurfaceKind::NodeGraph)
            .default_layout(create_default_layout(
                &[DAG_PLAY_WINDOW_MAIN.into(), DAG_PLAY_WINDOW_COMPILED.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["DAG".into(), "DSL".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                DAG_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                DAG_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                DAG_PLAY_BODY_INSPECTOR,
            )
            // ✏️ Document-mutating: dispatched as VCS operations with a true inverse.
            .operation("addNode", "Add Node")
            .operation("removeNode", "Remove Node")
            .operation("deleteSelection", "Delete Selection")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .operation("connectMediaPorts", "Connect Ports")
            .operation("disconnect", "Disconnect")
            .operation("moveMediaNode", "Move Node")
            .operation("renameDagNode", "Rename Node")
            .operation("reorganize", "Reorganize")
            .operation("patchDagNodes", "Patch Nodes")
            // 👁️ Ephemeral view state — selection and camera/viewport.
            .view_action("setSelection", "Set Selection")
            .view_action("selectNode", "Select Node")
            .view_action("nodeGraphSelect", "Node Graph Select")
            .view_action("nodeGraphHover", "Node Graph Hover")
            .view_action("nodeGraphViewport", "Node Graph Viewport")
            .view_action("graphPointerDown", "Graph Pointer Down")
            // 📝 Staged argument form for the panel-visible create action.
            .action_args("addNode", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("computation", "Computation"),
                    ActionArgOption::new("slider", "Slider"),
                    ActionArgOption::new("select", "Select"),
                    ActionArgOption::new("screen", "Screen"),
                    ActionArgOption::new("note", "Note"),
                    ActionArgOption::new("preview", "Preview"),
                    ActionArgOption::new("stepper", "Stepper"),
                ]).default_value("computation"),
            ]),
    )
    .example("demo", "Demo", serde_json::to_string(&default_dag_document()).expect("default DAG document has no non-string map keys or non-finite floats, so JSON serialization is infallible"))
    .program("dag", "DAG", "graph")
}

fn register_dag_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "dag", label: "DAG", version: "0.1.0",
    setup: register_dag_exports,
    apps: [ create_dag_app => DagPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};

    fn meta(actor: &str) -> ActionMeta {
        ActionMeta { actor: actor.into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<DagPlayApp> {
        VcsDocumentApp::new(DagPlayApp::default())
    }

    #[test]
    fn dag_play_labels_resolve_native_by_default() {
        let mut app = new_app();
        let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Nodes"));
        assert!(json.contains("Edges"));
    }

    #[test]
    fn dag_play_labels_resolve_native_in_german() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &view_state).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Knoten"));
        assert!(json.contains("Kanten"));
    }

    #[test]
    fn renders_node_graph_scene() {
        let mut app = new_app();
        let node = app.render(DAG_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_compiled_dag_text_editor() {
        let mut app = new_app();
        let node = app.render(DAG_PLAY_BODY_COMPILED, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
    }

    #[test]
    fn add_node_action_updates_document() {
        let mut app = new_app();
        app.handle_action("addNode", Some(&json!({ "kind": "slider" })), &ViewState::default(), &meta("local")).expect("add node");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Slider { .. })));
    }

    #[test]
    fn rename_dag_node_rewrites_nodes_and_edges() {
        let mut app = new_app();
        let old_id = app.projection().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");
        app.handle_action("renameDagNode", Some(&json!({ "oldId": old_id, "value": "renamed-node" })), &ViewState::default(), &meta("local")).expect("rename");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().any(|node| node.id == "renamed-node"));
        assert!(document.nodes.iter().all(|node| node.id != old_id));
    }

    #[test]
    fn remove_node_deletes_node_and_connected_edges() {
        let mut app = new_app();
        let node_id = app.projection().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");
        app.handle_action("removeNode", Some(&json!({ "nodeId": node_id })), &ViewState::default(), &meta("local")).expect("remove");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().all(|node| node.id != node_id));
        assert!(document.edges.iter().all(|edge| {
            let (from, _) = split_endpoint(&edge.source);
            let (to, _) = split_endpoint(&edge.target);
            from != node_id && to != node_id
        }));
    }

    #[test]
    fn add_node_then_undo_restores_document() {
        let mut app = new_app();
        let before = app.projection().expect("projection").nodes.len();
        app.handle_action("addNode", Some(&json!({ "kind": "note" })), &ViewState::default(), &meta("local")).expect("add");
        assert_eq!(app.projection().expect("projection").nodes.len(), before + 1);
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").nodes.len(), before);
    }

    #[test]
    fn patch_slider_value_coalesces_into_one_edit() {
        let mut app = new_app();
        app.handle_action("addNode", Some(&json!({ "kind": "slider" })), &ViewState::default(), &meta("local")).expect("add slider");
        let node_id = app.projection().expect("projection").nodes.iter().find(|node| matches!(node.kind, DagNodeKind::Slider { .. })).map(|node| node.id.clone()).expect("slider");
        for value in [1.0, 2.0, 5.0] {
            app.handle_action("patchDagNodes", Some(&json!({ "nodeIds": [node_id], "field": "value", "value": value })), &ViewState::default(), &meta("local")).expect("patch slider");
        }
        let slider_value = app
            .projection()
            .expect("projection")
            .nodes
            .iter()
            .find(|node| node.id == node_id)
            .and_then(|node| match &node.kind {
                DagNodeKind::Slider { value, .. } => Some(*value),
                _ => None,
            })
            .expect("slider value");
        assert_eq!(slider_value, 5.0);
    }

    /// 🧪 Two instances apply DISJOINT edits (A adds a note node, B adds a slider node) and converge to
    /// contain BOTH via a `MemoryBackbone` — impossible with whole-document snapshots.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        use vcs::MemoryBackbone;
        let mut instance_a = new_app();
        let mut instance_b = new_app();
        let base = instance_a.projection().expect("projection").nodes.len();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://dag-convergence", "mem://dag-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        instance_a.handle_action("addNode", Some(&json!({ "kind": "note" })), &ViewState::default(), &meta("actor-a")).expect("a adds note");
        instance_b.handle_action("addNode", Some(&json!({ "kind": "slider" })), &ViewState::default(), &meta("actor-b")).expect("b adds slider");

        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection");
        let projection_b = instance_b.projection().expect("projection");
        assert_eq!(projection_a.nodes.len(), base + 2, "instance A has both new nodes");
        assert_eq!(projection_b.nodes.len(), base + 2, "instance B has both new nodes");
        assert!(projection_a.nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Note { .. })));
        assert!(projection_a.nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Slider { .. })));
    }
}
//#endregion 🧪Tests
