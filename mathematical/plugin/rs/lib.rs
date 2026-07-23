//! 🧮 Combined mathematical framework playground — graph algorithms and computational geometry as one hot-swappable WASM plugin.

use semio_framework_plugin::{
    app_labels, create_default_layout, is_de_locale, localized_label_map, resolve_labels, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, AppLabelsOverlay, AppLabelsOverlayExt, Canvas2dScene, DocumentApp,
    DocumentView, NodeGraphScene, SurfaceKind, UiComponentSceneNode, UiNode, ViewState,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use vcs::{Operation, OperationDiff};

//#region 🔖Constants
const MATH_APP_ID: &str = "mathematical-play";
const MATH_WINDOW_GRAPH: &str = "math-graph";
const MATH_WINDOW_GEOMETRY: &str = "math-geometry";
const MATH_BODY_GRAPH: &str = "mathematical.play.graph";
const MATH_BODY_GEOMETRY: &str = "mathematical.play.geometry";
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathNode {
    id: String,
    label: String,
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathEdge {
    id: String,
    source: String,
    target: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathCamera {
    x: f64,
    y: f64,
    zoom: f64,
}

impl Default for MathCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🕸️ Graph playground state: quadrant toggle, retained layout, and the active algorithm overlay.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathGraph {
    directed: bool,
    nodes: Vec<MathNode>,
    edges: Vec<MathEdge>,
    camera: MathCamera,
    algorithm: String,
    #[serde(default)]
    algorithm_seed: Option<String>,
}

impl Default for MathGraph {
    fn default() -> Self {
        Self {
            directed: true,
            nodes: vec![
                MathNode { id: "a".into(), label: "A".into(), x: 40.0, y: 60.0 },
                MathNode { id: "b".into(), label: "B".into(), x: 240.0, y: 20.0 },
                MathNode { id: "c".into(), label: "C".into(), x: 240.0, y: 180.0 },
                MathNode { id: "d".into(), label: "D".into(), x: 440.0, y: 100.0 },
            ],
            edges: vec![
                MathEdge { id: "e1".into(), source: "a".into(), target: "b".into() },
                MathEdge { id: "e2".into(), source: "a".into(), target: "c".into() },
                MathEdge { id: "e3".into(), source: "b".into(), target: "d".into() },
                MathEdge { id: "e4".into(), source: "c".into(), target: "d".into() },
            ],
            camera: MathCamera::default(),
            algorithm: "topo".into(),
            algorithm_seed: None,
        }
    }
}

/// 📐 Geometry playground state: a point cloud for convex-hull/centroid demonstration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathGeometry {
    points: Vec<(f64, f64)>,
}

impl Default for MathGeometry {
    fn default() -> Self {
        Self { points: vec![(40.0, 220.0), (260.0, 40.0), (360.0, 140.0), (300.0, 260.0), (140.0, 300.0), (180.0, 160.0)] }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathProjection {
    graph: MathGraph,
    geometry: MathGeometry,
}
//#endregion 🔖Document

//#region 🔖Operation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MathDiff {
    #[serde(default)]
    graph: Option<MathGraph>,
    #[serde(default)]
    geometry: Option<MathGeometry>,
}

impl OperationDiff<MathProjection> for MathDiff {
    fn apply(&self, projection: &MathProjection) -> MathProjection {
        let mut next = projection.clone();
        if let Some(graph) = &self.graph {
            next.graph = graph.clone();
        }
        if let Some(geometry) = &self.geometry {
            next.geometry = geometry.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.graph.is_some() {
            self.graph = other.graph;
        }
        if other.geometry.is_some() {
            self.geometry = other.geometry;
        }
    }
}

/// 📤 Coarse-grained ops: each replaces one top-level projection slice; `backwards` snapshots the pre-state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
enum MathOp {
    SetGraph { graph: MathGraph },
    SetGeometry { geometry: MathGeometry },
}

impl Operation<MathProjection> for MathOp {
    type Diff = MathDiff;

    fn diff(&self, _projection: &MathProjection) -> MathDiff {
        match self {
            MathOp::SetGraph { graph } => MathDiff { graph: Some(graph.clone()), geometry: None },
            MathOp::SetGeometry { geometry } => MathDiff { graph: None, geometry: Some(geometry.clone()) },
        }
    }

    fn backwards(&self, projection: &MathProjection) -> Vec<Self> {
        match self {
            MathOp::SetGraph { .. } => vec![MathOp::SetGraph { graph: projection.graph.clone() }],
            MathOp::SetGeometry { .. } => vec![MathOp::SetGeometry { geometry: projection.geometry.clone() }],
        }
    }
}
//#endregion 🔖Operation

//#region 🔖GraphAlgorithms
/// 🕸️ Runs the selected algorithm over the current graph and returns a per-node label suffix overlay.
fn algorithm_overlay(graph: &MathGraph) -> std::collections::HashMap<String, String> {
    use mathematical_graph::algorithms::{adjacency, bfs_distances, connected_components, strongly_connected_components, topo_sort, IdIndex};

    let index = IdIndex::from_ids(graph.nodes.iter().map(|n| n.id.as_str()));
    let edge_pairs: Vec<(usize, usize)> = graph.edges.iter().filter_map(|e| Some((index.index_of(&e.source)?, index.index_of(&e.target)?))).collect();
    let adj = adjacency(index.len(), &edge_pairs, graph.directed);
    let mut overlay = std::collections::HashMap::new();

    match graph.algorithm.as_str() {
        "topo" => match topo_sort(&adj) {
            Ok(order) => {
                for (rank, &i) in order.iter().enumerate() {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" #{rank}"));
                    }
                }
            }
            Err(_) => {
                for node in &graph.nodes {
                    overlay.insert(node.id.clone(), " ⟲".into());
                }
            }
        },
        "components" => {
            for (i, label) in connected_components(&adj).into_iter().enumerate() {
                if let Some(id) = index.id_of(i) {
                    overlay.insert(id.to_string(), format!(" ⬤{label}"));
                }
            }
        }
        "scc" => {
            for (group, component) in strongly_connected_components(&adj).into_iter().enumerate() {
                for i in component {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" ⬤{group}"));
                    }
                }
            }
        }
        "bfs" => {
            if let Some(seed) = graph.algorithm_seed.as_deref().and_then(|s| index.index_of(s)) {
                for (i, dist) in bfs_distances(&adj, seed).into_iter().enumerate() {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), dist.map(|d| format!(" d{d}")).unwrap_or_else(|| " ∞".into()));
                    }
                }
            }
        }
        _ => {}
    }
    overlay
}

