//! ⚙️ Mathematical app — headless compute (constitutional: engine).

use mathematical::{MathGeometry, MathGraph};
use serde_json::{json, Value};

//#region 🔖GraphAlgorithms
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

pub fn workflow_json(graph: &MathGraph) -> (String, String) {
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
//#endregion 🔖Geometry

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical::MathNode;

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
        let (nodes_json, edges_json) = workflow_json(&graph);
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
}
//#endregion 🧪Tests
