//! ⚙️ Mathematical app — headless compute (constitutional: engine).

use mathematical::{MathCamera, MathGeometry, MathGraph};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ui_wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord};

//#region 🔖️Config
/// 🧮️ B1: mathematical's real `DocumentApp::Config` — absorbs the former app-struct `RefCell`
/// (`MathPlayRuntime::camera`, the node-graph viewport) plus the locale the UI used to read off the
/// deleted `ViewState` — session-only view state now round-trips through the config `DocumentStore`
/// exactly like document content, with a real `backwards` per
/// `mathematical_op::MathConfigOperation` instead of never being VCS'd at all.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "mathematicalcfg")]
#[dsl(layout = "lines")]
pub struct MathConfig {
    /// 🎥️ Node-graph viewport camera — session-only, never a document field. Was `MathPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: MathCamera,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for MathConfig {
    fn default() -> Self {
        Self { camera: MathCamera::default(), locale: "en-US".into() }
    }
}

impl store::ConfigRecord for MathConfig {}

/// @emoji 🧮️ Whole-record diff for `mathematical_op::MathConfigOperation` (lives here, not in
/// `mathematical_op`, since `protocol::OperationDiff`/`MathConfig` are both foreign to that crate —
/// the orphan rule requires at least one local type). Mirrors `shooting_engine::ShootingConfig`'s
/// pattern: `apply` ignores `base` entirely.
impl protocol::OperationDiff<MathConfig> for MathConfig {
    fn apply(&self, _base: &MathConfig) -> MathConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_mathematical_app` declares via `.artifact_kind(...)` (`computation.mathematical`, newly
/// declared since mathematical had no artifact kind before, reused verbatim as this port's `kind_id`),
/// plus one extra output port: `result:out`, the current graph+geometry projection as a generic data
/// value (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub fn mathematical_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: "semio.mathematical/v1".into(),
        document_media_type: semio_framework_plugin::MediaType {
            class: semio_framework_plugin::MediaClass::Computation,
            form: semio_framework_plugin::MediaForm::Value,
        },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType {
                class: semio_framework_plugin::MediaClass::Data,
                form: semio_framework_plugin::MediaForm::Value,
            },
            kind_id: Some("computation.mathematical".into()),
            required: false,
            multiplicity: semio_framework_core::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation {
            id: "computation.mathematical".into(),
            name: "Mathematical".into(),
            dimension: "graph".into(),
            component_kind: "mathematical".into(),
        },
    }
}
//#endregion 🔖️Io

//#region 🔖️GraphAlgorithms
/// 🕸️ Runs the selected algorithm over the current graph and returns a per-node label suffix overlay.
pub fn algorithm_overlay(graph: &MathGraph) -> std::collections::HashMap<String, String> {
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
                    overlay.insert(id.to_string(), format!(" ⬤️{label}"));
                }
            }
        }
        "scc" => {
            for (group, component) in strongly_connected_components(&adj).into_iter().enumerate() {
                for i in component {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" ⬤️{group}"));
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

pub fn workflow_json(graph: &MathGraph) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let overlay = algorithm_overlay(graph);
    let nodes: Vec<NodeGraphNodeRecord> = graph
        .nodes
        .iter()
        .map(|node| {
            let suffix = overlay.get(&node.id).cloned().unwrap_or_default();
            NodeGraphNodeRecord {
                id: node.id.clone(),
                label: Some(format!("{}{}", node.label, suffix)),
                x: node.x,
                y: node.y,
                width: 72.0,
                height: 40.0,
                inputs: Vec::new(),
                outputs: Vec::new(),
                ..Default::default()
            }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = graph
        .edges
        .iter()
        .map(|edge| NodeGraphEdgeRecord {
            id: edge.id.clone(),
            source_node_id: edge.source.clone(),
            source_port_id: "out".into(),
            target_node_id: edge.target.clone(),
            target_port_id: "in".into(),
            label: None,
        })
        .collect();
    (nodes, edges)
}
//#endregion 🔖️GraphAlgorithms

//#region 🔖️Geometry
pub fn geometry_layers_json(geometry: &MathGeometry) -> String {
    let points: Vec<mathematical_geometry::Point> = geometry.points.iter().map(|p| mathematical_geometry::Point::new(p.x, p.y)).collect();
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
//#endregion 🔖️Geometry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical::MathNode;

    //#region MathConfig / mathematical_io
    #[test]
    fn math_config_default_is_the_identity_camera_and_english_locale() {
        let config = MathConfig::default();
        assert_eq!(config.camera, MathCamera::default());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn math_config_dsl_round_trips() {
        let mut config = MathConfig::default();
        config.camera = MathCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        config.locale = "de-DE".into();
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn mathematical_io_declares_result_out_with_the_computation_mathematical_kind() {
        let io = mathematical_io();
        assert_eq!(io.document_schema, "semio.mathematical/v1");
        assert_eq!(io.artifact.id, "computation.mathematical");
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "result:out");
        assert_eq!(port.kind_id.as_deref(), Some("computation.mathematical"));
        assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(port.multiplicity, semio_framework_core::PortMultiplicity::Many);
        assert!(!port.required);
    }
    //#endregion MathConfig / mathematical_io

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
    fn workflow_json_round_trips_node_count() {
        let graph = MathGraph::default();
        let (nodes, edges) = workflow_json(&graph);
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
}
//#endregion 🧪️Tests
