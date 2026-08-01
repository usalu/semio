//! 🧮️ Mathematical app — DocumentApp impl, render, manifest (constitutional: ui).

use mathematical::{MathCamera, MathEdge, MathGeometry, MathGraph, MathNode, MathPoint, MathProjection};
use mathematical_engine::{geometry_layers_json, workflow_json};
use mathematical_op::MathOperation;
use semio_framework_plugin::{
    app_labels, create_default_layout, is_de_locale, localized_label_map, resolve_labels, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, AppLabelsOverlay, AppLabelsOverlayExt, Canvas2dScene, DocumentApp,
    DocumentView, NodeGraphScene, SurfaceKind, UiComponentSceneNode, UiNode, UiPresence, ViewState,
};
use serde_json::Value;
use store::DocumentDsl;

//#region 🔖️Constants
const MATH_APP_ID: &str = "mathematical-play";
const MATH_WINDOW_GRAPH: &str = "math-graph";
const MATH_WINDOW_GEOMETRY: &str = "math-geometry";
const MATH_BODY_GRAPH: &str = "mathematical.play.graph";
const MATH_BODY_GEOMETRY: &str = "mathematical.play.geometry";
//#endregion 🔖️Constants

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the mathematical app; one field per label makes every locale combination compile-checked.
/// 🧮️ Graph/node/geometry vocabulary here is pure math terminology, not building-assembly terminology, so no reuse variant applies.
app_labels! {
    struct MathematicalLabels {
        window_graph: &'static str = en: "Graph", de: "Graph";
        window_geometry: &'static str = en: "Geometry", de: "Geometrie";
        mode_edit: &'static str = en: "Edit", de: "Bearbeiten";
        example_demo: &'static str = en: "Demo", de: "Demo";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation declared in `create_mathematical_app`'s static manifest —
/// the manifest itself has no `view_state`/locale parameter, so this overlay is how the command palette and Actions
/// rail get a translated label without threading locale through the whole builder chain.
fn mathematical_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    localized_label_map(
        is_de,
        &[
            ("setDocument", "Set Document", "Dokument festlegen"),
            ("setAlgorithm", "Set Algorithm", "Algorithmus festlegen"),
            ("setDirected", "Set Directed", "Gerichtet festlegen"),
            ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
            ("nodeGraphViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
            ("setPoints", "Set Points", "Punkte festlegen"),
        ],
    )
}
//#endregion 🔖️CommandLabels

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
    },
    menu: None,
}

fn render_graph_window(graph: &MathGraph, camera: &MathCamera) -> UiNode {
    let (nodes_json, edges_json) = workflow_json(graph);
    let viewport_json = serde_json::to_string(camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let mut scene = empty_component_scene(MATH_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes_json, edges_json, viewport_json) });
    UiNode::ComponentScene(scene)
}

