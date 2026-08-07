//! ⚙️ Mathematical artifact — headless compute over the `MathProjection` document (constitutional:
//! engine).
//!
//! Everything here is pure over `crate::artifacts::mathematical` types and takes no app-only view-state
//! parameter. The rule for what lands here rather than next to a single caller: a helper with MORE THAN
//! ONE consumer across the taxonomy tree lives here; a helper with exactly one consumer lives in that
//! consumer's component file. `empty_component_scene` is the example — both `🎭️modes/✏️edit/🪟️windows/*`
//! renderers need it, so it lives here rather than in either window's file.

use crate::artifacts::mathematical::{MathGeometry, MathGraph};
use semio_framework_plugin::{SurfaceKind, UiComponentSceneNode, UiPresence};
use serde_json::{json, Value};
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord};

//#region 🔖️Register
/// 🗂️ Registers `MathProjection`'s pack↔dsl codec under `MATH_DOCUMENT_SCHEMA` so `framework/sync`'s
/// folder endpoints and any other schema-string-keyed caller can print/parse mathematical documents.
/// Called from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::mathematical::MathematicalPlayApp>(crate::artifacts::mathematical::MATH_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "mathematical.document",
        extension: Some("mathematical"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::mathematical::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::mathematical::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::mathematical::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::mathematical::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("mathematical.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "mathematical.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::mathematical::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::mathematical::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::mathematical::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::mathematical::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("mathematical.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "mathematical.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::mathematical::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::mathematical::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("mathematical.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "mathematical.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::mathematical::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::mathematical::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("mathematical.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "mathematical.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::mathematical::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::mathematical::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("mathematical.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_mathematical_app` declares via `.artifact_kind(...)` (`computation.mathematical`), plus one
/// extra output port: `result:out`, the current graph+geometry projection as a generic data value
/// (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub fn mathematical_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::artifacts::mathematical::MATH_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: Some("computation.mathematical".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "computation.mathematical".into(), name: "Mathematical".into(), dimension: "graph".into(), component_kind: "mathematical".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Scene
/// 🖼️ An empty `UiComponentSceneNode` shell for a body key, ready for its `node_graph`/`canvas_2d` field
/// to be filled in — shared by both `🎭️modes/✏️edit/🪟️windows/*` renderers.
pub fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: crate::apps::mathematical::MATH_APP_ID.into(),
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
//#endregion 🔖️Scene

//#region 🔖️GraphAlgorithms
/// 🕸️ Runs the selected algorithm over the current graph and returns a per-node label suffix overlay.
pub fn algorithm_overlay(graph: &MathGraph) -> std::collections::HashMap<String, String> {
    use math::graph::algorithms::{adjacency, bfs_distances, connected_components, strongly_connected_components, topo_sort, IdIndex};

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

pub fn workflow_json(graph: &MathGraph) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let overlay = algorithm_overlay(graph);
    let nodes: Vec<NodeGraphNodeRecord> = graph
        .nodes
        .iter()
        .map(|node| {
            let suffix = overlay.get(&node.id).cloned().unwrap_or_default();
            NodeGraphNodeRecord { id: node.id.clone(), label: Some(format!("{}{}", node.label, suffix)), x: node.x, y: node.y, width: 72.0, height: 40.0, inputs: Vec::new(), outputs: Vec::new(), ..Default::default() }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> =
        graph.edges.iter().map(|edge| NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id: edge.source.clone(), source_port_id: "out".into(), target_node_id: edge.target.clone(), target_port_id: "in".into(), label: None }).collect();
    (nodes, edges)
}
//#endregion 🔖️GraphAlgorithms

//#region 🔖️Geometry
pub fn geometry_layers_json(geometry: &MathGeometry) -> String {
    let points: Vec<math::geometry::Point> = geometry.points.iter().map(|p| math::geometry::Point::new(p.x, p.y)).collect();
    let hull = math::geometry::convex_hull(&points);
    let centroid = math::geometry::polygon_centroid(&hull);

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
    use crate::artifacts::mathematical::MathNode;

    //#region MathematicalIo
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
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert!(!port.required);
    }
    //#endregion MathematicalIo

    #[test]
    fn topo_algorithm_overlay_orders_dag_nodes() {
        let graph = MathGraph::default();
        let overlay = algorithm_overlay(&graph);
        assert!(overlay.get("a").unwrap().starts_with(" #0"));
        assert!(overlay.get("d").unwrap().starts_with(" #"));
    }

    #[test]
    fn components_algorithm_overlay_groups_disconnected_node() {
        let mut graph = MathGraph { algorithm: "components".into(), ..MathGraph::default() };
        graph.nodes.push(MathNode { id: "z".into(), label: "Z".into(), x: 0.0, y: 0.0 });
        let overlay = algorithm_overlay(&graph);
        assert_ne!(overlay.get("a"), overlay.get("z"));
    }

    #[test]
    fn bfs_algorithm_overlay_reports_hop_distance() {
        let graph = MathGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..MathGraph::default() };
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
