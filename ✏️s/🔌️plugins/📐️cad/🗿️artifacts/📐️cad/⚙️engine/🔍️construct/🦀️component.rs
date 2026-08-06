//! 🕸️ "Construct" — CAD's topology query capability — is NOT a new parser: it is Jack
//! (`mathematical_graph_dsl`, an already-complete Cypher-like language with its own parser,
//! executor, formatter, and LSP) applied to brep topology through this `QueryableGraph`
//! implementation. Every editable entity (`CadVertex`/`CadEdge`/`CadWire`/`CadFace`/`CadShell`/
//! `CadSolid`) becomes a Jack node labeled by its kind (`"Vertex"`/`"Edge"`/.../`"Solid"`);
//! `[:BOUNDED_BY]` moves down one topological dimension (Solid->Shell, Shell->Face, Face->Wire),
//! `[:CONTAINS]` reaches the entities that directly compose a boundary member (Wire->Edge,
//! Edge->Vertex) — exactly the relationship vocabulary `.🦑️repo/✍️notes/construct.md`'s TopoCypher
//! design calls for. A query like `MATCH (f:Face)-[:BOUNDED_BY]->(w:Wire)-[:CONTAINS]->(e:Edge)`
//! runs against this today via `mathematical_graph_dsl::run_query`, with zero new grammar.

use crate::artifacts::cad::CadGeometry;
use mathematical_graph_dsl::{QueryableEdge, QueryableGraph};
use mathematical_graph_manifest::PropertyValue;
use std::collections::BTreeSet;

/// @emoji 🏷️ The Jack node-label vocabulary for brep entities — mirrors TopoCypher's
/// `(:Vertex) (:Edge) (:Wire) (:Face) (:Shell) (:Solid)` (the "Cell"/"CellComplex"/"Cluster"
/// labels from `construct.md` have no `CadGeometry` equivalent yet, so they're omitted rather
/// than faked).
const KIND_VERTEX: &str = "Vertex";
const KIND_EDGE: &str = "Edge";
const KIND_WIRE: &str = "Wire";
const KIND_FACE: &str = "Face";
const KIND_SHELL: &str = "Shell";
const KIND_SOLID: &str = "Solid";

const REL_BOUNDED_BY: &str = "BOUNDED_BY";
const REL_CONTAINS: &str = "CONTAINS";

/// @emoji 🕸️ One `CadGeometry` pane (e.g. `scene.shape_geometry`), exposed as a Jack
/// `QueryableGraph` — read-only, matching `construct.md`'s explicit constraint that direct
/// graph mutation is unsafe for a B-rep and must go through a validated command layer instead.
pub struct CadTopologyGraph<'a> {
    geometry: &'a CadGeometry,
}

impl<'a> CadTopologyGraph<'a> {
    pub fn new(geometry: &'a CadGeometry) -> Self {
        Self { geometry }
    }
}