fn render_geometry_window(geometry: &MathGeometry) -> UiNode {
    let mut scene = empty_component_scene(MATH_BODY_GEOMETRY, SurfaceKind::Canvas2d);
    scene.canvas_2d = Some(Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: geometry_layers_json(geometry) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️Render

//#region 🔖️MathematicalPlayApp
/// 🎛️ Ephemeral view state (node-graph camera) — lives in the app struct, not the document, so it
/// stays out of undo history and off the operation channel.
#[derive(Clone, Debug, Default, PartialEq)]
struct MathPlayRuntime {
    camera: MathCamera,
}

#[derive(Default)]
pub struct MathematicalPlayApp {
    runtime: MathPlayRuntime,
}

impl DocumentApp for MathematicalPlayApp {
    type Projection = MathProjection;
    type Operation = MathOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        MATH_APP_ID
    }

    fn document_schema(&self) -> &str {
        "semio.mathematical/v1"
    }

    fn initial_projection(&self) -> MathProjection {
        MathProjection::default()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, MathProjection>, _view_state: &ViewState) -> ActionEmit<MathOperation> {
        match action {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")).and_then(|value| serde_json::from_value::<MathProjection>(value.clone()).ok()) {
                    let mut operations = Vec::new();
                    if next.graph != doc.projection.graph {
                        operations.push(MathOperation::SetGraph { graph: next.graph });
                    }
                    if next.geometry != doc.projection.geometry {
                        operations.push(MathOperation::SetGeometry { geometry: next.geometry });
                    }
                    return ActionEmit::operations(operations);
                }
            }
            "setAlgorithm" => {
                if let Some(algorithm) = args.and_then(|value| value.get("algorithm")).and_then(Value::as_str) {
                    let mut graph = doc.projection.graph.clone();
                    graph.algorithm = algorithm.to_string();
                    graph.algorithm_seed = args.and_then(|value| value.get("seed")).and_then(Value::as_str).map(str::to_string);
                    return ActionEmit::commit(vec![MathOperation::SetGraph { graph }], "setAlgorithm");
                }
            }
            "setDirected" => {
                if let Some(directed) = args.and_then(|value| value.get("directed")).and_then(Value::as_bool) {
                    let mut graph = doc.projection.graph.clone();
                    graph.directed = directed;
                    return ActionEmit::operations(vec![MathOperation::SetGraph { graph }]);
                }
            }
            "nodeGraphEdit" => {
                let edit_operations = args.and_then(|value| value.get("operations")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let mut graph = doc.projection.graph.clone();
                let mut changed = false;
                for operation in edit_operations {
                    match operation.get("operation").and_then(Value::as_str).unwrap_or("") {
                        "addNode" => {
                            let x = operation.get("x").and_then(Value::as_f64).unwrap_or(0.0);
                            let y = operation.get("y").and_then(Value::as_f64).unwrap_or(0.0);
                            let id = format!("n{}", graph.nodes.len());
                            graph.nodes.push(MathNode { label: id.to_uppercase(), id, x, y });
                            changed = true;
                        }
                        "connect" => {
                            if let (Some(source), Some(target)) = (operation.get("sourceNodeId").and_then(Value::as_str), operation.get("targetNodeId").and_then(Value::as_str)) {
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
                    return ActionEmit::operations(vec![MathOperation::SetGraph { graph }]);
                }
            }
            // 👁️ View action: the node-graph viewport never touches the document — it's written
            // straight into `self.runtime`, session-only, no VCS edit, no undo entry.
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(Value::as_str) {
                    if let Ok(camera) = serde_json::from_str::<MathCamera>(viewport_json) {
                        self.runtime.camera = camera;
                    }
                }
            }
            "setPoints" => {
                if let Some(points) = args.and_then(|value| value.get("points")).and_then(|value| serde_json::from_value::<Vec<MathPoint>>(value.clone()).ok()) {
                    return ActionEmit::operations(vec![MathOperation::SetGeometry { geometry: MathGeometry { points } }]);
                }
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, MathProjection>, _view_state: &ViewState) -> UiNode {
        match body_key {
            MATH_BODY_GRAPH => render_graph_window(&doc.projection.graph, &self.runtime.camera),
            MATH_BODY_GEOMETRY => render_geometry_window(&doc.projection.geometry),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<MathematicalLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(MATH_WINDOW_GRAPH, labels.window_graph)
            .window_kind_label(MATH_WINDOW_GEOMETRY, labels.window_geometry)
            .mode_label("edit", labels.mode_edit)
            .action_labels(mathematical_action_labels(is_de))
            .example_labels(std::collections::HashMap::from([("demo".to_string(), labels.example_demo.to_string())]))
    }
}
//#endregion 🔖️MathematicalPlayApp

//#region 🔖️Manifest
pub fn create_mathematical_app() -> App {
    App::from_builder(
        App::builder(MATH_APP_ID, "Mathematical")
            .document(["semio", "mathematical"])
            .icon_id("math-app")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(MATH_WINDOW_GRAPH, "Graph", MATH_BODY_GRAPH, SurfaceKind::NodeGraph, "math-graph")
            .window_kind(MATH_WINDOW_GEOMETRY, "Geometry", MATH_BODY_GEOMETRY, SurfaceKind::Canvas2d, "hexagon")
            .default_layout(create_default_layout(&[MATH_WINDOW_GRAPH.into(), MATH_WINDOW_GEOMETRY.into()], "row", Some(&[60.0, 40.0]), Some(&["Graph".into(), "Geometry".into()])))
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setDocument", "Set Document", ActionKind::Operation) })
            .operation("setAlgorithm", "Set Algorithm")
            .operation("setDirected", "Set Directed")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .view_action("nodeGraphViewport", "Node Graph Viewport")
            .operation("setPoints", "Set Points")
            // 📝️ Staged argument forms for the graph analysis controls.
            .action_args("setAlgorithm", vec![
                ActionArgDef::select("algorithm", "Algorithm", vec![
                    ActionArgOption::new("topo", "Topological Order"),
                    ActionArgOption::new("components", "Connected Components"),
                    ActionArgOption::new("scc", "Strongly Connected Components"),
                    ActionArgOption::new("bfs", "Breadth-First Distances"),
                ]).required(),
            ])
            .action_args("setDirected", vec![
                ActionArgDef::toggle("directed", "Directed").default_value(true),
            ]),
    )
    .example("demo", "Demo", MathProjection::default().print_dsl())
    .workflow("mathematical", "Mathematical", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_node_graph_scene() {
        let app = MathematicalPlayApp::default();
        let projection = MathProjection::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(MATH_BODY_GRAPH, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_canvas_2d_scene() {
        let app = MathematicalPlayApp::default();
        let projection = MathProjection::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(MATH_BODY_GEOMETRY, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }

    /// 🎥️ `"nodeGraphViewport"` is a View action — it must never emit a `MathOperation` (no VCS edit,
    /// no undo entry) and instead write straight into `self.runtime`.
    #[test]
    fn node_graph_viewport_writes_runtime_not_operations() {
        let mut app = MathematicalPlayApp::default();
        let projection = MathProjection::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };
        let emit = app.handle_action("nodeGraphViewport", Some(&serde_json::json!({ "viewportJson": r#"{"x":5.0,"y":6.0,"zoom":2.0}"# })), &doc, &ViewState::default());
        assert!(emit.operations.is_empty(), "nodeGraphViewport must not emit a VCS operation");
        assert_eq!(app.runtime.camera.zoom, 2.0);
        assert_eq!(app.runtime.camera.x, 5.0);
        let node = app.render(MATH_BODY_GRAPH, &doc, &ViewState::default());
        let payload: Value = serde_json::to_value(&node).unwrap();
        let viewport: Value = serde_json::from_str(payload["nodeGraph"]["viewportJson"].as_str().unwrap()).unwrap();
        assert_eq!(viewport["zoom"], serde_json::json!(2.0));
    }
}
//#endregion 🧪️Tests