fn media_graph_json(graph: &MathGraph) -> (String, String) {
    let overlay = algorithm_overlay(graph);
    let nodes: Vec<Value> = graph
        .nodes
        .iter()
        .map(|node| {
            let suffix = overlay.get(&node.id).cloned().unwrap_or_default();
            json!({
                "id": node.id,
                "label": format!("{}{}", node.label, suffix),
                "x": node.x,
                "y": node.y,
                "width": 72.0,
                "height": 40.0,
                "inputs": [],
                "outputs": [],
            })
        })
        .collect();
    let edges: Vec<Value> = graph
        .edges
        .iter()
        .map(|edge| {
            json!({
                "id": edge.id,
                "sourceNodeId": edge.source,
                "sourcePortId": "out",
                "targetNodeId": edge.target,
                "targetPortId": "in",
            })
        })
        .collect();
    (serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()))
}
//#endregion 🔖GraphAlgorithms

//#region 🔖Geometry
fn geometry_layers_json(geometry: &MathGeometry) -> String {
    let points: Vec<mathematical_geometry::Point> = geometry.points.iter().map(|&(x, y)| mathematical_geometry::Point::new(x, y)).collect();
    let hull = mathematical_geometry::convex_hull(&points);
    let centroid = mathematical_geometry::polygon_centroid(&hull);

    let mut layers: Vec<Value> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        layers.push(json!({ "kind": "circle", "id": format!("point-{i}"), "x": p.x() - 5.0, "y": p.y() - 5.0, "width": 10.0, "height": 10.0, "color": "#38bdf8" }));
    }
    if hull.len() >= 2 {
        let mut hull_points: Vec<[f64; 2]> = Vec::new();
        for i in 0..hull.len() {
            let a = hull[i];
            let b = hull[(i + 1) % hull.len()];
            hull_points.push([a.x(), a.y()]);
            hull_points.push([b.x(), b.y()]);
        }
        layers.push(json!({ "kind": "polyline", "id": "hull", "points": hull_points, "color": "#facc15" }));
    }
    layers.push(json!({ "kind": "circle", "id": "centroid", "x": centroid.x() - 4.0, "y": centroid.y() - 4.0, "width": 8.0, "height": 8.0, "color": "#f472b6" }));
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖Geometry

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the mathematical app; one field per label makes every locale combination compile-checked.
/// 🧮 Graph/node/geometry vocabulary here is pure math terminology, not building-assembly terminology, so no reuse variant applies.
app_labels! {
    struct MathematicalLabels {
        window_graph: &'static str = en: "Graph", de: "Graph";
        window_geometry: &'static str = en: "Geometry", de: "Geometrie";
        mode_edit: &'static str = en: "Edit", de: "Bearbeiten";
        example_demo: &'static str = en: "Demo", de: "Demo";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
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
//#endregion 🔖CommandLabels

//#region 🔖Render
fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: MATH_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
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
    }
}