impl QueryableGraph for CadTopologyGraph<'_> {
    fn manifest(&self) -> Option<&mathematical_graph_manifest::GraphManifest> {
        // No compile-time schema for a dynamically-shaped brep pane — every query resolves
        // purely against `node_kind`/`node_property`, matching `EmptyGraph`'s precedent in
        // `mathematical_graph_dsl`'s own idiom-hooks completion path.
        None
    }

    fn node_ids(&self) -> Vec<String> {
        let g = self.geometry;
        g.vertices
            .iter()
            .map(|v| v.id.clone())
            .chain(g.edges.iter().map(|e| e.id.clone()))
            .chain(g.wires.iter().map(|w| w.id.clone()))
            .chain(g.faces.iter().map(|f| f.id.clone()))
            .chain(g.shells.iter().map(|s| s.id.clone()))
            .chain(g.solids.iter().map(|s| s.id.clone()))
            .collect()
    }

    fn node_kind(&self, id: &str) -> Option<String> {
        let g = self.geometry;
        if g.vertices.iter().any(|v| v.id == id) {
            return Some(KIND_VERTEX.to_string());
        }
        if g.edges.iter().any(|e| e.id == id) {
            return Some(KIND_EDGE.to_string());
        }
        if g.wires.iter().any(|w| w.id == id) {
            return Some(KIND_WIRE.to_string());
        }
        if g.faces.iter().any(|f| f.id == id) {
            return Some(KIND_FACE.to_string());
        }
        if g.shells.iter().any(|s| s.id == id) {
            return Some(KIND_SHELL.to_string());
        }
        if g.solids.iter().any(|s| s.id == id) {
            return Some(KIND_SOLID.to_string());
        }
        None
    }

    fn node_name(&self, id: &str) -> Option<String> {
        // Brep entities have no separate display name distinct from their id.
        self.node_kind(id).map(|_| id.to_string())
    }

    fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
        let g = self.geometry;
        match key {
            "position" => g.vertices.iter().find(|v| v.id == id).map(|v| PropertyValue::Array(v.position.iter().map(|c| PropertyValue::Number(*c)).collect())),
            "curveKind" => g.edges.iter().find(|e| e.id == id).map(|e| PropertyValue::String(e.curve.kind.clone())),
            "surfaceKind" => g.faces.iter().find(|f| f.id == id).map(|f| PropertyValue::String(f.surface.kind.clone())),
            "normal" => g.faces.iter().find(|f| f.id == id).map(|f| PropertyValue::Array(f.surface.normal.iter().map(|c| PropertyValue::Number(*c)).collect())),
            _ => None,
        }
    }

    fn edges(&self) -> Vec<QueryableEdge> {
        let g = self.geometry;
        let mut out = Vec::new();
        let mut next_id = 0usize;
        let mut push = |kind: &str, source_node_id: String, target_node_id: String| {
            next_id += 1;
            out.push(QueryableEdge { id: format!("{kind}-{next_id}"), kind: kind.to_string(), source_node_id, target_node_id, source_port: None, target_port: None, properties: mathematical_graph_manifest::PropertyBag::default() });
        };
        for solid in &g.solids {
            for shell_id in &solid.shell_ids {
                push(REL_BOUNDED_BY, solid.id.clone(), shell_id.clone());
            }
        }
        for shell in &g.shells {
            for face_id in &shell.face_ids {
                push(REL_BOUNDED_BY, shell.id.clone(), face_id.clone());
            }
        }
        for face in &g.faces {
            for wire_id in &face.wire_ids {
                push(REL_BOUNDED_BY, face.id.clone(), wire_id.clone());
            }
        }
        for wire in &g.wires {
            for edge_id in &wire.edge_ids {
                push(REL_CONTAINS, wire.id.clone(), edge_id.clone());
            }
        }
        for edge in &g.edges {
            for vertex_id in &edge.vertex_ids {
                push(REL_CONTAINS, edge.id.clone(), vertex_id.clone());
            }
        }
        out
    }

    fn subgraph_fixture_json(&self, _node_ids: &BTreeSet<String>, _edge_ids: &BTreeSet<String>) -> Option<String> {
        // Not needed for querying — this graph is read directly off `CadGeometry`, never
        // round-tripped through Jack's own fixture JSON format.
        None
    }
}

