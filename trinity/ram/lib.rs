//! 🔺 In-memory trinity directed property port graph with compile-time manifest.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// #region 🔖Property
/// 📊 Runtime property value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<PropertyValue>),
    Object(BTreeMap<String, PropertyValue>),
}

impl Default for PropertyValue {
    fn default() -> Self {
        Self::Null
    }
}

impl PropertyValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&BTreeMap<String, PropertyValue>> {
        match self {
            Self::Object(m) => Some(m),
            _ => None,
        }
    }
}

/// 🏷️ Compile-time property kind.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PropertyKind {
    Data,
    Derived,
}

/// 📋 Property definition on a kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDef {
    pub name: String,
    pub kind: PropertyKind,
    #[serde(default = "default_value_type")]
    pub value_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}

fn default_value_type() -> String {
    "any".into()
}

/// 🧮 Property bag keyed by name.
pub type PropertyBag = BTreeMap<String, PropertyValue>;
// #endregion 🔖Property

// #region 🔖Manifest
/// 🔌 Port direction on a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PortDirection {
    In,
    Out,
}

/// 🏷️ Port kind in the manifest.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortKindDef {
    pub name: String,
    pub direction: PortDirection,
    #[serde(default)]
    pub properties: Vec<PropertyDef>,
}

/// 🏷️ Node kind in the manifest.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeKindDef {
    pub name: String,
    #[serde(default)]
    pub properties: Vec<PropertyDef>,
    #[serde(default)]
    pub port_kinds: Vec<String>,
}

/// 🏷️ Edge kind in the manifest.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeKindDef {
    pub name: String,
    #[serde(default)]
    pub properties: Vec<PropertyDef>,
}

/// 📜 Compile-time schema for a trinity graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    #[serde(default)]
    pub node_kinds: Vec<NodeKindDef>,
    #[serde(default)]
    pub edge_kinds: Vec<EdgeKindDef>,
    #[serde(default)]
    pub port_kinds: Vec<PortKindDef>,
}

impl Manifest {
    pub fn nakagin_default() -> Self {
        serde_json::from_value(serde_json::json!({
            "nodeKinds": [{
                "name": "Piece",
                "properties": [
                    { "name": "position", "kind": "data", "valueType": "object" },
                    { "name": "flatPosition", "kind": "derived", "valueType": "object", "expr": "flatFromConnections" }
                ],
                "portKinds": ["Connector"]
            }],
            "edgeKinds": [{
                "name": "Connection",
                "properties": [
                    { "name": "gap", "kind": "data", "valueType": "number" },
                    { "name": "rotation", "kind": "data", "valueType": "number" },
                    { "name": "tilt", "kind": "data", "valueType": "number" },
                    { "name": "rise", "kind": "data", "valueType": "number" },
                    { "name": "turn", "kind": "data", "valueType": "number" },
                    { "name": "shift", "kind": "data", "valueType": "number" },
                    { "name": "u", "kind": "data", "valueType": "number" },
                    { "name": "v", "kind": "data", "valueType": "number" }
                ]
            }],
            "portKinds": [{ "name": "Connector", "direction": "out" }]
        }))
        .unwrap()
    }

    pub fn node_kind(&self, name: &str) -> Option<&NodeKindDef> {
        self.node_kinds.iter().find(|k| k.name == name)
    }

    pub fn edge_kind(&self, name: &str) -> Option<&EdgeKindDef> {
        self.edge_kinds.iter().find(|k| k.name == name)
    }

    pub fn port_kind(&self, name: &str) -> Option<&PortKindDef> {
        self.port_kinds.iter().find(|k| k.name == name)
    }
}
// #endregion 🔖Manifest

// #region 🔖Runtime
/// 🔌 Runtime port on a node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    pub id: String,
    pub kind: String,
    pub direction: PortDirection,
    #[serde(default)]
    pub properties: PropertyBag,
}

/// 🧩 Runtime node (piece).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    #[serde(default)]
    pub properties: PropertyBag,
    #[serde(default)]
    pub ports: Vec<Port>,
}

/// 🔗 Runtime edge (connection).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub properties: PropertyBag,
}

/// 📷 Camera for fixture documents.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraV1 {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for CameraV1 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 📦 `trinity.graph/v1` fixture document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphFixtureV1 {
    pub schema: String,
    pub name: String,
    pub manifest: Manifest,
    pub camera: CameraV1,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_node_id: Option<String>,
}

