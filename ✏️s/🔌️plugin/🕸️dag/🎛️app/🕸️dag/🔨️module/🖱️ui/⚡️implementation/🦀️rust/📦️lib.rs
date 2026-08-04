//! 🖥️ DAG app — DocumentApp impl, render, manifest (constitutional: ui).

use dag::{DagDocument, DAG_DOCUMENT_SCHEMA};
use dag_engine::{connect_edge, dag_config_camera, default_node_for_kind, document_to_workflow, next_node_id, node_patch_for_field, split_endpoint, DagConfig};
use dag_op::{DagConfigOperation, DagOperation};
use dag_protocol::{DagCommand, DagNodeGraphEditOp};
use infinite_board_port_directed_dag::{
    dag_document_from_fixture, dag_fixture_from_document, dag_fixture_to_wire_literal, dag_node_kind_tag, default_dag_document, DagCamera, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodePatch, DagNodeSpec,
};
use protocol::CollectionOperation;
use semio_framework_plugin::{
    app_labels, build_node_graph_scene, build_text_editor_scene, create_default_layout, tree_item, tree_item_desc, tree_item_with_action, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number,
    ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppLabels, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Label, Locale,
    LocalizedLabel, MediaClass, MediaForm, MediaType, NodeGraphScene, NodeGraphViewport, OsMediaCapability, PanelGroup, SurfaceKind, Terminology, TextEditorScene, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::{json, Value};

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

//#region 🔖️DocumentHelpers
fn dag_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(DAG_PLAY_APP_ID).action(action, args)
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

//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the DAG app; one field per label makes every locale combination
    /// compile-checked. This app has no separate reuse-terminology concept, so the `reuse_*` cells
    /// repeat the `native_*` text verbatim.
    struct DagPlayLabels {
        nodes: native_en "Nodes", native_de "Knoten", reuse_en "Nodes", reuse_de "Knoten";
        edges: native_en "Edges", native_de "Kanten", reuse_en "Edges", reuse_de "Kanten";
        empty: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        kind_computation: native_en "Computation", native_de "Berechnung", reuse_en "Computation", reuse_de "Berechnung";
        kind_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        kind_select: native_en "Select", native_de "Auswahl", reuse_en "Select", reuse_de "Auswahl";
        kind_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        kind_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        kind_screen: native_en "Screen", native_de "Bildschirm", reuse_en "Screen", reuse_de "Bildschirm";
        select_a_node: native_en "Select a node in the document.", native_de "Wählen Sie einen Knoten im Dokument aus.", reuse_en "Select a node in the document.", reuse_de "Wählen Sie einen Knoten im Dokument aus.";
        node_not_found: native_en "Node not found", native_de "Knoten nicht gefunden", reuse_en "Node not found", reuse_de "Knoten nicht gefunden";
        slider_group: native_en "slider", native_de "schieberegler", reuse_en "slider", reuse_de "schieberegler";
        node_group: native_en "Node", native_de "Knoten", reuse_en "Node", reuse_de "Knoten";
        field_value: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        field_min: native_en "Min", native_de "Min", reuse_en "Min", reuse_de "Min";
        field_max: native_en "Max", native_de "Max", reuse_en "Max", reuse_de "Max";
        field_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        field_kind: native_en "Kind", native_de "Typ", reuse_en "Kind", reuse_de "Typ";
        field_id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        selected_suffix: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
    }
}

/// 🗣️ B1: `cfg.locale`-driven counterpart to the deleted `ViewState`-driven locale read.
fn is_de_locale(cfg: &DagConfig) -> bool {
    cfg.locale.starts_with("de")
}

/// 🗣️ Derives the compile-time-checked `Locale` from the BCP-47 `cfg.locale` tag.
fn dag_locale(cfg: &DagConfig) -> Locale {
    if is_de_locale(cfg) {
        Locale::De
    } else {
        Locale::En
    }
}

/// 🗣️ Resolves the active label set from `cfg.locale`; this app has no terminology variant, so
/// `Terminology` is always `Native`.
fn dag_play_labels(cfg: &DagConfig) -> &'static DagPlayLabels {
    DagPlayLabels::labels(dag_locale(cfg), Terminology::Native)
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn build_document_tree(document: &DagDocument, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    let node_items: Vec<UiTreeItemNode> = document
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(
                format!("dag-play-document.node.{}", node.id),
                Label::data(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
                Some(dag_node_kind_tag(&node.kind).into()),
                dag_action("setSelection", Some(json!({ "ids": [node.id.clone()] }))),
            )
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = document.edges.iter().map(|edge| tree_item_desc(format!("dag-play-document.edge.{}", edge.id), Label::data(format!("{} → {}", edge.source, edge.target)), Some(edge.id.clone()))).collect();
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
    UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None, menu: None })
}