fn render_graph_window(graph: &MathGraph) -> UiNode {
    let (nodes_json, edges_json) = media_graph_json(graph);
    let viewport_json = serde_json::to_string(&graph.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let mut scene = empty_component_scene(MATH_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes_json, edges_json, viewport_json) });
    UiNode::ComponentScene(scene)
}

fn render_geometry_window(geometry: &MathGeometry) -> UiNode {
    let mut scene = empty_component_scene(MATH_BODY_GEOMETRY, SurfaceKind::Canvas2d);
    scene.canvas_2d = Some(Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json: geometry_layers_json(geometry) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖Render

//#region 🔖MathematicalPlayApp
#[derive(Default)]
struct MathematicalPlayApp;

impl DocumentApp for MathematicalPlayApp {
    type Projection = MathProjection;
    type Op = MathOp;

    fn app_id(&self) -> &str {
        MATH_APP_ID
    }

    fn document_schema(&self) -> &str {
        "semio.mathematical/v1"
    }

    fn initial_projection(&self) -> MathProjection {
        MathProjection::default()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, MathProjection>, _view_state: &ViewState) -> ActionEmit<MathOp> {
        match action {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")).and_then(|value| serde_json::from_value::<MathProjection>(value.clone()).ok()) {
                    let mut ops = Vec::new();
                    if next.graph != doc.projection.graph {
                        ops.push(MathOp::SetGraph { graph: next.graph });
                    }
                    if next.geometry != doc.projection.geometry {
                        ops.push(MathOp::SetGeometry { geometry: next.geometry });
                    }
                    return ActionEmit::ops(ops);
                }
            }
            "setAlgorithm" => {
                if let Some(algorithm) = args.and_then(|value| value.get("algorithm")).and_then(Value::as_str) {
                    let mut graph = doc.projection.graph.clone();
                    graph.algorithm = algorithm.to_string();
                    graph.algorithm_seed = args.and_then(|value| value.get("seed")).and_then(Value::as_str).map(str::to_string);
                    return ActionEmit::commit(vec![MathOp::SetGraph { graph }], "setAlgorithm");
                }
            }
            "setDirected" => {
                if let Some(directed) = args.and_then(|value| value.get("directed")).and_then(Value::as_bool) {
                    let mut graph = doc.projection.graph.clone();
                    graph.directed = directed;
                    return ActionEmit::ops(vec![MathOp::SetGraph { graph }]);
                }
            }
            "nodeGraphEdit" => {
                let edit_ops = args.and_then(|value| value.get("ops")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let mut graph = doc.projection.graph.clone();
                let mut changed = false;
                for op in edit_ops {
                    match op.get("op").and_then(Value::as_str).unwrap_or("") {
                        "addNode" => {
                            let x = op.get("x").and_then(Value::as_f64).unwrap_or(0.0);
                            let y = op.get("y").and_then(Value::as_f64).unwrap_or(0.0);
                            let id = format!("n{}", graph.nodes.len());
                            graph.nodes.push(MathNode { label: id.to_uppercase(), id, x, y });
                            changed = true;
                        }
                        "connect" => {
                            if let (Some(source), Some(target)) = (op.get("sourceNodeId").and_then(Value::as_str), op.get("targetNodeId").and_then(Value::as_str)) {
                                let id = format!("e{}", graph.edges.len());
                                graph.edges.push(MathEdge { id, source: source.into(), target: target.into() });
                                changed = true;
                            }
                        }
                        "deleteSelection" => {
                            if let Some(ids) = op.get("nodeIds").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                                graph.nodes.retain(|node| !ids.contains(&node.id));
                                graph.edges.retain(|edge| !ids.contains(&edge.source) && !ids.contains(&edge.target));
                                changed = true;
                            }
                        }
                        _ => {}
                    }
                }
                if changed {
                    return ActionEmit::ops(vec![MathOp::SetGraph { graph }]);
                }
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(Value::as_str) {
                    if let Ok(camera) = serde_json::from_str::<MathCamera>(viewport_json) {
                        let mut graph = doc.projection.graph.clone();
                        graph.camera = camera;
                        return ActionEmit::amend(vec![MathOp::SetGraph { graph }], "viewport");
                    }
                }
            }
            "setPoints" => {
                if let Some(points) = args.and_then(|value| value.get("points")).and_then(|value| serde_json::from_value::<Vec<(f64, f64)>>(value.clone()).ok()) {
                    return ActionEmit::ops(vec![MathOp::SetGeometry { geometry: MathGeometry { points } }]);
                }
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, MathProjection>, _view_state: &ViewState) -> UiNode {
        match body_key {
            MATH_BODY_GRAPH => render_graph_window(&doc.projection.graph),
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
//#endregion 🔖MathematicalPlayApp

//#region 🔖Manifest
fn create_mathematical_app() -> App {
    App::from_builder(
        App::builder(MATH_APP_ID, "Mathematical")
            .document(["semio", "mathematical"])
            .icon_id("sigma")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(MATH_WINDOW_GRAPH, "Graph", MATH_BODY_GRAPH, SurfaceKind::NodeGraph)
            .window_kind(MATH_WINDOW_GEOMETRY, "Geometry", MATH_BODY_GEOMETRY, SurfaceKind::Canvas2d)
            .default_layout(create_default_layout(&[MATH_WINDOW_GRAPH.into(), MATH_WINDOW_GEOMETRY.into()], "row", Some(&[60.0, 40.0]), Some(&["Graph".into(), "Geometry".into()])))
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setDocument", "Set Document", ActionKind::Operation) })
            .operation("setAlgorithm", "Set Algorithm")
            .operation("setDirected", "Set Directed")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .operation("nodeGraphViewport", "Node Graph Viewport")
            .operation("setPoints", "Set Points")
            // 📝 Staged argument forms for the graph analysis controls.
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
    .example("demo", "Demo", serde_json::to_string(&MathProjection::default()).unwrap())
    .program("mathematical", "Mathematical", "graph")
}

fn register_mathematical_exports() {}

semio_framework_plugin::semio_plugin! {
    id: "mathematical", label: "Mathematical", version: "0.1.0",
    setup: register_mathematical_exports,
    apps: [ create_mathematical_app => MathematicalPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topo_algorithm_overlay_orders_dag_nodes() {
        let graph = MathGraph::default();
        let overlay = algorithm_overlay(&graph);
        assert!(overlay.get("a").unwrap().starts_with(" #0"));
        assert!(overlay.get("d").unwrap().starts_with(" #"));
    }

    #[test]
    fn components_algorithm_overlay_groups_disconnected_node() {
        let mut graph = MathGraph::default();
        graph.algorithm = "components".into();
        graph.nodes.push(MathNode { id: "z".into(), label: "Z".into(), x: 0.0, y: 0.0 });
        let overlay = algorithm_overlay(&graph);
        assert_ne!(overlay.get("a"), overlay.get("z"));
    }

    #[test]
    fn bfs_algorithm_overlay_reports_hop_distance() {
        let mut graph = MathGraph::default();
        graph.algorithm = "bfs".into();
        graph.algorithm_seed = Some("a".into());
        let overlay = algorithm_overlay(&graph);
        assert_eq!(overlay.get("a").unwrap(), " d0");
        assert_eq!(overlay.get("b").unwrap(), " d1");
    }

    #[test]
    fn media_graph_json_round_trips_node_count() {
        let graph = MathGraph::default();
        let (nodes_json, edges_json) = media_graph_json(&graph);
        let nodes: Vec<Value> = serde_json::from_str(&nodes_json).unwrap();
        let edges: Vec<Value> = serde_json::from_str(&edges_json).unwrap();
        assert_eq!(nodes.len(), graph.nodes.len());
        assert_eq!(edges.len(), graph.edges.len());
    }

    #[test]
    fn geometry_layers_include_hull_and_centroid() {
        let geometry = MathGeometry::default();
        let layers_json = geometry_layers_json(&geometry);
        assert!(layers_json.contains("\"hull\""));
        assert!(layers_json.contains("\"centroid\""));
    }

    #[test]
    fn renders_node_graph_scene() {
        let app = MathematicalPlayApp;
        let projection = MathProjection::default();
        let history = semio_framework_plugin::HistoryView { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, current_checkpoint_id: None };
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(MATH_BODY_GRAPH, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_canvas_2d_scene() {
        let app = MathematicalPlayApp;
        let projection = MathProjection::default();
        let history = semio_framework_plugin::HistoryView { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, current_checkpoint_id: None };
        let doc = DocumentView { projection: &projection, history: &history };
        let node = app.render(MATH_BODY_GEOMETRY, &doc, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canvas-2d"));
    }
}
//#endregion 🧪Tests
