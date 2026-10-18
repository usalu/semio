//! 📄️ Document IO, algorithm and geometry JSON: the math app id and equation document reset, the equation IO surface, the algorithm overlay, the workflow JSON and the geometry layers JSON. See [`super`].

use super::*;

//#region 🔖️Constants
pub const MATH_APP_ID: &str = "equation-play";

/// 🧬️ The whole-document replacement `setActiveExample` emits. The spr is built with
/// `store::os_store::text_format::empty_document_spr`, NEVER by minting an `ArtifactEnvelope` to print one: an envelope is
/// a terminal store shell whose `Drop` asserts that its app-owned bounded retirement authority
/// detached every nested owner first, and nothing on this path ever mounts or retires it — the
/// envelope route panics inside the guest (`🗒️note`/`✒️writer`/`🕸️dag` all take this route). The log
/// is edit-free by construction, which is exactly what a whole-document replace carries.
pub fn reset_equation_document_effect(document: &EquationSnapshot) -> semio_framework_plugin::Effect {
    let pack = EquationSnapshot::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr(MATH_APP_ID, MATH_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_equation_app` declares via `.artifact_kind(...)` (`computation.equation`), plus one
/// extra output port: `result:out`, the current graph+geometry projection as a generic data value
/// (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub fn equation_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: MATH_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: Some("computation.equation".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "computation.equation".into(), name: "Equation".into(), dimension: "graph".into(), component_kind: "equation".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️GraphAlgorithms
/// 🕸️ Runs the selected algorithm over the current graph and returns a per-node label suffix overlay.
pub fn algorithm_overlay(graph: &EquationGraph) -> std::collections::HashMap<String, String> {
    use semio_framework_graph_algorithms::{adjacency, bfs_distances, connected_components, strongly_connected_components, topo_sort, IdIndex};

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
                        overlay.insert(id.to_string(), dist.map_or_else(|| " ∞".into(), |d| format!(" d{d}")));
                    }
                }
            }
        }
        _ => {}
    }
    overlay
}

/// 🔌️ The one output and one input port every equation node exposes. The edges have always named them,
/// but the nodes declared no ports at all, so the node-graph engine could resolve no edge endpoint and the
/// mathematical play pane drew its four nodes without a single edge (measured live 2026-09-23).
pub const EQUATION_EDGE_SOURCE_PORT: &str = "out";

pub const EQUATION_EDGE_TARGET_PORT: &str = "in";

pub fn workflow_json(graph: &EquationGraph) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
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
                inputs: vec![NodeGraphPortRecord { id: EQUATION_EDGE_TARGET_PORT.into(), ..Default::default() }],
                outputs: vec![NodeGraphPortRecord { id: EQUATION_EDGE_SOURCE_PORT.into(), ..Default::default() }],
                ..Default::default()
            }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> =
        graph.edges.iter().map(|edge| NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id: edge.source.clone(), source_port_id: EQUATION_EDGE_SOURCE_PORT.into(), target_node_id: edge.target.clone(), target_port_id: EQUATION_EDGE_TARGET_PORT.into(), label: None }).collect();
    (nodes, edges)
}
//#endregion 🔖️GraphAlgorithms

//#region 🔖️Geometry
pub fn geometry_layers_json(geometry: &EquationGeometry) -> String {
    let points: Vec<geometry::Point> = geometry.points.iter().map(|p| geometry::Point::new(p.x, p.y)).collect();
    let hull = geometry::convex_hull(&points);
    let centroid = geometry::polygon_centroid(&hull);

    let mut layers: Vec<Value> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        layers.push(semio_framework_pack_json::object([
            ("kind".to_string(), Value::from("circle")),
            ("id".to_string(), Value::from(format!("point-{i}"))),
            ("x".to_string(), Value::from(p.x() - 5.0)),
            ("y".to_string(), Value::from(p.y() - 5.0)),
            ("width".to_string(), Value::from(10.0)),
            ("height".to_string(), Value::from(10.0)),
            ("color".to_string(), Value::from("#38bdf8")),
        ]));
    }
    if hull.len() >= 2 {
        let mut hull_points: Vec<Value> = Vec::new();
        for i in 0..hull.len() {
            let a = hull[i];
            let b = hull[(i + 1) % hull.len()];
            hull_points.push(semio_framework_pack_json::array([Value::from(a.x()), Value::from(a.y())]));
            hull_points.push(semio_framework_pack_json::array([Value::from(b.x()), Value::from(b.y())]));
        }
        layers.push(semio_framework_pack_json::object([("kind".to_string(), Value::from("polyline")), ("id".to_string(), Value::from("hull")), ("points".to_string(), semio_framework_pack_json::array(hull_points)), ("color".to_string(), Value::from("#facc15"))]));
    }
    layers.push(semio_framework_pack_json::object([
        ("kind".to_string(), Value::from("circle")),
        ("id".to_string(), Value::from("centroid")),
        ("x".to_string(), Value::from(centroid.x() - 4.0)),
        ("y".to_string(), Value::from(centroid.y() - 4.0)),
        ("width".to_string(), Value::from(8.0)),
        ("height".to_string(), Value::from(8.0)),
        ("color".to_string(), Value::from("#f472b6")),
    ]));
    semio_framework_pack_json::to_string(&semio_framework_pack_json::array(layers))
}
//#endregion 🔖️Geometry
