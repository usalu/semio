//! 🔍️ Construct query of the cad inference: the read-only graph adapter over one geometry pane. See [`super`].

use semio_s_artifact_cad_cad_model::geometry::CadGeometry;
use semio_framework_graph_dsl::{QueryableEdge, QueryableGraph};
use semio_framework_graph_manifest::PropertyValue;
use std::collections::BTreeSet;

const KIND_VERTEX: &str = "Vertex";
const KIND_EDGE: &str = "Edge";
const KIND_WIRE: &str = "Wire";
const KIND_FACE: &str = "Face";
const KIND_SHELL: &str = "Shell";
const KIND_SOLID: &str = "Solid";

const REL_BOUNDED_BY: &str = "BOUNDED_BY";
const REL_CONTAINS: &str = "CONTAINS";

/// 🕸️ One `CadGeometry` pane (e.g. `scene.shape_geometry`), exposed as a Jack
/// `QueryableGraph` — read-only, matching `construct.md`'s explicit constraint that direct
/// graph mutation is unsafe for a B-rep and must go through a validated command layer instead.
pub struct CadTopologyGraph<'a> {
    geometry: &'a CadGeometry,
}

impl<'a> CadTopologyGraph<'a> {
    #[cfg(test)]
    pub(crate) fn new(geometry: &'a CadGeometry) -> Self {
        Self { geometry }
    }
}

impl QueryableGraph for CadTopologyGraph<'_> {
    fn manifest(&self) -> Option<&semio_framework_graph_manifest::GraphManifest> {
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
            out.push(QueryableEdge { id: format!("{kind}-{next_id}"), kind: kind.to_string(), source_node_id, target_node_id, source_port: None, target_port: None, properties: semio_framework_graph_manifest::PropertyBag::default() });
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

    fn subgraph_snapshot_json(&self, _node_ids: &BTreeSet<String>, _edge_ids: &BTreeSet<String>) -> Option<String> {
        None
    }
}

/// 🔍️ Runs a Jack query against one `CadGeometry` pane and returns its JSON result —
/// the single entry point `cad-ui`/an MCP tool calls for topology queries (`saved selections`,
/// non-manifold-edge checks, adjacency lookups), reusing `semio_framework_graph_dsl::run_query_json`
/// unchanged.
#[cfg(test)]
pub(crate) fn run_construct_query(geometry: &CadGeometry, source: &str) -> Result<String, semio_framework_graph_dsl::GraphDslError> {
    let graph = CadTopologyGraph::new(geometry);
    semio_framework_graph_dsl::run_query_json(&graph, source)
}

#[cfg(test)]
include!("../🧪️tests/🔬️construct-query-unit/🦀️.rs");
