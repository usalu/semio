//! 🧮️ Mathematical app — DocumentApp impl, render, manifest (constitutional: ui). B1: the pure-trait
//! pilot — `MathematicalPlayApp` is a unit struct; the former `MathPlayRuntime` app-struct `RefCell`
//! (the node-graph viewport camera) now lives in `mathematical_engine::MathConfig`, written via
//! `mathematical_op::MathConfigOperation`s (real `backwards`, no ad hoc inverse tracking); every action
//! dispatches through the single typed `mathematical_protocol::MathCommand` channel via
//! `DocumentApp::handle`.

use mathematical::{MathCamera, MathEdge, MathGeometry, MathGraph, MathNode, MathProjection};
use mathematical_engine::{algorithm_overlay, geometry_layers_json, mathematical_io, workflow_json, MathConfig};
use mathematical_op::{MathConfigOperation, MathOperation};
use mathematical_protocol::MathCommand;
use semio_framework_plugin::{
    create_default_layout, ui_text, ActionArgDef, ActionArgOption, App, ArtifactKindSpec, Canvas2dScene, ConfigView, DocumentApp, DocumentView, Emit, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType,
    NodeGraphScene, NodeGraphViewport, OsMediaCapability, SurfaceKind, UiComponentSceneNode, UiNode, UiPresence,
};
use store::DocumentPack;

//#region 🔖️Constants
const MATH_APP_ID: &str = "mathematical-play";
const MATH_WINDOW_GRAPH: &str = "math-graph";
const MATH_WINDOW_GEOMETRY: &str = "math-geometry";
const MATH_BODY_GRAPH: &str = "mathematical.play.graph";
const MATH_BODY_GEOMETRY: &str = "mathematical.play.geometry";
const MATH_DOCUMENT_SCHEMA: &str = "semio.mathematical/v1";
//#endregion 🔖️Constants

//#region 🔖️Render
fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: MATH_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    }
}

fn render_graph_window(graph: &MathGraph, camera: &MathCamera) -> UiNode {
    let (nodes, edges) = workflow_json(graph);
    let viewport = NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom };
    let mut scene = empty_component_scene(MATH_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) });
    UiNode::ComponentScene(scene)
}

