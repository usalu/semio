//! ↔️ Normal undirected graph: node-to-node edges without port ordering (WIRES, mindmaps).

pub mod fixture_layout;

pub use fixture_layout::{
    apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value,
    apply_force_graph_layout_to_fixture_v1_value_resolved, apply_redraw_layout_to_fixture_v1_json, resolve_node_id_endpoint,
    ForceGraphLayoutOptions,
};
pub use mathematical_graph::*;
pub use mathematical_core::{Directedness, Normal, Undirected};

/// ↔️ Node graph engine without ports; endpoints are unordered node pairs.
pub type UndirectedGraphEngine = GraphEngine<Normal, Undirected>;

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undirected_graph_engine_alias() {
        let mut g = UndirectedGraphEngine::new();
        g.create_node(1, 0.0, 0.0, 20.0, true);
        g.create_node(2, 50.0, 0.0, 20.0, true);
        g.create_edge(9, 2, 1);
        let e = g.edges.get(&9).unwrap();
        assert_eq!(e.source, 1);
        assert_eq!(e.target, 2);
    }

    #[test]
    fn force_graph_without_gravity_stays_near_initial_centroid() {
        let fixture = serde_json::json!({
            "schema": "reasoning.mindmap.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 1000.0, "y": 1000.0, "radius": 40.0 },
                { "id": "b", "x": 1001.0, "y": 1000.0, "radius": 40.0 }
            ],
            "edges": [{ "id": "e1", "source": "a", "target": "b" }]
        });
        let opts = serde_json::json!({
            "iterations": 200,
            "idealEdgeLength": 180.0,
            "repulsionStrength": 0.0,
            "springStrength": 0.04,
            "gravity": 0.0,
            "centerX": 0.0,
            "centerY": 0.0,
            "randomSeed": 7
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        let mean_x = (ax + bx) * 0.5;
        assert!(mean_x > 900.0, "expected layout to stay near initial centroid, not creep to world origin, mean_x={mean_x}");
    }

    #[test]
    fn force_graph_node_id_edges_apply_spring_forces() {
        let fixture = serde_json::json!({
            "schema": "reasoning.mindmap.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 40.0 },
                { "id": "b", "x": 1.0, "y": 0.0, "radius": 40.0 }
            ],
            "edges": [{ "id": "e1", "source": "a", "target": "b" }]
        });
        let opts = serde_json::json!({
            "iterations": 200,
            "idealEdgeLength": 180.0,
            "repulsionStrength": 0.0,
            "springStrength": 0.04,
            "gravity": 0.0,
            "randomSeed": 7
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected undirected springs to spread nodes, got a={ax} b={bx}");
    }
}
// #endregion 🔖Tests