fn build_catalogue_tree(labels: &DagPlayLabels) -> UiNode {
    let kinds = [("computation", labels.kind_computation), ("slider", labels.kind_slider), ("select", labels.kind_select), ("screen", labels.kind_screen), ("note", labels.kind_note), ("preview", labels.kind_preview)];
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "dag-play-catalogue.node-kinds".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: kinds.iter().map(|(kind, label)| tree_item_with_action(format!("dag-play-catalogue.kind.{kind}"), *label, Some((*kind).into()), dag_action("addNode", Some(json!({ "kind": kind }))))).collect(),
        }],
        presence: UiPresence::default(),
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
        menu: None,
    })
}

fn inspector_number_field(node_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[f64], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{field_id}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER)) },
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

fn inspector_text_field(node_ids: &[String], field_id: &str, label: impl Into<Label>, values: &[String], field: &str) -> UiNode {
    let mixed = ui_inspector_mixed_text(values);
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: field_id.into(),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            presence: UiPresence::default(),
            id: format!("{field_id}.input"),
            input_kind: "text".into(),
            value: mixed.value,
            placeholder: mixed.placeholder.map(Label::data),
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
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
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
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.node_not_found)],
            menu: None,
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if nodes.iter().all(|node| matches!(node.kind, DagNodeKind::Slider { .. })) {
        groups.push(UiInspectorFieldGroup {
            presence: UiPresence::default(),
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
                presence: UiPresence::default(),
                id: "dag-play-inspector.id".into(),
                label: labels.field_id.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
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
        base_fields.insert(0, ui_inspector_readonly_field("dag-play-inspector.id", labels.field_id, format!("{} {}", node_ids.len(), labels.selected_suffix.as_str())));
    }
    groups.push(UiInspectorFieldGroup { presence: UiPresence::default(), id: "dag-play-inspector.base".into(), label: labels.node_group.into(), default_open: None, fields: base_fields });
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_main_graph(document: &DagDocument, camera: &DagCamera, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    let (nodes, edges) = document_to_workflow(document);
    let viewport = NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom };
    let selection = selected.to_vec();
    build_node_graph_scene(DAG_PLAY_SURFACE_MAIN, DAG_PLAY_APP_ID, NodeGraphScene { editable: Some(true), selection, ..NodeGraphScene::base(nodes, edges, viewport) })
}

fn render_compiled_dag(document: &DagDocument, camera: &DagCamera) -> UiNode {
    let fixture = dag_fixture_from_document(document, camera.clone());
    build_text_editor_scene(DAG_PLAY_SURFACE_COMPILED, DAG_PLAY_APP_ID, TextEditorScene::base(dag_fixture_to_wire_literal(&fixture), Some("wire".into()), None))
}
//#endregion 🔖️Render

//#region 🔖️DagPlayApp
/// 🧪️ B1: unit struct — every former `DagPlayRuntime`/`ViewState.locale` field now lives in
/// `dag_engine::DagConfig` (see `DocumentApp::Config`), written through `dag_op::DagConfigOperation`s.
#[derive(Default)]
pub struct DagPlayApp;

/// 🗑️ Builds the removal `DagOperation`s plus the config op that CLEARS the whole selection, or
/// `None` when nothing in `node_ids` exists to remove — shared by `DagCommand::DeleteSelection` and
/// `DagNodeGraphEditOp::DeleteSelection` (both were the same `handle_action` "deleteSelection" logic,
/// reachable from two different action ids pre-B1). `DagCommand::RemoveNode` deliberately does NOT use
/// this helper: it only pulls the removed id out of the selection, never clears it outright.
fn delete_selection_result(document: &DagDocument, node_ids: &[String]) -> Option<(Vec<DagOperation>, DagConfigOperation)> {
    let removes = remove_nodes_operations(document, node_ids);
    if removes.is_empty() {
        None
    } else {
        Some((removes, DagConfigOperation::SetSelection { node_ids: Vec::new() }))
    }
}

impl DocumentApp for DagPlayApp {
    type Projection = DagDocument;
    type Operation = DagOperation;
    type Config = DagConfig;
    type ConfigOperation = DagConfigOperation;
    type Command = DagCommand;

    fn app_id(&self) -> &str {
        DAG_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        DAG_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> DagDocument {
        default_dag_document()
    }

    fn whole_document_operation(&self, projection: DagDocument) -> Option<DagOperation> {
        Some(DagOperation::SetDocument { document: projection })
    }

    /// 🏷️ Maps each `DagCommand` variant back to the action id it was declared under in
    /// `create_dag_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check. `SetLocale` has no manifest declaration (host-pushed, not a
    /// user-facing action — see `dag_protocol::DagCommand::SetLocale`'s doc), matching
    /// `shooting_protocol::ShootingCommand::SetLocale`'s equally-undeclared precedent.
    fn command_id(&self, command: &DagCommand) -> &str {
        match command {
            DagCommand::AddNode { .. } => "addNode",
            DagCommand::RemoveNode { .. } => "removeNode",
            DagCommand::DeleteSelection => "deleteSelection",
            DagCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            DagCommand::ConnectMediaPorts { .. } => "connectMediaPorts",
            DagCommand::Disconnect { .. } => "disconnect",
            DagCommand::MoveMediaNode { .. } => "moveMediaNode",
            DagCommand::RenameDagNode { .. } => "renameDagNode",
            DagCommand::Reorganize => "reorganize",
            DagCommand::PatchDagNodes { .. } => "patchDagNodes",
            DagCommand::SetSelection { .. } => "setSelection",
            DagCommand::SelectNode { .. } => "selectNode",
            DagCommand::NodeGraphSelect { .. } => "nodeGraphSelect",
            DagCommand::NodeGraphHover => "nodeGraphHover",
            DagCommand::NodeGraphViewport { .. } => "nodeGraphViewport",
            DagCommand::GraphPointerDown => "graphPointerDown",
            DagCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &DagCommand, doc: &DocumentView<'_, DagDocument>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagOperation, DagConfigOperation>, Fault> {
        let document = doc.projection;
        let config = cfg.projection;
        match command {
            DagCommand::SetSelection { ids } => Ok(Emit::config(vec![DagConfigOperation::SetSelection { node_ids: ids.clone() }]),
            DagCommand::SelectNode { node_id } => Ok(Emit::config(vec![DagConfigOperation::SetSelection { node_ids: vec![node_id.clone()] }]),
            DagCommand::NodeGraphSelect { node_ids } => Ok(Emit::config(vec![DagConfigOperation::SetSelection { node_ids: node_ids.clone() }]),
            DagCommand::NodeGraphHover => Ok(Emit::default(),
            DagCommand::GraphPointerDown => Ok(Emit::config(vec![DagConfigOperation::SetSelection { node_ids: Vec::new() }]),
            DagCommand::NodeGraphViewport { x, y, zoom } => Ok(Emit::config(vec![DagConfigOperation::SetCamera { x: *x, y: *y, zoom: *zoom }]),
            DagCommand::SetLocale { value } => Ok(Emit::config(vec![DagConfigOperation::SetLocale { value: value.clone() }]),
            DagCommand::NodeGraphEdit { operations } => {
                let mut document_operations: Vec<DagOperation> = Vec::new();
                let mut config_operations: Vec<DagConfigOperation> = Vec::new();
                for sub_operation in operations {
                    match sub_operation {
                        DagNodeGraphEditOp::SetFixture { fixture_json } => {
                            if let Ok(fixture) = serde_json::from_str::<DagFixture>(fixture_json) {
                                config_operations.push(DagConfigOperation::SetCamera { x: fixture.camera.x, y: fixture.camera.y, zoom: fixture.camera.zoom });
                                document_operations.push(DagOperation::SetDocument { document: dag_document_from_fixture(&fixture) });
                            }
                        }
                        DagNodeGraphEditOp::DeleteSelection => {
                            if let Some((removes, clear_selection)) = delete_selection_result(document, &config.selected_node_ids) {
                                document_operations.extend(removes);
                                config_operations.push(clear_selection);
                            }
                        }
                        DagNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                            if let Ok(edge) = connect_edge(document, source_node_id, source_port_id, target_node_id, target_port_id) {
                                document_operations.push(DagOperation::Edges(CollectionOperation::Add { id: edge.id.clone(), at: document.edges.len(), item: edge }));
                            }
                        }
                    }
                }
                Emit { document_operations, config_operations, ..Default::default() }
            }
            DagCommand::DeleteSelection => match delete_selection_result(document, &config.selected_node_ids) {
                Some((removes, clear_selection)) => Ok(Emit { document_operations: removes, config_operations: vec![clear_selection], ..Default::default() },
                None => Ok(Emit::default(),
            },
            DagCommand::RenameDagNode { old_id, value } => {
                let trimmed = value.trim();
                if trimmed.is_empty() || trimmed == old_id.as_str() || document.nodes.iter().any(|node| node.id == trimmed) {
                    return Emit::default();
                }
                let nodes: Vec<DagNodeSpec> = document.nodes.iter().map(|node| if &node.id == old_id { DagNodeSpec { id: trimmed.into(), ..node.clone() } } else { node.clone() }).collect();
                let edges: Vec<DagFixtureEdge> = document
                    .edges
                    .iter()
                    .map(|edge| {
                        let (from_node, from_port) = split_endpoint(&edge.source);
                        let (to_node, to_port) = split_endpoint(&edge.target);
                        DagFixtureEdge {
                            source: if &from_node == old_id { format!("{trimmed}@{from_port}") } else { edge.source.clone() },
                            target: if &to_node == old_id { format!("{trimmed}@{to_port}") } else { edge.target.clone() },
                            ..edge.clone()
                        }
                    })
                    .collect();
                Emit { document_operations: vec![DagOperation::SetNodes { nodes }, DagOperation::SetEdges { edges }], config_operations: vec![DagConfigOperation::SetSelection { node_ids: vec![trimmed.to_string()] }], ..Default::default() }
            }
            DagCommand::RemoveNode { node_id } => {
                let removes = remove_nodes_operations(document, std::slice::from_ref(node_id));
                if removes.is_empty() {
                    Emit::default()
                } else {
                    Emit { document_operations: removes, config_operations: vec![DagConfigOperation::SetSelection { node_ids: config.selected_node_ids.iter().filter(|id| *id != node_id).cloned().collect() }], ..Default::default() }
                }
            }
            DagCommand::Disconnect { edge_id } => {
                if document.edges.iter().any(|edge| &edge.id == edge_id) {
                    Emit::operations(vec![DagOperation::Edges(CollectionOperation::Remove { id: edge_id.clone() })])
                } else {
                    Emit::default()
                }
            }
            DagCommand::MoveMediaNode { node_id, x, y } => {
                if document.nodes.iter().any(|node| &node.id == node_id) {
                    Emit::amend(vec![DagOperation::Nodes(CollectionOperation::Patch { id: node_id.clone(), patch: DagNodePatch { x: Some(*x), y: Some(*y), ..Default::default() } })], format!("move-{node_id}"))
                } else {
                    Emit::default()
                }
            }
            DagCommand::ConnectMediaPorts { source_node_id, source_port_id, target_node_id, target_port_id } => match connect_edge(document, source_node_id, source_port_id, target_node_id, target_port_id) {
                Ok(edge) => Ok(Emit::operations(vec![DagOperation::Edges(CollectionOperation::Add { id: edge.id.clone(), at: document.edges.len(), item: edge })]),
                Err(_) => Ok(Emit::default(),
            },
            DagCommand::AddNode { kind, x, y } => {
                let id = next_node_id(document);
                let node = default_node_for_kind(kind, &id, x.unwrap_or(120.0), y.unwrap_or(120.0));
                Emit {
                    document_operations: vec![DagOperation::Nodes(CollectionOperation::Add { id: node.id.clone(), at: document.nodes.len(), item: node })],
                    config_operations: vec![DagConfigOperation::SetSelection { node_ids: vec![id] }],
                    ..Default::default()
                }
            }
            DagCommand::Reorganize => {
                let camera = dag_config_camera(config);
                if let Ok(mut host) = DagHost::load_fixture_json(&serde_json::to_string(&dag_fixture_from_document(document, camera)).unwrap_or_default()) {
                    let _ = host.reorganize(&DagLayoutOptions::default());
                    if let Ok(json) = host.fixture_json() {
                        if let Ok(fixture) = serde_json::from_str::<DagFixture>(&json) {
                            return Emit::operations(vec![DagOperation::SetNodes { nodes: fixture.nodes }]);
                        }
                    }
                }
                Emit::default()
            }
            DagCommand::PatchDagNodes { node_ids, field, value } => {
                let operations: Vec<DagOperation> = document
                    .nodes
                    .iter()
                    .filter(|node| node_ids.contains(&node.id))
                    .filter_map(|node| node_patch_for_field(node, field, Some(value.as_str())).map(|patch| DagOperation::Nodes(CollectionOperation::Patch { id: node.id.clone(), patch })))
                    .collect();
                if operations.is_empty() {
                    Emit::default()
                } else {
                    Emit::amend(operations, format!("patch-{field}-{}", node_ids.join(",")))
                }
            }
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, DagDocument>, cfg: &ConfigView<'_, DagConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let selected = &config.selected_node_ids;
        let camera = dag_config_camera(config);
        let labels = dag_play_labels(config);
        match body_key {
            DAG_PLAY_BODY_MAIN => render_main_graph(document, &camera, selected, labels),
            DAG_PLAY_BODY_COMPILED => render_compiled_dag(document, &camera),
            DAG_PLAY_BODY_DOCUMENT => build_document_tree(document, selected, labels),
            DAG_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            DAG_PLAY_BODY_INSPECTOR => build_inspector_tree(document, selected, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &DocumentView<'_, DagDocument>,
        cfg: &ConfigView<'_, DagConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let labels = dag_play_labels(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        let selected = &cfg.projection.selected_node_ids;
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), selected, &[]);
        let hit_edge_id = request.surface.as_ref().and_then(|target| target.hits.iter().find(|hit| hit.domain == "edge")).map(|hit| hit.id.clone());

        // 🗂️ Grouped disclosure: `addNode`/`reorganize` stay top-level (the most frequent verbs);
        // `renameDagNode` joins them only for a single-node selection; `disconnect` folds into the
        // "transfer" taxonomy group when an edge is hit — `organize_context_menu` (applied automatically
        // at the `VcsDocumentApp::context_menu` funnel) sorts groups into `RIBBON_PARENT_CATEGORIES`
        // order and inserts the pre-destructive separator itself, so no `.separator()` call is needed
        // ahead of the `deleteSelection`/`nodeGraphEdit` destructive row below.
        let mut menu = Menu::of(registry).action_args("addNode", json!({ "kind": "computation" })).action("reorganize");
        if nodes.len() == 1 {
            menu = menu.action("renameDagNode");
        }
        if let Some(edge_id) = hit_edge_id {
            menu = menu.group("transfer", |m| m.action_args("disconnect", json!({ "edgeId": edge_id })));
        }
        if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }
}
//#endregion 🔖️DagPlayApp

//#region 🔖️Manifest
pub fn create_dag_app() -> App {
    App::from_builder(
        App::builder(DAG_PLAY_APP_ID, LocalizedLabel::native("DAG", "DAG")).document(["semio", "mathematical", "graph", "port", "directed", "dag"])
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
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(DAG_PLAY_WINDOW_MAIN, LocalizedLabel::native("DAG", "DAG"), DAG_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "graph-dag")
            .window_kind(DAG_PLAY_WINDOW_COMPILED, LocalizedLabel::native("DSL", "DSL"), DAG_PLAY_BODY_COMPILED, SurfaceKind::NodeGraph, "code")
            .default_layout(create_default_layout(
                &[DAG_PLAY_WINDOW_MAIN.into(), DAG_PLAY_WINDOW_COMPILED.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["DAG".into(), "DSL".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                DAG_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                DAG_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                DAG_PLAY_BODY_INSPECTOR,
            )
            // ✏️ Document-mutating: dispatched as VCS operations with a true inverse.
            // 🗂️ Referenced by `DagPlayApp::context_menu` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"), ActionKind::Operation).with_category("create"))
            .operation("removeNode", LocalizedLabel::native("Remove Node", "Knoten entfernen"))
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Operation).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"), ActionKind::Operation).with_category("selection"))
            .operation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
            .action_with(ActionDefinition::new_catalog("disconnect", LocalizedLabel::native("Disconnect", "Trennen"), ActionKind::Operation).with_category("transfer"))
            .operation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("renameDagNode", LocalizedLabel::native("Rename Node", "Knoten umbenennen"), ActionKind::Operation).with_category("actions"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation).with_category("transform"))
            .operation("patchDagNodes", LocalizedLabel::native("Patch Nodes", "Knoten patchen"))
            // 👁️ Ephemeral view state — selection and camera/viewport.
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("selectNode", LocalizedLabel::native("Select Node", "Knoten auswählen"))
            .view_action("nodeGraphSelect", LocalizedLabel::native("Node Graph Select", "Knotengraph auswählen"))
            .view_action("nodeGraphHover", LocalizedLabel::native("Node Graph Hover", "Knotengraph-Hover"))
            .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            .view_action("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"))
            .keybinding("delete,backspace", "deleteSelection")
            // 📝️ Staged argument form for the panel-visible create action.
            .action_args("addNode", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Typ"), vec![
                    ActionArgOption::new("computation", LocalizedLabel::native("Computation", "Berechnung")),
                    ActionArgOption::new("slider", LocalizedLabel::native("Slider", "Schieberegler")),
                    ActionArgOption::new("select", LocalizedLabel::native("Select", "Auswahl")),
                    ActionArgOption::new("screen", LocalizedLabel::native("Screen", "Bildschirm")),
                    ActionArgOption::new("note", LocalizedLabel::native("Note", "Notiz")),
                    ActionArgOption::new("preview", LocalizedLabel::native("Preview", "Vorschau")),
                ]).default_value("computation"),
            ])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1)
            // — dag has no user-visible config defaults to expose, so `config_spec()` stays the trait
            // default `ConfigSpec::empty()`; declaring it explicitly here still keeps this app's typed
            // channel surface consistent with `shooting_ui::create_shooting_app`'s convention.
            .config(DagPlayApp::default().config_spec()),
    )
    .example("demo", LocalizedLabel::native("Demo", "Demo"), serde_json::to_string(&default_dag_document()).expect("default DAG document has no non-string map keys or non-finite floats, so JSON serialization is infallible"), "cylinder")
    .workflow("dag", "DAG", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, ViewState};

    fn new_app() -> VcsDocumentApp<DagPlayApp> {
        testkit::new_app::<DagPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so kind discipline runs.
    fn new_app_with_registry() -> VcsDocumentApp<DagPlayApp> {
        testkit::new_app_with_registry::<DagPlayApp>(create_dag_app)
    }

    #[test]
    fn dag_play_labels_resolve_native_by_default() {
        let mut app = new_app();
        let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Nodes"));
        assert!(json.contains("Edges"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
    /// passing a `ViewState` into `render`/`app_labels`/`context_menu` for this purpose.
    #[test]
    fn dag_play_labels_resolve_native_in_german() {
        let mut app = new_app();
        app.dispatch_typed(DagCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let node = app.render(DAG_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
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
    fn add_node_action_updates_document_and_selects_the_new_node() {
        let mut app = new_app();
        app.dispatch_typed(DagCommand::AddNode { kind: "slider".into(), x: None, y: None }, &testkit::meta("local")).expect("add node");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Slider { .. })));
        let added_id = document.nodes.last().expect("added node").id.clone();
        let node = app.render(DAG_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains(&added_id), "the new node becomes the config selection");
    }

    #[test]
    fn rename_dag_node_rewrites_nodes_and_edges() {
        let mut app = new_app();
        let old_id = app.projection().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");
        app.dispatch_typed(DagCommand::RenameDagNode { old_id: old_id.clone(), value: "renamed-node".into() }, &testkit::meta("local")).expect("rename");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().any(|node| node.id == "renamed-node"));
        assert!(document.nodes.iter().all(|node| node.id != old_id));
    }

    #[test]
    fn remove_node_deletes_node_and_connected_edges_and_prunes_selection() {
        let mut app = new_app();
        let node_id = app.projection().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");
        app.dispatch_typed(DagCommand::SetSelection { ids: vec![node_id.clone()] }, &testkit::meta("local")).expect("select");
        app.dispatch_typed(DagCommand::RemoveNode { node_id: node_id.clone() }, &testkit::meta("local")).expect("remove");
        let document = app.projection().expect("projection");
        assert!(document.nodes.iter().all(|node| node.id != node_id));
        assert!(document.edges.iter().all(|edge| {
            let (from, _) = split_endpoint(&edge.source);
            let (to, _) = split_endpoint(&edge.target);
            from != node_id && to != node_id
        }));
        let node = app.render(DAG_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        assert!(!serde_json::to_string(&node).unwrap().contains(&node_id), "the removed node is pruned from the config selection");
    }

    #[test]
    fn add_node_then_undo_restores_document() {
        let mut app = new_app();
        let before = app.projection().expect("projection").nodes.len();
        app.dispatch_typed(DagCommand::AddNode { kind: "note".into(), x: None, y: None }, &testkit::meta("local")).expect("add");
        assert_eq!(app.projection().expect("projection").nodes.len(), before + 1);
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").nodes.len(), before);
    }

    #[test]
    fn patch_slider_value_coalesces_into_one_edit() {
        let mut app = new_app();
        app.dispatch_typed(DagCommand::AddNode { kind: "slider".into(), x: None, y: None }, &testkit::meta("local")).expect("add slider");
        let node_id = app.projection().expect("projection").nodes.iter().find(|node| matches!(node.kind, DagNodeKind::Slider { .. })).map(|node| node.id.clone()).expect("slider");
        for value in [1.0, 2.0, 5.0] {
            app.dispatch_typed(DagCommand::PatchDagNodes { node_ids: vec![node_id.clone()], field: "value".into(), value: value.to_string() }, &testkit::meta("local")).expect("patch slider");
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

    /// 🧪️ `setSelection`/`selectNode`/`nodeGraphSelect` are three distinct declared actions that all
    /// drive the same config selection — B1 gave each its own `DagCommand` variant (matching the
    /// manifest 1:1) instead of the pre-B1 shared `handle_action` match arm.
    #[test]
    fn set_selection_select_node_and_node_graph_select_all_drive_config_selection() {
        let mut app = new_app();
        let node_id = app.projection().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");

        app.dispatch_typed(DagCommand::SetSelection { ids: vec![node_id.clone()] }, &testkit::meta("local")).expect("setSelection");
        assert!(serde_json::to_string(&app.render(DAG_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render")).unwrap().contains(&node_id));

        app.dispatch_typed(DagCommand::GraphPointerDown, &testkit::meta("local")).expect("clear");
        app.dispatch_typed(DagCommand::SelectNode { node_id: node_id.clone() }, &testkit::meta("local")).expect("selectNode");
        assert!(serde_json::to_string(&app.render(DAG_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render")).unwrap().contains(&node_id));

        app.dispatch_typed(DagCommand::GraphPointerDown, &testkit::meta("local")).expect("clear");
        app.dispatch_typed(DagCommand::NodeGraphSelect { node_ids: vec![node_id.clone()] }, &testkit::meta("local")).expect("nodeGraphSelect");
        assert!(serde_json::to_string(&app.render(DAG_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render")).unwrap().contains(&node_id));
    }

    #[test]
    fn node_graph_viewport_drives_the_rendered_camera() {
        let mut app = new_app();
        app.dispatch_typed(DagCommand::NodeGraphViewport { x: 10.0, y: 20.0, zoom: 2.0 }, &testkit::meta("local")).expect("viewport");
        let node = app.render(DAG_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let payload: Value = serde_json::to_value(&node).unwrap();
        assert_eq!(payload["nodeGraph"]["viewport"], json!({ "x": 10.0, "y": 20.0, "zoom": 2.0 }));
    }

    /// 🧪️ `nodeGraphEdit` batches multiple sub-edits (connect + delete-selection here) into a single
    /// typed command — mirrors the pre-B1 JSON `operations` array, now closed and typed via
    /// `dag_protocol::DagNodeGraphEditOp`.
    #[test]
    fn node_graph_edit_batches_connect_then_delete_selection() {
        let mut app = new_app();
        let (source_id, target_id) = {
            let projection = app.projection().expect("projection");
            (projection.nodes[0].id.clone(), projection.nodes[1].id.clone())
        };
        let edges_before = app.projection().expect("projection").edges.len();
        app.dispatch_typed(
            DagCommand::NodeGraphEdit { operations: vec![DagNodeGraphEditOp::Connect { source_node_id: source_id.clone(), source_port_id: "out".into(), target_node_id: target_id.clone(), target_port_id: "in".into() }] },
            &testkit::meta("local"),
        )
        .expect("batched connect");
        assert!(app.projection().expect("projection").edges.len() >= edges_before, "connect either adds an edge or is a safe no-op (e.g. a cycle)");

        app.dispatch_typed(DagCommand::SetSelection { ids: vec![source_id.clone()] }, &testkit::meta("local")).expect("select");
        let nodes_before = app.projection().expect("projection").nodes.len();
        app.dispatch_typed(DagCommand::NodeGraphEdit { operations: vec![DagNodeGraphEditOp::DeleteSelection] }, &testkit::meta("local")).expect("batched delete");
        assert_eq!(app.projection().expect("projection").nodes.len(), nodes_before - 1);
    }

    /// 🧪️ Two instances apply DISJOINT edits (A adds a note node, B adds a slider node) and converge to
    /// contain BOTH via a `MemoryBackbone` — impossible with whole-document snapshots.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<DagPlayApp, (bool, bool)>("mem://dag-convergence", DagCommand::AddNode { kind: "note".into(), x: None, y: None }, DagCommand::AddNode { kind: "slider".into(), x: None, y: None }, |app| {
            let projection = app.projection().expect("projection");
            (projection.nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Note { .. })), projection.nodes.iter().any(|node| matches!(node.kind, DagNodeKind::Slider { .. })))
        });
    }

    #[test]
    fn ingest_operations_is_idempotent_for_dag() {
        testkit::assert_ingest_idempotent::<DagPlayApp, usize>(DagCommand::AddNode { kind: "note".into(), x: None, y: None }, |app| app.projection().expect("projection").nodes.len());
    }

    #[test]
    fn move_media_node_drag_coalesces_into_one_edit() {
        let mut app = new_app();
        let node_id = app.projection().expect("projection").nodes.first().map(|node| node.id.clone()).expect("node");
        for position in [10.0, 20.0, 30.0] {
            app.dispatch_typed(DagCommand::MoveMediaNode { node_id: node_id.clone(), x: position, y: position }, &testkit::meta("local")).expect("drag tick");
        }
        // A whole drag (three ticks, same coalesce key) is ONE undo step, not one-operation-per-tick.
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        let restored = app.projection().expect("projection");
        let original = default_dag_document().nodes.iter().find(|node| node.id == node_id).map(|node| node.x).expect("original x");
        assert_eq!(restored.nodes.iter().find(|node| node.id == node_id).unwrap().x, original, "undoing the coalesced drag restores the pre-drag position");
    }

    /// 🗂️ Grouped-context-menu disclosure: the top-level row budget stays small even with a large
    /// selection, and the known `deleteSelection` destructive row (dispatched via `nodeGraphEdit` —
    /// `NodeGraphDeleteDispatch::ViaNodeGraphEdit`) is always last, either as a top-level leaf or as the
    /// tail of its group — mirrors `flow_ui`'s `context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last`.
    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        use semio_framework_plugin::{ContextMenuHit, ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};

        let mut app = new_app_with_registry();
        let node_ids: Vec<String> = app.projection().expect("projection").nodes.iter().map(|node| node.id.clone()).collect();
        app.dispatch_typed(DagCommand::SetSelection { ids: node_ids.clone() }, &testkit::meta("local")).expect("setSelection");
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "nodeGraph".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget {
                surface_id: "main".into(),
                kind: "nodeGraph".into(),
                hits: vec![ContextMenuHit { domain: "node".into(), id: node_ids[0].clone(), label: None }],
                selection: vec![ContextMenuSelectionGroup { domain: "node".into(), ids: node_ids.clone() }],
                text: None,
            }),
            window_instance_id: None,
            point: None,
        };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("nodeGraphEdit");
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).map(|child| child.destructive == Some(true)).unwrap_or(false);
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection (via nodeGraphEdit) must be last: {menu:?}");
    }

    #[test]
    fn every_declared_action_is_registered_and_set_selection_is_a_view_action() {
        let definition = create_dag_app().definition;
        for command in [
            "addNode",
            "removeNode",
            "deleteSelection",
            "nodeGraphEdit",
            "connectMediaPorts",
            "disconnect",
            "moveMediaNode",
            "renameDagNode",
            "reorganize",
            "patchDagNodes",
            "setSelection",
            "selectNode",
            "nodeGraphSelect",
            "nodeGraphHover",
            "nodeGraphViewport",
            "graphPointerDown",
        ] {
            assert!(definition.actions.iter().any(|action| action.id == command), "registry declares {command}");
        }
        let mut app = new_app_with_registry();
        let result = app.dispatch_typed(DagCommand::SetSelection { ids: Vec::new() }, &testkit::meta("local")).expect("setSelection");
        assert!(result.operations.is_empty(), "setSelection (View) emits no operations even under registry enforcement");
    }
}
//#endregion 🧪️Tests
