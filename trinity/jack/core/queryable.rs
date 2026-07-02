//! 🔍 Trinity RAM graph adapter for shared Jack query language.

use mathematical_graph_dsl::{QueryableEdge, QueryableGraph};
use mathematical_graph_manifest::{manifest_by_id, GraphManifest, PropertyValue};
use std::collections::BTreeSet;
use std::sync::OnceLock;
use trinity_ram::{port_node_id, Graph};

static TRINITY_JACK_MANIFEST: OnceLock<GraphManifest> = OnceLock::new();

fn trinity_jack_manifest() -> &'static GraphManifest {
    TRINITY_JACK_MANIFEST.get_or_init(|| manifest_by_id("nakagin").expect("nakagin manifest"))
}

/// 🕸️ Jack query surface over an in-memory trinity graph.
pub struct TrinityQueryableGraph<'a>(pub &'a Graph);

impl QueryableGraph for TrinityQueryableGraph<'_> {
    fn manifest(&self) -> Option<&GraphManifest> {
        Some(trinity_jack_manifest())
    }

    fn node_ids(&self) -> Vec<String> {
        self.0.nodes.keys().cloned().collect()
    }

    fn node_kind(&self, id: &str) -> Option<String> {
        self.0.nodes.get(id).map(|node| node.kind.clone())
    }

    fn node_name(&self, id: &str) -> Option<String> {
        self.0.nodes.get(id).map(|node| node.name.clone())
    }

    fn node_property(&self, id: &str, key: &str) -> Option<PropertyValue> {
        let node = self.0.nodes.get(id)?;
        match key {
            "id" => Some(PropertyValue::String(id.to_string())),
            "name" | "label" | "text" => Some(PropertyValue::String(node.name.clone())),
            "kind" => Some(PropertyValue::String(node.kind.clone())),
            "__all" => Some(PropertyValue::Object(node.properties.clone())),
            _ => node.properties.get(key).cloned(),
        }
    }

    fn edges(&self) -> Vec<QueryableEdge> {
        self.0
            .edges
            .values()
            .filter_map(|edge| {
                let source_node_id = port_node_id(&edge.source)?.to_string();
                let target_node_id = port_node_id(&edge.target)?.to_string();
                Some(QueryableEdge {
                    id: edge.id.clone(),
                    kind: edge.kind.clone(),
                    source_node_id,
                    target_node_id,
                    properties: edge.properties.clone(),
                })
            })
            .collect()
    }

    fn subgraph_fixture_json(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> Option<String> {
        self.0.subgraph_fixture(node_ids, edge_ids).to_json().ok()
    }
}
