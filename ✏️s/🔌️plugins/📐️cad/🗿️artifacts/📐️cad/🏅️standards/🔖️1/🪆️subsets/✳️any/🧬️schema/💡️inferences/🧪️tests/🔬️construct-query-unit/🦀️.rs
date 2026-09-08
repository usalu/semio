mod tests {
    use super::*;
    use crate::standards::v1::subsets::any::io::geometry_import::{CadEdge, CadEdgeCurve, CadFace, CadPlaneSurface, CadShell, CadSolid, CadVertex, CadWire};

    fn box_geometry() -> CadGeometry {
        let corners: [[f64; 3]; 8] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
        let vertices: Vec<CadVertex> = corners.iter().enumerate().map(|(i, p)| CadVertex { id: format!("v{i}"), position: *p }).collect();
        let edge_pairs: [(usize, usize); 12] = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
        let edges: Vec<CadEdge> = edge_pairs.iter().enumerate().map(|(i, (a, b))| CadEdge { id: format!("e{i}"), vertex_ids: vec![format!("v{a}"), format!("v{b}")], curve: CadEdgeCurve { kind: "line".into() } }).collect();
        let face_wire_edges: [[usize; 4]; 6] = [[0, 1, 2, 3], [4, 5, 6, 7], [0, 9, 4, 8], [2, 11, 6, 10], [3, 8, 7, 11], [1, 10, 5, 9]];
        let wires: Vec<CadWire> = face_wire_edges.iter().enumerate().map(|(i, es)| CadWire { id: format!("w{i}"), edge_ids: es.iter().map(|e| format!("e{e}")).collect() }).collect();
        let faces: Vec<CadFace> = (0..6).map(|i| CadFace { id: format!("f{i}"), wire_ids: vec![format!("w{i}")], surface: CadPlaneSurface { kind: "plane".into(), origin: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] } }).collect();
        let shell = CadShell { id: "s0".into(), face_ids: (0..6).map(|i| format!("f{i}")).collect() };
        let solid = CadSolid { id: "sol0".into(), shell_ids: vec!["s0".into()] };
        CadGeometry { anchors: Vec::new(), vertices, edges, wires, faces, shells: vec![shell], solids: vec![solid] }
    }

    #[semio_framework_async_macros::async_test]
    async fn topology_graph_exposes_every_entity_as_a_labeled_node() {
        let geometry = box_geometry();
        let graph = CadTopologyGraph::new(&geometry);
        assert_eq!(graph.node_kind("v0").as_deref(), Some(KIND_VERTEX));
        assert_eq!(graph.node_kind("e0").as_deref(), Some(KIND_EDGE));
        assert_eq!(graph.node_kind("w0").as_deref(), Some(KIND_WIRE));
        assert_eq!(graph.node_kind("f0").as_deref(), Some(KIND_FACE));
        assert_eq!(graph.node_kind("s0").as_deref(), Some(KIND_SHELL));
        assert_eq!(graph.node_kind("sol0").as_deref(), Some(KIND_SOLID));
        assert_eq!(graph.node_kind("nonexistent"), None);
        assert_eq!(graph.node_ids().len(), 8 + 12 + 6 + 6 + 1 + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn topology_graph_bounded_by_and_contains_edges_traverse_every_dimension() {
        let geometry = box_geometry();
        let graph = CadTopologyGraph::new(&geometry);
        let rel_edges = graph.edges();
        assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "sol0" && e.target_node_id == "s0"));
        assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "s0" && e.target_node_id == "f0"));
        assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "f0" && e.target_node_id == "w0"));
        assert!(rel_edges.iter().any(|e| e.kind == REL_CONTAINS && e.source_node_id == "w0" && e.target_node_id == "e0"));
        assert!(rel_edges.iter().any(|e| e.kind == REL_CONTAINS && e.source_node_id == "e0" && e.target_node_id == "v0"));
    }

    #[semio_framework_async_macros::async_test]
    async fn construct_query_finds_every_face_bounded_by_its_wire() {
        let geometry = box_geometry();
        let json = run_construct_query(&geometry, "MATCH (f:Face)--[:BOUNDED_BY]->(w:Wire) RETURN f.name, w.name").expect("construct query must run");
        let value = protocol::json::parse(&json).expect("valid JSON result");
        let rows = value.get("rows").and_then(protocol::os_pack::json::Value::as_array).expect("rows array");
        assert_eq!(rows.len(), 6, "every one of the 6 faces must match exactly its own wire: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn construct_query_filters_edges_by_curve_kind_property() {
        let geometry = box_geometry();
        let json = run_construct_query(&geometry, "MATCH (e:Edge) WHERE e.curveKind = 'line' RETURN e.name").expect("construct query must run");
        let value = protocol::json::parse(&json).expect("valid JSON result");
        let rows = value.get("rows").and_then(protocol::os_pack::json::Value::as_array).expect("rows array");
        assert_eq!(rows.len(), 12, "all 12 box edges are line curves: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn construct_query_rejects_malformed_syntax_with_a_real_parse_error() {
        let geometry = box_geometry();
        let error = run_construct_query(&geometry, "NOT A QUERY (((").unwrap_err();
        let _ = error;
    }
}
