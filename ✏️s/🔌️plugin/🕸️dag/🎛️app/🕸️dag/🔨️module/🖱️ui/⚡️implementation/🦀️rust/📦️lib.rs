//! 🖥️ DAG app — DocumentApp impl, render, manifest (constitutional: ui).

use dag::{DagDocument, DAG_DOCUMENT_SCHEMA};
use dag_engine::{connect_edge, default_node_for_kind, document_to_workflow, next_node_id, node_patch_for_field, split_endpoint};
use dag_op::DagOperation;
use infinite_board_port_directed_dag::{
    dag_document_from_fixture, dag_fixture_from_document, dag_fixture_to_wire_literal, dag_node_kind_tag, default_dag_document, DagCamera, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodePatch, DagNodeSpec,
};
use protocol::CollectionOperation;
use semio_framework_plugin::{
    build_node_graph_scene, build_text_editor_scene, create_default_layout, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef,
    ActionArgOption, ActionDescriptor, ActionEmit, App, DocumentApp, DocumentView, NodeGraphScene, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, ArtifactKindSpec, SurfaceKind, TextEditorScene, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode,
    UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
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
//#endregion 🔖️Constants

//#region 🔖️Runtime
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
//#endregion 🔖️Runtime

//#region 🔖️DocumentHelpers
fn dag_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: DAG_PLAY_APP_ID.into(), action: action.into(), args }
}

/// 🗑️ Operations removing `node_ids` and every edge touching them, for delete-node / delete-selection.
fn remove_nodes_operations(document: &DagDocument, node_ids: &[String]) -> Vec<DagOperation> {
    let mut operations: Vec<DagOperation> = document.nodes.iter().filter(|node| node_ids.contains(&node.id)).map(|node| DagOperation::Nodes(CollectionOperation::Remove { id: node.id.clone() })).collect();
    operations.extend(
        document
            .edges
            .iter()
            .filter(|edge| {
                let (from, _) = split_endpoint(&edge.source);
                let (to, _) = split_endpoint(&edge.target);
                node_ids.iter().any(|id| id == &from || id == &to)
            })
            .map(|edge| DagOperation::Edges(CollectionOperation::Remove { id: edge.id.clone() })),
    );
    operations
}

fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
}

fn tree_item_with_description(id: impl Into<String>, label: impl Into<String>, description: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: Some(description.into()),
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
}

fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the DAG app; one field per label makes every locale combination compile-checked.
struct DagPlayLabels {
    nodes: &'static str,
    edges: &'static str,
    empty: &'static str,
    kind_computation: &'static str,
    kind_slider: &'static str,
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
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
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
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
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
    let mut sections = vec![
        UiTreeSectionNode {
            id: "dag-play-document.nodes".into(),
            label: Some(labels.nodes.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: if node_items.is_empty() { vec![tree_item("dag-play-document.nodes.empty", labels.empty)] } else { node_items },
        },
        UiTreeSectionNode {
            id: "dag-play-document.edges".into(),
            label: Some(labels.edges.into()),
            default_open: Some(false),
            presence: UiPresence::default(),
            items: if edge_items.is_empty() { vec![tree_item("dag-play-document.edges.empty", labels.empty)] } else { edge_items },
        },
    ];
    let selected_ids: std::collections::HashSet<String> = selected.iter().map(|id| format!("dag-play-document.node.{id}")).collect();
    semio_framework_plugin::ui_tree_stamp_presence(&mut sections, &selected_ids, &std::collections::HashSet::new());
    UiNode::Tree(UiTreeNode {
        sections,
        presence: UiPresence::default(),
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
        menu: None,
    })
}

fn build_catalogue_tree(labels: &DagPlayLabels) -> UiNode {
    let kinds = [
        ("computation", labels.kind_computation),
        ("slider", labels.kind_slider),
        ("select", labels.kind_select),
        ("screen", labels.kind_screen),
        ("note", labels.kind_note),
        ("preview", labels.kind_preview),
    ];
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "dag-play-catalogue.node-kinds".into(),
            label: Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: kinds
                .iter()
                .map(|(kind, label)| {
                    tree_item_with_action(
                        format!("dag-play-catalogue.kind.{kind}"),
                        *label,
                        Some((*kind).into()),
                        dag_action("addNode", Some(json!({ "kind": kind }))),
                    )
                })
                .collect(),
            }],
        presence: UiPresence::default(),
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
        menu: None,
    })
}

fn inspector_number_field(node_ids: &[String], field_id: &str, label: &str, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {presence: UiPresence::default(),
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
    })
}