impl GraphFixtureV1 {
    pub const SCHEMA: &'static str = "trinity.graph/v1";

    pub fn validate_schema(&self) -> Result<(), String> {
        if self.schema != Self::SCHEMA {
            return Err(format!("expected schema {}, got {}", Self::SCHEMA, self.schema));
        }
        Ok(())
    }

    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let fixture: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        fixture.validate_schema()?;
        Ok(fixture)
    }
}

/// 🧠 In-memory trinity graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    pub name: String,
    pub manifest: Manifest,
    pub camera: CameraV1,
    pub nodes: BTreeMap<String, Node>,
    pub edges: BTreeMap<String, Edge>,
    pub root_node_id: Option<String>,
}

impl Graph {
    pub fn from_fixture(fixture: GraphFixtureV1) -> Result<Self, String> {
        fixture.validate_schema()?;
        let mut nodes = BTreeMap::new();
        for node in fixture.nodes {
            nodes.insert(node.id.clone(), node);
        }
        let mut edges = BTreeMap::new();
        for edge in fixture.edges {
            edges.insert(edge.id.clone(), edge);
        }
        Ok(Self {
            name: fixture.name,
            manifest: fixture.manifest,
            camera: fixture.camera,
            nodes,
            edges,
            root_node_id: fixture.root_node_id,
        })
    }

    pub fn to_fixture(&self) -> GraphFixtureV1 {
        GraphFixtureV1 {
            schema: GraphFixtureV1::SCHEMA.to_string(),
            name: self.name.clone(),
            manifest: self.manifest.clone(),
            camera: self.camera.clone(),
            nodes: self.nodes.values().cloned().collect(),
            edges: self.edges.values().cloned().collect(),
            root_node_id: self.root_node_id.clone(),
        }
    }

    pub fn load_json(json: &str) -> Result<Self, String> {
        Self::from_fixture(GraphFixtureV1::from_json(json)?)
    }

    pub fn fixture_json(&self) -> Result<String, String> {
        self.to_fixture().to_json()
    }

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn remove_node(&mut self, id: &str) -> bool {
        if self.nodes.remove(id).is_none() {
            return false;
        }
        let edge_ids: Vec<String> = self
            .edges
            .iter()
            .filter(|(_, e)| port_node_id(&e.source) == Some(id) || port_node_id(&e.target) == Some(id))
            .map(|(id, _)| id.clone())
            .collect();
        for eid in edge_ids {
            self.edges.remove(&eid);
        }
        if self.root_node_id.as_deref() == Some(id) {
            self.root_node_id = None;
        }
        true
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge.id.clone(), edge);
    }

    pub fn remove_edge(&mut self, id: &str) -> bool {
        self.edges.remove(id).is_some()
    }

    pub fn set_property(&mut self, entity: EntityRef, key: &str, value: PropertyValue) -> Result<(), String> {
        match entity {
            EntityRef::Node(id) => {
                let node = self.nodes.get_mut(&id).ok_or_else(|| format!("node {id} not found"))?;
                node.properties.insert(key.to_string(), value);
            }
            EntityRef::Edge(id) => {
                let edge = self.edges.get_mut(&id).ok_or_else(|| format!("edge {id} not found"))?;
                edge.properties.insert(key.to_string(), value);
            }
        }
        Ok(())
    }

    pub fn recompute_derived(&mut self) {
        let root = self
            .root_node_id
            .clone()
            .or_else(|| self.nodes.keys().next().cloned());
        let Some(root_id) = root else { return };
        let mut flat: BTreeMap<String, (f64, f64)> = BTreeMap::new();
        flat.insert(root_id.clone(), (0.0, 0.0));
        let mut queue = vec![root_id];
        while let Some(parent_id) = queue.pop() {
            let (pu, pv) = flat.get(&parent_id).copied().unwrap_or((0.0, 0.0));
            let child_edges: Vec<(String, f64, f64)> = self
                .edges
                .values()
                .filter_map(|e| {
                    let src_node = port_node_id(&e.source)?;
                    let tgt_node = port_node_id(&e.target)?;
                    if src_node == parent_id {
                        let u = e.properties.get("u").and_then(PropertyValue::as_f64).unwrap_or(0.0);
                        let v = e.properties.get("v").and_then(PropertyValue::as_f64).unwrap_or(0.0);
                        return Some((tgt_node.to_string(), pu + u, pv + v));
                    }
                    None
                })
                .collect();
            for (child_id, cu, cv) in child_edges {
                if !flat.contains_key(&child_id) {
                    flat.insert(child_id.clone(), (cu, cv));
                    queue.push(child_id);
                }
            }
        }
        for (node_id, (u, v)) in flat {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                let mut obj = BTreeMap::new();
                obj.insert("u".into(), PropertyValue::Number(u));
                obj.insert("v".into(), PropertyValue::Number(v));
                node.properties.insert("flatPosition".into(), PropertyValue::Object(obj));
            }
        }
    }
}