fn render_geometry_window(geometry: &MathGeometry) -> UiNode {
    let mut scene = empty_component_scene(MATH_BODY_GEOMETRY, SurfaceKind::Canvas2d);
    scene.canvas_2d = Some(Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: geometry_layers_json(geometry) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️Render

//#region 🔖️MathematicalPlayApp
/// 🧪️ B1: unit struct — the former `MathPlayRuntime`/`self.runtime` field now lives in
/// `mathematical_engine::MathConfig` (see `DocumentApp::Config`), written through
/// `mathematical_op::MathConfigOperation`s.
#[derive(Default)]
pub struct MathematicalPlayApp;

impl DocumentApp for MathematicalPlayApp {
    type Projection = MathProjection;
    type Operation = MathOperation;
    type Config = MathConfig;
    type ConfigOperation = MathConfigOperation;
    type Command = MathCommand;

    fn app_id(&self) -> &str {
        MATH_APP_ID
    }

    fn document_schema(&self) -> &str {
        MATH_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> MathProjection {
        MathProjection::default()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(mathematical_io())
    }

    /// 🏷️ Maps each `MathCommand` variant back to the action id it was declared under in
    /// `create_mathematical_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &MathCommand) -> &str {
        match command {
            MathCommand::SetDocument { .. } => "setDocument",
            MathCommand::SetAlgorithm { .. } => "setAlgorithm",
            MathCommand::SetDirected { .. } => "setDirected",
            MathCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            MathCommand::NodeGraphViewport { .. } => "nodeGraphViewport",
            MathCommand::SetPoints { .. } => "setPoints",
            MathCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &MathCommand, doc: &DocumentView<'_, MathProjection>, _cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathOperation, MathConfigOperation>, Fault> {
        let projection = doc.projection;
        match command {
            MathCommand::SetDocument { graph, geometry } => {
                let Ok(graph) = mathematical::math_graph_from_dsl(graph.clone()) else {
                    return Emit::default();
                };
                let mut operations = Vec::new();
                if graph != projection.graph {
                    operations.push(MathOperation::SetGraph { graph });
                }
                if geometry != &projection.geometry {
                    operations.push(MathOperation::SetGeometry { geometry: geometry.clone() });
                }
                Emit::operations(operations)
            }
            MathCommand::SetAlgorithm { algorithm, seed } => {
                let mut graph = projection.graph.clone();
                graph.algorithm = algorithm.clone();
                graph.algorithm_seed = seed.clone();
                Emit::commit(vec![MathOperation::SetGraph { graph }], "setAlgorithm")
            }
            MathCommand::SetDirected { directed } => {
                let mut graph = projection.graph.clone();
                graph.directed = *directed;
                Emit::operations(vec![MathOperation::SetGraph { graph }])
            }
            // 🎨️ `nodeGraphActions.edit` (`"nodeGraphEdit"`) is the shared renderer-wide action id the
            // generic node-graph canvas dispatches interactive edit gestures under — see
            // `mathematical_protocol::MathCommand::NodeGraphEdit`'s doc comment. Field shape (a JSON
            // array of `{operation, ...}` tagged sub-edits) is unchanged from the pre-B1 `nodeGraphEdit`
            // action's `args.operations`.
            MathCommand::NodeGraphEdit { operations_json } => {
                let edit_operations: Vec<serde_json::Value> = serde_json::from_str(operations_json).unwrap_or_default();
                let mut graph = projection.graph.clone();
                let mut changed = false;
                for operation in edit_operations {
                    match operation.get("operation").and_then(serde_json::Value::as_str).unwrap_or("") {
                        "addNode" => {
                            let x = operation.get("x").and_then(serde_json::Value::as_f64).unwrap_or(0.0);
                            let y = operation.get("y").and_then(serde_json::Value::as_f64).unwrap_or(0.0);
                            let id = format!("n{}", graph.nodes.len());
                            graph.nodes.push(MathNode { label: id.to_uppercase(), id, x, y });
                            changed = true;
                        }
                        "move" => {
                            if let (Some(node_id), Some(x), Some(y)) = (operation.get("nodeId").and_then(serde_json::Value::as_str), operation.get("x").and_then(serde_json::Value::as_f64), operation.get("y").and_then(serde_json::Value::as_f64)) {
                                if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == node_id) {
                                    node.x = x;
                                    node.y = y;
                                    changed = true;
                                }
                            }
                        }
                        "connect" => {
                            if let (Some(source), Some(target)) = (operation.get("sourceNodeId").and_then(serde_json::Value::as_str), operation.get("targetNodeId").and_then(serde_json::Value::as_str)) {
                                let id = format!("e{}", graph.edges.len());
                                graph.edges.push(MathEdge { id, source: source.into(), target: target.into() });
                                changed = true;
                            }
                        }
                        "deleteSelection" => {
                            if let Some(ids) = operation.get("nodeIds").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                                graph.nodes.retain(|node| !ids.contains(&node.id));
                                graph.edges.retain(|edge| !ids.contains(&edge.source) && !ids.contains(&edge.target));
                                changed = true;
                            }
                        }
                        _ => {}
                    }
                }
                if changed {
                    Emit::operations(vec![MathOperation::SetGraph { graph }])
                } else {
                    Emit::default()
                }
            }
            // 👁️ Config-only: the node-graph viewport never touches the document — it's written into
            // `cfg`, session-only, no VCS edit, no undo entry on the document store.
            MathCommand::NodeGraphViewport { camera } => Ok(Emit::config(vec![MathConfigOperation::SetCamera { camera: camera.clone() }]),
            MathCommand::SetPoints { geometry } => Ok(Emit::operations(vec![MathOperation::SetGeometry { geometry: geometry.clone() }]),
            MathCommand::SetLocale { value } => Ok(Emit::config(vec![MathConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    /// 🎞️ `"result:out"` exports the active algorithm's per-node overlay (topo order/connected
    /// components/SCC group/BFS distance — the port recipe's `computation.mathematical`-kinded output);
    /// `"document:out"` replicates `DocumentApp::export_media`'s default whole-document-pack behavior
    /// (unreachable once this override exists).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, MathProjection>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let overlay = algorithm_overlay(&doc.projection.graph);
                let json = serde_json::to_string(&serde_json::json!({ "algorithm": doc.projection.graph.algorithm, "overlay": overlay })).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.mathematical".into(), json } })
            }
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, MathProjection>, cfg: &ConfigView<'_, MathConfig>) -> UiNode {
        match body_key {
            MATH_BODY_GRAPH => render_graph_window(&doc.projection.graph, &cfg.projection.camera),
            MATH_BODY_GEOMETRY => render_geometry_window(&doc.projection.geometry),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️MathematicalPlayApp

//#region 🔖️Manifest
pub fn create_mathematical_app() -> App {
    App::from_builder(
        App::builder(MATH_APP_ID, LocalizedLabel::native("Mathematical", "Mathematik"))
            .document(["semio", "mathematical"])
            .artifact_kind(ArtifactKindSpec {
                id: "computation.mathematical".into(),
                name: "Mathematical".into(),
                source_format: MATH_DOCUMENT_SCHEMA.into(),
                component_kind: "mathematical".into(),
                dimension: "graph".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
                schema: "computation.mathematical".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("math-app")
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind(MATH_WINDOW_GRAPH, LocalizedLabel::native("Graph", "Graph"), MATH_BODY_GRAPH, SurfaceKind::NodeGraph, "math-graph")
            .window_kind(MATH_WINDOW_GEOMETRY, LocalizedLabel::native("Geometry", "Geometrie"), MATH_BODY_GEOMETRY, SurfaceKind::Canvas2d, "hexagon")
            .default_layout(create_default_layout(&[MATH_WINDOW_GRAPH.into(), MATH_WINDOW_GEOMETRY.into()], "row", Some(&[60.0, 40.0]), Some(&["Graph".into(), "Geometry".into()])))
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .operation("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen"))
            .operation("setAlgorithm", LocalizedLabel::native("Set Algorithm", "Algorithmus festlegen"))
            .operation("setDirected", LocalizedLabel::native("Set Directed", "Gerichtet festlegen"))
            .operation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            .operation("setPoints", LocalizedLabel::native("Set Points", "Punkte festlegen"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 📝️ Staged argument forms for the graph analysis controls.
            .action_args("setAlgorithm", vec![
                ActionArgDef::select("algorithm", LocalizedLabel::native("Algorithm", "Algorithmus"), vec![
                    ActionArgOption::new("topo", LocalizedLabel::native("Topological Order", "Topologische Ordnung")),
                    ActionArgOption::new("components", LocalizedLabel::native("Connected Components", "Zusammenhangskomponenten")),
                    ActionArgOption::new("scc", LocalizedLabel::native("Strongly Connected Components", "Starke Zusammenhangskomponenten")),
                    ActionArgOption::new("bfs", LocalizedLabel::native("Breadth-First Distances", "Breitensuche-Distanzen")),
                ]).required(),
            ])
            .action_args("setDirected", vec![
                ActionArgDef::toggle("directed", LocalizedLabel::native("Directed", "Gerichtet")).default_value(true),
            ])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
            // WORKFLOWS-END-TO-END-TYPED-PORTS) — `mathematical_io()` is this port information's single
            // source of truth, reused here rather than duplicated.
            .io(mathematical_io()),
    )
    .example("demo", LocalizedLabel::native("Demo", "Demo"), <MathProjection as store::DocumentDsl>::print_dsl(&MathProjection::default()), "cylinder")
    .workflow("mathematical", "Mathematical", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, HistoryView, PluginApp, VcsDocumentApp};

    fn new_app() -> VcsDocumentApp<MathematicalPlayApp> {
        testkit::new_app::<MathematicalPlayApp>()
    }

    fn doc_view<'a>(projection: &'a MathProjection, history: &'a HistoryView) -> DocumentView<'a, MathProjection> {
        DocumentView { projection, history }
    }

    #[test]
    fn mathematical_io_is_declared_on_the_manifest() {
        let app = create_mathematical_app();
        assert_eq!(app.definition.io.artifact.id, "computation.mathematical");
        assert_eq!(app.definition.io.ports.len(), 1);
        assert_eq!(app.definition.io.ports[0].id, "result:out");
    }

    #[test]
    fn renders_node_graph_scene() {
        let app = MathematicalPlayApp::default();
        let projection = MathProjection::default();
        let history = HistoryView::empty();
        let config = MathConfig::default();
        let node = app.render(MATH_BODY_GRAPH, &doc_view(&projection, &history), &ConfigView { projection: &config });
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_canvas_2d_scene() {
        let app = MathematicalPlayApp::default();
        let projection = MathProjection::default();
        let history = HistoryView::empty();
        let config = MathConfig::default();
        let node = app.render(MATH_BODY_GEOMETRY, &doc_view(&projection, &history), &ConfigView { projection: &config });
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    /// 🎥️ `"nodeGraphViewport"` is a View command — it must never emit a `MathOperation` (no VCS edit,
    /// no undo entry) and instead write straight into the config store.
    #[test]
    fn node_graph_viewport_writes_config_not_document_operations() {
        let mut app = new_app();
        let camera = MathCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let result = app.dispatch_typed(MathCommand::NodeGraphViewport { camera: camera.clone() }, &testkit::meta("local")).expect("viewport");
        assert!(result.operations.is_empty(), "nodeGraphViewport must not emit a VCS operation");
        let node = app.render(MATH_BODY_GRAPH, None, &Default::default()).expect("render");
        let payload: serde_json::Value = serde_json::to_value(&node).unwrap();
        assert_eq!(payload["nodeGraph"]["viewport"]["zoom"], serde_json::json!(2.0));
    }

    /// 🎨️ Builds a `NodeGraphEdit` command carrying one tagged sub-edit — mirrors the shape
    /// `nodeGraphActions.edit`'s real dispatcher sends (`{ operations: [{ operation, ... }] }`).
    fn node_graph_edit(operation: serde_json::Value) -> MathCommand {
        MathCommand::NodeGraphEdit { operations_json: serde_json::to_string(&vec![operation]).unwrap() }
    }

    #[test]
    fn node_graph_edit_add_node_appends_a_node() {
        let mut app = new_app();
        let before = app.projection().expect("projection").graph.nodes.len();
        app.dispatch_typed(node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 1.0, "y": 2.0 })), &testkit::meta("local")).expect("add node");
        assert_eq!(app.projection().expect("projection").graph.nodes.len(), before + 1);
    }

    #[test]
    fn node_graph_edit_move_updates_node_position() {
        let mut app = new_app();
        let node_id = app.projection().expect("projection").graph.nodes[0].id.clone();
        app.dispatch_typed(node_graph_edit(serde_json::json!({ "operation": "move", "nodeId": node_id, "x": 42.0, "y": 43.0 })), &testkit::meta("local")).expect("move");
        let moved = app.projection().expect("projection").graph.nodes.iter().find(|node| node.id == node_id).cloned().expect("moved node");
        assert_eq!((moved.x, moved.y), (42.0, 43.0));
    }

    #[test]
    fn node_graph_edit_connect_appends_an_edge() {
        let mut app = new_app();
        let before = app.projection().expect("projection").graph.edges.len();
        app.dispatch_typed(node_graph_edit(serde_json::json!({ "operation": "connect", "sourceNodeId": "a", "targetNodeId": "d" })), &testkit::meta("local")).expect("connect");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.graph.edges.len(), before + 1);
        assert!(projection.graph.edges.iter().any(|edge| edge.source == "a" && edge.target == "d"));
    }

    #[test]
    fn node_graph_edit_delete_selection_removes_nodes_and_incident_edges() {
        let mut app = new_app();
        app.dispatch_typed(node_graph_edit(serde_json::json!({ "operation": "deleteSelection", "nodeIds": ["a"] })), &testkit::meta("local")).expect("delete");
        let projection = app.projection().expect("projection");
        assert!(!projection.graph.nodes.iter().any(|node| node.id == "a"));
        assert!(!projection.graph.edges.iter().any(|edge| edge.source == "a" || edge.target == "a"));
    }

    #[test]
    fn node_graph_edit_unknown_operation_and_empty_array_emit_no_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(node_graph_edit(serde_json::json!({ "operation": "unknownTag" })), &testkit::meta("local")).expect("no-op tag");
        assert!(result.operations.is_empty());
        let result = app.dispatch_typed(MathCommand::NodeGraphEdit { operations_json: "[]".into() }, &testkit::meta("local")).expect("empty array");
        assert!(result.operations.is_empty());
    }

    #[test]
    fn set_algorithm_updates_graph_and_seed() {
        let mut app = new_app();
        app.dispatch_typed(MathCommand::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }, &testkit::meta("local")).expect("set algorithm");
        let projection = app.projection().expect("projection");
        assert_eq!(projection.graph.algorithm, "bfs");
        assert_eq!(projection.graph.algorithm_seed.as_deref(), Some("a"));
    }

    #[test]
    fn set_directed_toggles_the_graph() {
        let mut app = new_app();
        app.dispatch_typed(MathCommand::SetDirected { directed: false }, &testkit::meta("local")).expect("set directed");
        assert!(!app.projection().expect("projection").graph.directed);
    }

    #[test]
    fn set_points_replaces_geometry() {
        let mut app = new_app();
        let geometry = MathGeometry { points: vec![mathematical::MathPoint { x: 1.0, y: 2.0 }] };
        app.dispatch_typed(MathCommand::SetPoints { geometry: geometry.clone() }, &testkit::meta("local")).expect("set points");
        assert_eq!(app.projection().expect("projection").geometry, geometry);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        let before = app.projection().expect("projection").graph.nodes.len();
        testkit::assert_undo_redo_round_trip(&mut app, node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 1.0, "y": 2.0 })), |app| app.projection().expect("projection").graph.nodes.len(), before, before + 1);
    }

    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<MathematicalPlayApp, _>("mem://mathematical-convergence", node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 9.0, "y": 9.0 })), MathCommand::SetDirected { directed: false }, |app| {
            let projection = app.projection().expect("projection");
            (projection.graph.nodes.len(), projection.graph.directed)
        });
    }

    #[test]
    fn ingest_operations_is_idempotent_for_mathematical() {
        testkit::assert_ingest_idempotent::<MathematicalPlayApp, _>(node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 3.0, "y": 4.0 })), |app| app.projection().expect("projection").graph.nodes.len());
    }
}
//#endregion 🧪️Tests