/// @emoji 🔍️ Runs a Jack query against one `CadGeometry` pane and returns its JSON result —
/// the single entry point `cad-ui`/an MCP tool calls for topology queries (`saved selections`,
/// non-manifold-edge checks, adjacency lookups), reusing `mathematical_graph_dsl::run_query_json`
/// unchanged.
pub fn run_construct_query(geometry: &CadGeometry, source: &str) -> Result<String, mathematical_graph_dsl::GraphDslError> {
    let graph = CadTopologyGraph::new(geometry);
    mathematical_graph_dsl::run_query_json(&graph, source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::{CadEdge, CadEdgeCurve, CadFace, CadPlaneSurface, CadShell, CadSolid, CadVertex, CadWire};

    /// 📦️ A closed box: 8 vertices, 12 edges, 6 wires, 6 faces, 1 shell, 1 solid — enough
    /// topology to exercise BOUNDED_BY/CONTAINS traversal across every dimension.
    fn box_geometry() -> CadGeometry {
        let corners: [[f64; 3]; 8] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
        let vertices: Vec<CadVertex> = corners.iter().enumerate().map(|(i, p)| CadVertex { id: format!("v{i}"), position: *p }).collect();
        let edge_pairs: [(usize, usize); 12] = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0), // bottom
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4), // top
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7), // verticals
        ];
        let edges: Vec<CadEdge> = edge_pairs.iter().enumerate().map(|(i, (a, b))| CadEdge { id: format!("e{i}"), vertex_ids: vec![format!("v{a}"), format!("v{b}")], curve: CadEdgeCurve { kind: "line".into() } }).collect();
        let face_wire_edges: [[usize; 4]; 6] = [
            [0, 1, 2, 3],   // bottom
            [4, 5, 6, 7],   // top
            [0, 9, 4, 8],   // front
            [2, 11, 6, 10], // back
            [3, 8, 7, 11],  // left
            [1, 10, 5, 9],  // right
        ];
        let wires: Vec<CadWire> = face_wire_edges.iter().enumerate().map(|(i, es)| CadWire { id: format!("w{i}"), edge_ids: es.iter().map(|e| format!("e{e}")).collect() }).collect();
        let faces: Vec<CadFace> = (0..6).map(|i| CadFace { id: format!("f{i}"), wire_ids: vec![format!("w{i}")], surface: CadPlaneSurface { kind: "plane".into(), origin: [0.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0] } }).collect();
        let shell = CadShell { id: "s0".into(), face_ids: (0..6).map(|i| format!("f{i}")).collect() };
        let solid = CadSolid { id: "sol0".into(), shell_ids: vec!["s0".into()] };
        CadGeometry { anchors: Vec::new(), vertices, edges, wires, faces, shells: vec![shell], solids: vec![solid] }
    }

    #[test]
    fn topology_graph_exposes_every_entity_as_a_labeled_node() {
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

    #[test]
    fn topology_graph_bounded_by_and_contains_edges_traverse_every_dimension() {
        let geometry = box_geometry();
        let graph = CadTopologyGraph::new(&geometry);
        let rel_edges = graph.edges();
        // Solid -[:BOUNDED_BY]-> Shell -[:BOUNDED_BY]-> Face -[:BOUNDED_BY]-> Wire -[:CONTAINS]-> Edge -[:CONTAINS]-> Vertex
        assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "sol0" && e.target_node_id == "s0"));
        assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "s0" && e.target_node_id == "f0"));
        assert!(rel_edges.iter().any(|e| e.kind == REL_BOUNDED_BY && e.source_node_id == "f0" && e.target_node_id == "w0"));
        assert!(rel_edges.iter().any(|e| e.kind == REL_CONTAINS && e.source_node_id == "w0" && e.target_node_id == "e0"));
        assert!(rel_edges.iter().any(|e| e.kind == REL_CONTAINS && e.source_node_id == "e0" && e.target_node_id == "v0"));
    }

    /// 🕸️ Runs a REAL Jack query — `MATCH (f:Face)-[:BOUNDED_BY]->(w:Wire) RETURN f.name, w.name`
    /// — against `CadTopologyGraph`, proving Jack's existing parser/executor answers a genuine
    /// TopoCypher-shaped question with zero new grammar, exactly as `construct.md` envisioned.
    #[test]
    fn construct_query_finds_every_face_bounded_by_its_wire() {
        let geometry = box_geometry();
        let json = run_construct_query(&geometry, "MATCH (f:Face)--[:BOUNDED_BY]->(w:Wire) RETURN f.name, w.name").expect("construct query must run");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON result");
        let rows = value["rows"].as_array().expect("rows array");
        assert_eq!(rows.len(), 6, "every one of the 6 faces must match exactly its own wire: {json}");
    }

    #[test]
    fn construct_query_filters_edges_by_curve_kind_property() {
        let geometry = box_geometry();
        let json = run_construct_query(&geometry, "MATCH (e:Edge) WHERE e.curveKind = 'line' RETURN e.name").expect("construct query must run");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON result");
        let rows = value["rows"].as_array().expect("rows array");
        assert_eq!(rows.len(), 12, "all 12 box edges are line curves: {json}");
    }

    #[test]
    fn construct_query_rejects_malformed_syntax_with_a_real_parse_error() {
        let geometry = box_geometry();
        let error = run_construct_query(&geometry, "NOT A QUERY (((").unwrap_err();
        let _ = error; // exists and is Err — the exact message is Jack's own concern, not this adapter's.
    }
}