/// 🎯 Entity reference for mutations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityRef {
    Node(String),
    Edge(String),
}

/// 🔑 Parse `nodeId:portId` port key.
pub fn parse_port_key(key: &str) -> Option<(&str, &str)> {
    let (node, port) = key.split_once(':')?;
    if node.is_empty() || port.is_empty() {
        return None;
    }
    Some((node, port))
}

/// 🧩 Node id from a port key.
pub fn port_node_id(key: &str) -> Option<&str> {
    parse_port_key(key).map(|(n, _)| n)
}

/// 🔌 Port id from a port key.
pub fn port_port_id(key: &str) -> Option<&str> {
    parse_port_key(key).map(|(_, p)| p)
}

/// 🏗️ Build a port key.
pub fn port_key(node_id: &str, port_id: &str) -> String {
    format!("{node_id}:{port_id}")
}
// #endregion 🔖Runtime

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn mini_fixture() -> GraphFixtureV1 {
        GraphFixtureV1 {
            schema: GraphFixtureV1::SCHEMA.into(),
            name: "mini".into(),
            manifest: Manifest::nakagin_default(),
            camera: CameraV1::default(),
            root_node_id: Some("root".into()),
            nodes: vec![
                Node {
                    id: "root".into(),
                    kind: "Piece".into(),
                    name: "core".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: {
                        let mut p = PropertyBag::new();
                        let mut pos = BTreeMap::new();
                        pos.insert("x".into(), PropertyValue::Number(0.0));
                        pos.insert("y".into(), PropertyValue::Number(0.0));
                        pos.insert("z".into(), PropertyValue::Number(0.0));
                        p.insert("position".into(), PropertyValue::Object(pos));
                        p
                    },
                    ports: vec![Port { id: "out-a".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                Node {
                    id: "child".into(),
                    kind: "Piece".into(),
                    name: "capsule".into(),
                    x: 120.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in-a".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
            ],
            edges: vec![Edge {
                id: "e1".into(),
                kind: "Connection".into(),
                source: "root:out-a".into(),
                target: "child:in-a".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(1.2));
                    p.insert("v".into(), PropertyValue::Number(-0.6));
                    p
                },
            }],
        }
    }

    #[test]
    fn manifest_nakagin_has_piece_and_connection() {
        let m = Manifest::nakagin_default();
        assert!(m.node_kind("Piece").is_some());
        assert!(m.edge_kind("Connection").is_some());
        let piece = m.node_kind("Piece").unwrap();
        assert!(piece.properties.iter().any(|p| p.name == "flatPosition" && p.kind == PropertyKind::Derived));
    }

    #[test]
    fn derived_flat_position_bfs() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        g.recompute_derived();
        let root = g.node("root").unwrap();
        let child = g.node("child").unwrap();
        let root_flat = root.properties.get("flatPosition").unwrap().as_object().unwrap();
        let child_flat = child.properties.get("flatPosition").unwrap().as_object().unwrap();
        assert_eq!(root_flat.get("u").and_then(PropertyValue::as_f64), Some(0.0));
        assert_eq!(child_flat.get("u").and_then(PropertyValue::as_f64), Some(1.2));
        assert_eq!(child_flat.get("v").and_then(PropertyValue::as_f64), Some(-0.6));
    }

    #[test]
    fn fixture_round_trip() {
        let fixture = mini_fixture();
        let json = fixture.to_json().unwrap();
        let back = GraphFixtureV1::from_json(&json).unwrap();
        assert_eq!(back.nodes.len(), 2);
        assert_eq!(back.edges.len(), 1);
    }

    #[test]
    fn remove_node_cascades_edges() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        assert!(g.remove_node("root"));
        assert!(g.edges.is_empty());
        assert!(g.nodes.contains_key("child"));
    }
}
// #endregion 🔖Tests