fn inspector_text_field(node_ids: &[String], field_id: &str, label: &str, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {presence: UiPresence::default(),
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        menu: None,
    })
}

fn build_inspector_tree(document: &DagDocument, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "dag-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.select_a_node)],
            menu: None,
        }]);
    }
    let nodes: Vec<&DagNodeSpec> = selected.iter().filter_map(|id| document.nodes.iter().find(|node| &node.id == id)).collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "dag-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.node_not_found)],
            menu: None,
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if nodes.iter().all(|node| matches!(node.kind, DagNodeKind::Slider { .. })) {
        groups.push(UiInspectorFieldGroup { presence: UiPresence::default(),
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
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "dag-play-inspector.id".into(),
                label: labels.field_id.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
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
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        );
    } else {
        base_fields.insert(0, ui_inspector_readonly_field("dag-play-inspector.id", labels.field_id, format!("{} {}", node_ids.len(), labels.selected_suffix)));
    }
    groups.push(UiInspectorFieldGroup { presence: UiPresence::default(), id: "dag-play-inspector.base".into(), label: labels.node_group.into(), default_open: None, fields: base_fields });
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_main_graph(document: &DagDocument, camera: &DagCamera, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    let (nodes_json, edges_json) = document_to_workflow(document);
    let viewport_json = serde_json::to_string(camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let selection_json = if selected.is_empty() { None } else { serde_json::to_string(selected).ok() };
    let context_menu_json = json!([{
        "id": "delete-selection",
        "label": labels.delete_selection,
        "icon": "trash",
        "action": "nodeGraphEdit",
        "args": { "operations": [{ "operation": "deleteSelection" }] },
        "destructive": true,
    }])
    .to_string();
    build_node_graph_scene(DAG_PLAY_SURFACE_MAIN, DAG_PLAY_APP_ID, NodeGraphScene { editable: Some(true), selection_json, context_menu_json: Some(context_menu_json), ..NodeGraphScene::base(nodes_json, edges_json, viewport_json) })
}

fn render_compiled_dag(document: &DagDocument, camera: &DagCamera) -> UiNode {
    let fixture = dag_fixture_from_document(document, camera.clone());
    build_text_editor_scene(DAG_PLAY_SURFACE_COMPILED, DAG_PLAY_APP_ID, TextEditorScene::base(dag_fixture_to_wire_literal(&fixture), Some("wire".into()), None))
}
//#endregion 🔖️Render

//#region 🔖️DagPlayApp
#[derive(Default)]
pub struct DagPlayApp {
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
    type Operation = DagOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        DAG_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        DAG_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> DagDocument {
        default_dag_document()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, DagDocument>, _view_state: &ViewState) -> ActionEmit<DagOperation> {
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
                let operations = args.and_then(|value| value.get("operations")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let mut emitted: Vec<DagOperation> = Vec::new();
                for operation in operations {
                    match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                        "setFixture" => {
                            if let Some(fixture_json) = operation.get("fixtureJson").and_then(|value| value.as_str()) {
                                if let Ok(fixture) = serde_json::from_str::<DagFixture>(fixture_json) {
                                    self.runtime.camera = fixture.camera.clone();
                                    emitted.push(DagOperation::SetDocument { document: dag_document_from_fixture(&fixture) });
                                }
                            }
                        }
                        "deleteSelection" => {
                            let ids = self.runtime.selected_node_ids.clone();
                            let removes = remove_nodes_operations(document, &ids);
                            if !removes.is_empty() {
                                self.runtime.selected_node_ids.clear();
                                emitted.extend(removes);
                            }
                        }
                        "connect" => {
                            let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                            let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                            let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                            let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                            if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                if let Ok(edge) = connect_edge(document, from, from_port, to, to_port) {
                                    emitted.push(DagOperation::Edges(CollectionOperation::Add { id: edge.id.clone(), at: document.edges.len(), item: edge }));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                ActionEmit::operations(emitted)
            }
            "deleteSelection" => {
                let ids = self.runtime.selected_node_ids.clone();
                let removes = remove_nodes_operations(document, &ids);
                if removes.is_empty() {
                    return ActionEmit::default();
                }
                self.runtime.selected_node_ids.clear();
                ActionEmit::operations(removes)
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
                                    source: if from_node == old_id { format!("{trimmed}@{from_port}") } else { edge.source.clone() },
                                    target: if to_node == old_id { format!("{trimmed}@{to_port}") } else { edge.target.clone() },
                                    ..edge.clone()
                                }
                            })
                            .collect();
                        self.runtime.selected_node_ids = vec![trimmed.into()];
                        return ActionEmit::operations(vec![DagOperation::SetNodes { nodes }, DagOperation::SetEdges { edges }]);
                    }
                }
                ActionEmit::default()
            }
            "removeNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).or_else(|| args.and_then(|value| value.get("id"))).and_then(|value| value.as_str());
                if let Some(node_id) = node_id {
                    let removes = remove_nodes_operations(document, &[node_id.to_string()]);
                    if !removes.is_empty() {
                        self.runtime.selected_node_ids.retain(|id| id != node_id);
                        return ActionEmit::operations(removes);
                    }
                }
                ActionEmit::default()
            }
            "disconnect" => {
                let edge_id = args.and_then(|value| value.get("edgeId")).or_else(|| args.and_then(|value| value.get("synapseId"))).and_then(|value| value.as_str());
                match edge_id {
                    Some(edge_id) if document.edges.iter().any(|edge| edge.id == edge_id) => ActionEmit::operations(vec![DagOperation::Edges(CollectionOperation::Remove { id: edge_id.into() })]),
                    _ => ActionEmit::default(),
                }
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str());
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    if document.nodes.iter().any(|node| node.id == node_id) {
                        return ActionEmit::amend(vec![DagOperation::Nodes(CollectionOperation::Patch { id: node_id.into(), patch: DagNodePatch { x: Some(x), y: Some(y), ..Default::default() } })], format!("move-{node_id}"));
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
                        return ActionEmit::operations(vec![DagOperation::Edges(CollectionOperation::Add { id: edge.id.clone(), at: document.edges.len(), item: edge })]);
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
                ActionEmit::operations(vec![DagOperation::Nodes(CollectionOperation::Add { id: node.id.clone(), at: document.nodes.len(), item: node })])
            }
            "reorganize" => {
                if let Ok(mut host) = DagHost::load_fixture_json(&serde_json::to_string(&dag_fixture_from_document(document, self.runtime.camera.clone())).unwrap_or_default()) {
                    let _ = host.reorganize(&DagLayoutOptions::default());
                    if let Ok(json) = host.fixture_json() {
                        if let Ok(fixture) = serde_json::from_str::<DagFixture>(&json) {
                            return ActionEmit::operations(vec![DagOperation::SetNodes { nodes: fixture.nodes }]);
                        }
                    }
                }
                ActionEmit::default()
            }
            "patchDagNodes" => {
                let node_ids: Vec<String> = args.and_then(|value| value.get("nodeIds")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value"));
                let operations: Vec<DagOperation> =
                    document.nodes.iter().filter(|node| node_ids.contains(&node.id)).filter_map(|node| node_patch_for_field(node, field, raw_value).map(|patch| DagOperation::Nodes(CollectionOperation::Patch { id: node.id.clone(), patch }))).collect();
                if operations.is_empty() {
                    ActionEmit::default()
                } else {
                    ActionEmit::amend(operations, format!("patch-{field}-{}", node_ids.join(",")))
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
            tutorial_labels: HashMap::new(),
            group_labels: HashMap::new(),
        }
    }
}
//#endregion 🔖️DagPlayApp

//#region 🔖️Manifest
pub fn create_dag_app() -> App {
    App::from_builder(
        App::builder(DAG_PLAY_APP_ID, "DAG").document(["semio", "mathematical", "graph", "port", "directed", "dag"])
            .artifact_kind(ArtifactKindSpec {
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
            .window_kind(DAG_PLAY_WINDOW_MAIN, "DAG", DAG_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "graph-dag")
            .window_kind(DAG_PLAY_WINDOW_COMPILED, "DSL", DAG_PLAY_BODY_COMPILED, SurfaceKind::NodeGraph, "code")
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
            // 📝️ Staged argument form for the panel-visible create action.
            .action_args("addNode", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("computation", "Computation"),
                    ActionArgOption::new("slider", "Slider"),
                    ActionArgOption::new("select", "Select"),
                    ActionArgOption::new("screen", "Screen"),
                    ActionArgOption::new("note", "Note"),
                    ActionArgOption::new("preview", "Preview"),
                ]).default_value("computation"),
            ]),
    )
    .example("demo", "Demo", serde_json::to_string(&default_dag_document()).expect("default DAG document has no non-string map keys or non-finite floats, so JSON serialization is infallible"))
    .workflow("dag", "DAG", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
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

    /// 🧪️ Two instances apply DISJOINT edits (A adds a note node, B adds a slider node) and converge to
    /// contain BOTH via a `MemoryBackbone` — impossible with whole-document snapshots.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        use store::MemoryBackbone;
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
//#endregion 🧪️Tests
