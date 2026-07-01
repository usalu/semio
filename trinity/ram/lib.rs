//! 🔺 In-memory trinity directed property port graph with compile-time manifest.

use mathematical_graph_manifest::{manifest_by_id, GraphManifest, ManifestValidationError, TrinityManifest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub use mathematical_graph_manifest::{ManifestValidator, PropertyBag, PropertyDef, PropertyKind, PropertyValue, PortDirection};

/// 📜 Compile-time trinity manifest (projection of {@link GraphManifest}).
pub type Manifest = TrinityManifest;

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default)]
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

    pub fn resolve_manifest(&mut self) -> Result<(), String> {
        if let Some(id) = self.manifest_id.as_deref() {
            self.manifest = manifest_by_id(id)
                .ok_or_else(|| format!("unknown manifest id {id}"))?
                .to_trinity_manifest();
            return Ok(());
        }
        if self.manifest.node_kinds.is_empty()
            && self.manifest.edge_kinds.is_empty()
            && self.manifest.port_kinds.is_empty()
        {
            return Err("fixture missing manifest or manifestId".into());
        }
        Ok(())
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        let mut fixture: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        fixture.validate_schema()?;
        fixture.resolve_manifest()?;
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
    pub fn from_fixture(mut fixture: GraphFixtureV1) -> Result<Self, String> {
        fixture.validate_schema()?;
        fixture.resolve_manifest()?;
        if let Some(id) = fixture.manifest_id.as_deref() {
            if let Some(gm) = manifest_by_id(id) {
                validate_trinity_fixture(&gm, &fixture)?;
            }
        }
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
            manifest_id: Some("nakagin".into()),
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

    /// 🧩 Build a `trinity.graph/v1` fixture containing only the given node and edge ids.
    pub fn subgraph_fixture(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> GraphFixtureV1 {
        let nodes: Vec<Node> = node_ids.iter().filter_map(|id| self.nodes.get(id).cloned()).collect();
        let edges: Vec<Edge> = edge_ids.iter().filter_map(|id| self.edges.get(id).cloned()).collect();
        let root_node_id = self.root_node_id.clone().filter(|id| node_ids.contains(id));
        GraphFixtureV1 {
            schema: GraphFixtureV1::SCHEMA.to_string(),
            name: format!("{} subgraph", self.name),
            manifest_id: Some("nakagin".into()),
            manifest: self.manifest.clone(),
            camera: self.camera.clone(),
            nodes,
            edges,
            root_node_id,
        }
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
        if self.nodes.is_empty() {
            return;
        }
        let mut flat: BTreeMap<String, (f64, f64)> = BTreeMap::new();
        if let Some(root_id) = self.root_node_id.clone().or_else(|| self.nodes.keys().next().cloned()) {
            Self::extend_flat_positions_from_seed(self, &mut flat, root_id);
        }
        while flat.len() < self.nodes.len() {
            let remaining: BTreeSet<String> = self.nodes.keys().filter(|id| !flat.contains_key(*id)).cloned().collect();
            if remaining.is_empty() {
                break;
            }
            let seed = remaining
                .iter()
                .find(|id| !Self::has_incoming_from_remaining(self, id, &remaining))
                .cloned()
                .unwrap_or_else(|| remaining.iter().next().expect("remaining non-empty").clone());
            Self::extend_flat_positions_from_seed(self, &mut flat, seed);
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

    fn has_incoming_from_remaining(graph: &Graph, node_id: &str, remaining: &BTreeSet<String>) -> bool {
        graph.edges.values().any(|e| {
            let Some(tgt_node) = port_node_id(&e.target) else {
                return false;
            };
            if tgt_node != node_id {
                return false;
            }
            port_node_id(&e.source)
                .map(|src_node| remaining.contains(src_node))
                .unwrap_or(false)
        })
    }

    fn extend_flat_positions_from_seed(graph: &Graph, flat: &mut BTreeMap<String, (f64, f64)>, seed_id: String) {
        if flat.contains_key(&seed_id) {
            return;
        }
        flat.insert(seed_id.clone(), (0.0, 0.0));
        let mut queue = vec![seed_id];
        while let Some(parent_id) = queue.pop() {
            let (pu, pv) = flat.get(&parent_id).copied().unwrap_or((0.0, 0.0));
            let child_edges: Vec<(String, f64, f64)> = graph
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
    }
}

/// 🛡️ Validates trinity fixture instances against a compile-time graph manifest.
fn validate_trinity_fixture(gm: &GraphManifest, fixture: &GraphFixtureV1) -> Result<(), String> {
    let validator = ManifestValidator::new(gm);
    for node in &fixture.nodes {
        validator.validate_node_kind(&node.kind).map_err(manifest_err)?;
        validator.validate_node_properties(&node.kind, &node.properties).map_err(manifest_err)?;
        if let Some(node_def) = gm.node_kind(&node.kind) {
            for port in &node.ports {
                validator.validate_port_kind(&port.kind).map_err(manifest_err)?;
                if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == &port.kind) {
                    return Err(format!(
                        "nodes/{}/ports/{}: port kind {} not declared on node kind {}",
                        node.id, port.kind, port.kind, node.kind
                    ));
                }
            }
        }
    }
    for edge in &fixture.edges {
        validator.validate_edge_kind(&edge.kind).map_err(manifest_err)?;
        validator.validate_edge_properties(&edge.kind, &edge.properties).map_err(manifest_err)?;
    }
    Ok(())
}

fn manifest_err(error: ManifestValidationError) -> String {
    format!("{}: {}", error.path, error.message)
}

/// 🎯 Entity reference for mutations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "entity", content = "id")]
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

// #region 🔖GraphOps
use vcs::{
    apply_operation, create_document_vcs_envelope, CollectionDiff, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore,
    ItemPatch, Operation, OperationDiff,
};

pub const TRINITY_GRAPH_SCHEMA: &str = GraphFixtureV1::SCHEMA;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGeometryPatch {
    pub name: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPatch {
    pub key: String,
    pub value: Option<PropertyValue>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrinityGraphDiff {
    pub nodes: CollectionDiff<String, NodeGeometryPatch, Node>,
    pub edges: CollectionDiff<String, PropertyPatch, Edge>,
    pub node_properties: Vec<ItemPatch<String, PropertyPatch>>,
    pub edge_properties: Vec<ItemPatch<String, PropertyPatch>>,
    pub recompute_derived: bool,
}

impl OperationDiff<GraphFixtureV1> for TrinityGraphDiff {
    fn apply(&self, projection: &GraphFixtureV1) -> GraphFixtureV1 {
        let mut next = projection.clone();
        for id in &self.nodes.removed {
            remove_node_from_fixture(&mut next, id);
        }
        for patch in &self.nodes.modified {
            if let Some(node) = next.nodes.iter_mut().find(|node| node.id == patch.id) {
                if let Some(name) = &patch.patch.name {
                    node.name = name.clone();
                }
                if let Some(x) = patch.patch.x {
                    node.x = x;
                }
                if let Some(y) = patch.patch.y {
                    node.y = y;
                }
                if let Some(width) = patch.patch.width {
                    node.width = width;
                }
                if let Some(height) = patch.patch.height {
                    node.height = height;
                }
            }
        }
        for node in &self.nodes.added {
            next.nodes.push(node.clone());
        }
        for id in &self.edges.removed {
            next.edges.retain(|edge| edge.id != *id);
        }
        for edge in &self.edges.added {
            next.edges.push(edge.clone());
        }
        for patch in &self.node_properties {
            if let Some(node) = next.nodes.iter_mut().find(|node| node.id == patch.id) {
                match &patch.patch.value {
                    Some(value) => {
                        node.properties.insert(patch.patch.key.clone(), value.clone());
                    }
                    None => {
                        node.properties.remove(&patch.patch.key);
                    }
                }
            }
        }
        for patch in &self.edge_properties {
            if let Some(edge) = next.edges.iter_mut().find(|edge| edge.id == patch.id) {
                match &patch.patch.value {
                    Some(value) => {
                        edge.properties.insert(patch.patch.key.clone(), value.clone());
                    }
                    None => {
                        edge.properties.remove(&patch.patch.key);
                    }
                }
            }
        }
        if self.recompute_derived {
            if let Ok(mut graph) = Graph::from_fixture(next.clone()) {
                graph.recompute_derived();
                next = graph.to_fixture();
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.nodes.removed.extend(other.nodes.removed);
        self.nodes.modified.extend(other.nodes.modified);
        self.nodes.added.extend(other.nodes.added);
        self.edges.removed.extend(other.edges.removed);
        self.edges.added.extend(other.edges.added);
        self.node_properties.extend(other.node_properties);
        self.edge_properties.extend(other.edge_properties);
        self.recompute_derived |= other.recompute_derived;
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum TrinityGraphOp {
    CreateNode {
        id: String,
        kind: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        ports: Vec<Port>,
    },
    DeleteNode {
        id: String,
    },
    CreateEdge {
        id: String,
        kind: String,
        source: String,
        target: String,
        properties: PropertyBag,
    },
    DeleteEdge {
        id: String,
    },
    Rename {
        id: String,
        name: String,
    },
    Reposition {
        id: String,
        x: f64,
        y: f64,
    },
    SetDataProperty {
        entity: EntityRef,
        key: String,
        value: PropertyValue,
    },
    ClearDataProperty {
        entity: EntityRef,
        key: String,
    },
}

pub type TrinityGraphEnvelope = DocumentVcsEnvelope<GraphFixtureV1, TrinityGraphOp>;
pub type TrinityGraphStore = DocumentVcsStore<GraphFixtureV1, TrinityGraphOp>;

pub fn create_trinity_graph_envelope(id: &str, fixture: GraphFixtureV1) -> TrinityGraphEnvelope {
    create_document_vcs_envelope(TRINITY_GRAPH_SCHEMA, id, fixture, None)
}

pub fn validate_trinity_graph_op(op: &TrinityGraphOp, fixture: &GraphFixtureV1) -> Result<(), String> {
    match op {
        TrinityGraphOp::CreateNode { id, kind, ports, .. } => {
            if fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(format!("node {id} already exists"));
            }
            validate_node_kind_trinity(&fixture.manifest, kind)?;
            if let Some(node_def) = fixture.manifest.node_kind(kind) {
                for port in ports {
                    validate_port_kind_trinity(&fixture.manifest, &port.kind)?;
                    if !node_def.port_kinds.is_empty() && !node_def.port_kinds.iter().any(|p| p == &port.kind) {
                        return Err(format!(
                            "nodes/{id}/ports/{}: port kind {} not declared on node kind {}",
                            port.id, port.kind, kind
                        ));
                    }
                }
            }
        }
        TrinityGraphOp::DeleteNode { id } => {
            if !fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(format!("node {id} not found"));
            }
        }
        TrinityGraphOp::CreateEdge { id, kind, source, target, properties } => {
            if fixture.edges.iter().any(|edge| edge.id == *id) {
                return Err(format!("edge {id} already exists"));
            }
            validate_edge_kind_trinity(&fixture.manifest, kind)?;
            validate_edge_properties_trinity(&fixture.manifest, kind, properties)?;
            let source_node = port_node_id(source).ok_or_else(|| format!("invalid source port key {source}"))?;
            let target_node = port_node_id(target).ok_or_else(|| format!("invalid target port key {target}"))?;
            if !fixture.nodes.iter().any(|node| node.id == source_node) {
                return Err(format!("source node {source_node} not found"));
            }
            if !fixture.nodes.iter().any(|node| node.id == target_node) {
                return Err(format!("target node {target_node} not found"));
            }
        }
        TrinityGraphOp::DeleteEdge { id } => {
            if !fixture.edges.iter().any(|edge| edge.id == *id) {
                return Err(format!("edge {id} not found"));
            }
        }
        TrinityGraphOp::Rename { id, .. } | TrinityGraphOp::Reposition { id, .. } => {
            if !fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(format!("node {id} not found"));
            }
        }
        TrinityGraphOp::SetDataProperty { entity, key, value } => {
            validate_set_data_property(fixture, entity, key, value)?;
        }
        TrinityGraphOp::ClearDataProperty { entity, key } => {
            validate_clear_data_property(fixture, entity, key)?;
        }
    }
    Ok(())
}

pub fn apply_trinity_graph_ops(fixture: GraphFixtureV1, ops: &[TrinityGraphOp]) -> Result<GraphFixtureV1, String> {
    let mut projection = fixture;
    for op in ops {
        validate_trinity_graph_op(op, &projection)?;
        projection = apply_operation(&projection, op);
    }
    Ok(projection)
}

pub fn dispatch_trinity_graph_ops(store: &mut TrinityGraphStore, ops: Vec<TrinityGraphOp>) -> Result<(), String> {
    if ops.is_empty() {
        return Ok(());
    }
    let mut projection = store.projection().map_err(|e| e.to_string())?;
    for op in &ops {
        validate_trinity_graph_op(op, &projection)?;
        projection = apply_operation(&projection, op);
    }
    store
        .dispatch(DocumentVcsCommand::Apply {
            operations: ops,
            description: None,
        })
        .map_err(|e| e.to_string())
}

fn validate_clear_data_property(fixture: &GraphFixtureV1, entity: &EntityRef, key: &str) -> Result<(), String> {
    match entity {
        EntityRef::Node(id) => {
            fixture.nodes.iter().find(|node| node.id == *id).ok_or_else(|| format!("node {id} not found"))?;
        }
        EntityRef::Edge(id) => {
            fixture.edges.iter().find(|edge| edge.id == *id).ok_or_else(|| format!("edge {id} not found"))?;
        }
    }
    let _ = key;
    Ok(())
}

fn validate_set_data_property(
    fixture: &GraphFixtureV1,
    entity: &EntityRef,
    key: &str,
    value: &PropertyValue,
) -> Result<(), String> {
    let (defs, path_prefix) = match entity {
        EntityRef::Node(id) => {
            let node = fixture.nodes.iter().find(|node| node.id == *id).ok_or_else(|| format!("node {id} not found"))?;
            (
                fixture.manifest.node_kind(&node.kind).map(|def| &def.properties[..]),
                format!("nodes/{id}/properties/{key}"),
            )
        }
        EntityRef::Edge(id) => {
            let edge = fixture.edges.iter().find(|edge| edge.id == *id).ok_or_else(|| format!("edge {id} not found"))?;
            (
                fixture.manifest.edge_kind(&edge.kind).map(|def| &def.properties[..]),
                format!("edges/{id}/properties/{key}"),
            )
        }
    };
    let Some(defs) = defs else {
        return Err(format!("{path_prefix}: unknown kind"));
    };
    let Some(def) = defs.iter().find(|def| def.name == key) else {
        return Err(format!("{path_prefix}: unknown property {key:?}"));
    };
    if def.kind == PropertyKind::Derived {
        return Err(format!("{path_prefix}: property {key:?} is derived and cannot be set"));
    }
    let mut bag = PropertyBag::new();
    bag.insert(key.to_string(), value.clone());
    validate_property_bag_trinity(&path_prefix, defs, &bag)
}

fn validate_node_kind_trinity(manifest: &Manifest, kind: &str) -> Result<(), String> {
    if manifest.node_kind(kind).is_some() {
        Ok(())
    } else {
        Err(format!("nodes/{kind}: unknown node kind {kind:?}"))
    }
}

fn validate_edge_kind_trinity(manifest: &Manifest, kind: &str) -> Result<(), String> {
    if manifest.edge_kind(kind).is_some() {
        Ok(())
    } else {
        Err(format!("edges/{kind}: unknown edge kind {kind:?}"))
    }
}

fn validate_port_kind_trinity(manifest: &Manifest, kind: &str) -> Result<(), String> {
    if manifest.port_kind(kind).is_some() {
        Ok(())
    } else {
        Err(format!("ports/{kind}: unknown port kind {kind:?}"))
    }
}

fn validate_edge_properties_trinity(manifest: &Manifest, kind: &str, properties: &PropertyBag) -> Result<(), String> {
    let Some(def) = manifest.edge_kind(kind) else {
        return validate_edge_kind_trinity(manifest, kind);
    };
    validate_property_bag_trinity(&format!("edges/{kind}/properties"), &def.properties, properties)
}

fn validate_property_bag_trinity(path: &str, defs: &[PropertyDef], bag: &PropertyBag) -> Result<(), String> {
    for def in defs {
        if def.kind == PropertyKind::Derived {
            continue;
        }
        let Some(value) = bag.get(&def.name) else {
            continue;
        };
        if !property_value_matches_type_trinity(value, def) {
            return Err(format!(
                "{path}/{}: property type mismatch for {}",
                def.name,
                def.value_type.id()
            ));
        }
    }
    for key in bag.keys() {
        if !defs.iter().any(|def| def.name == *key) {
            return Err(format!("{path}/{key}: unknown property {key:?}"));
        }
    }
    Ok(())
}

fn property_value_matches_type_trinity(value: &PropertyValue, def: &PropertyDef) -> bool {
    match value {
        PropertyValue::Null => def.value_type.id() == "null",
        PropertyValue::Bool(_) => def.value_type.id() == "boolean",
        PropertyValue::Number(_) => {
            let id = def.value_type.id();
            id == "decimal" || id == "integer" || id == "number"
        }
        PropertyValue::String(_) => {
            let id = def.value_type.id();
            id == "string" || id == "text"
        }
        PropertyValue::Object(_) => {
            let id = def.value_type.id();
            id.starts_with("schema:") || id == "object"
        }
        PropertyValue::Array(_) => def.value_type.id() == "array",
    }
}

fn remove_node_from_fixture(fixture: &mut GraphFixtureV1, id: &str) {
    fixture.nodes.retain(|node| node.id != id);
    fixture.edges.retain(|edge| {
        port_node_id(&edge.source) != Some(id.as_ref()) && port_node_id(&edge.target) != Some(id.as_ref())
    });
    if fixture.root_node_id.as_deref() == Some(id) {
        fixture.root_node_id = None;
    }
}

fn delete_node_snapshot(fixture: &GraphFixtureV1, id: &str) -> (Option<Node>, Vec<Edge>) {
    let node = fixture.nodes.iter().find(|node| node.id == id).cloned();
    let edges: Vec<Edge> = fixture
        .edges
        .iter()
        .filter(|edge| port_node_id(&edge.source) == Some(id) || port_node_id(&edge.target) == Some(id))
        .cloned()
        .collect();
    (node, edges)
}

fn entity_property_value(fixture: &GraphFixtureV1, entity: &EntityRef, key: &str) -> Option<PropertyValue> {
    match entity {
        EntityRef::Node(id) => fixture
            .nodes
            .iter()
            .find(|node| node.id == *id)
            .and_then(|node| node.properties.get(key).cloned()),
        EntityRef::Edge(id) => fixture
            .edges
            .iter()
            .find(|edge| edge.id == *id)
            .and_then(|edge| edge.properties.get(key).cloned()),
    }
}

impl Operation<GraphFixtureV1> for TrinityGraphOp {
    type Diff = TrinityGraphDiff;

    fn diff(&self, projection: &GraphFixtureV1) -> TrinityGraphDiff {
        match self {
            TrinityGraphOp::CreateNode {
                id,
                kind,
                name,
                x,
                y,
                width,
                height,
                ports,
            } => TrinityGraphDiff {
                nodes: CollectionDiff {
                    added: vec![Node {
                        id: id.clone(),
                        kind: kind.clone(),
                        name: name.clone(),
                        x: *x,
                        y: *y,
                        width: *width,
                        height: *height,
                        properties: PropertyBag::new(),
                        ports: ports.clone(),
                    }],
                    ..Default::default()
                },
                recompute_derived: true,
                ..Default::default()
            },
            TrinityGraphOp::DeleteNode { id } => {
                let (node, edges) = delete_node_snapshot(projection, id);
                TrinityGraphDiff {
                    nodes: CollectionDiff {
                        removed: node.as_ref().map(|node| vec![node.id.clone()]).unwrap_or_default(),
                        ..Default::default()
                    },
                    edges: CollectionDiff {
                        removed: edges.iter().map(|edge| edge.id.clone()).collect(),
                        ..Default::default()
                    },
                    recompute_derived: true,
                    ..Default::default()
                }
            }
            TrinityGraphOp::CreateEdge {
                id,
                kind,
                source,
                target,
                properties,
            } => TrinityGraphDiff {
                edges: CollectionDiff {
                    added: vec![Edge {
                        id: id.clone(),
                        kind: kind.clone(),
                        source: source.clone(),
                        target: target.clone(),
                        properties: properties.clone(),
                    }],
                    ..Default::default()
                },
                recompute_derived: true,
                ..Default::default()
            },
            TrinityGraphOp::DeleteEdge { id } => TrinityGraphDiff {
                edges: CollectionDiff {
                    removed: vec![id.clone()],
                    ..Default::default()
                },
                recompute_derived: true,
                ..Default::default()
            },
            TrinityGraphOp::Rename { id, name } => TrinityGraphDiff {
                nodes: CollectionDiff {
                    modified: vec![ItemPatch {
                        id: id.clone(),
                        patch: NodeGeometryPatch {
                            name: Some(name.clone()),
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            },
            TrinityGraphOp::Reposition { id, x, y } => TrinityGraphDiff {
                nodes: CollectionDiff {
                    modified: vec![ItemPatch {
                        id: id.clone(),
                        patch: NodeGeometryPatch {
                            x: Some(*x),
                            y: Some(*y),
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            },
            TrinityGraphOp::SetDataProperty { entity, key, value } => {
                let patch = PropertyPatch {
                    key: key.clone(),
                    value: Some(value.clone()),
                };
                let recompute = matches!(entity, EntityRef::Edge(_)) && (key == "u" || key == "v");
                match entity {
                    EntityRef::Node(id) => TrinityGraphDiff {
                        node_properties: vec![ItemPatch { id: id.clone(), patch }],
                        recompute_derived: key == "flatPosition",
                        ..Default::default()
                    },
                    EntityRef::Edge(id) => TrinityGraphDiff {
                        edge_properties: vec![ItemPatch { id: id.clone(), patch }],
                        recompute_derived: recompute,
                        ..Default::default()
                    },
                }
            }
            TrinityGraphOp::ClearDataProperty { entity, key } => {
                let patch = PropertyPatch {
                    key: key.clone(),
                    value: None,
                };
                match entity {
                    EntityRef::Node(id) => TrinityGraphDiff {
                        node_properties: vec![ItemPatch { id: id.clone(), patch }],
                        ..Default::default()
                    },
                    EntityRef::Edge(id) => TrinityGraphDiff {
                        edge_properties: vec![ItemPatch { id: id.clone(), patch }],
                        recompute_derived: key == "u" || key == "v",
                        ..Default::default()
                    },
                }
            }
        }
    }

    fn backwards(&self, projection: &GraphFixtureV1) -> Vec<Self> {
        match self {
            TrinityGraphOp::CreateNode { id, .. } => vec![TrinityGraphOp::DeleteNode { id: id.clone() }],
            TrinityGraphOp::DeleteNode { id } => {
                let (node, edges) = delete_node_snapshot(projection, id);
                let mut out = Vec::new();
                if let Some(node) = node {
                    out.push(TrinityGraphOp::CreateNode {
                        id: node.id,
                        kind: node.kind,
                        name: node.name,
                        x: node.x,
                        y: node.y,
                        width: node.width,
                        height: node.height,
                        ports: node.ports,
                    });
                    for edge in edges {
                        out.push(TrinityGraphOp::CreateEdge {
                            id: edge.id,
                            kind: edge.kind,
                            source: edge.source,
                            target: edge.target,
                            properties: edge.properties,
                        });
                    }
                }
                out
            }
            TrinityGraphOp::CreateEdge {
                id,
                kind,
                source,
                target,
                properties,
            } => vec![TrinityGraphOp::DeleteEdge { id: id.clone() }],
            TrinityGraphOp::DeleteEdge { id } => projection
                .edges
                .iter()
                .find(|edge| edge.id == *id)
                .map(|edge| {
                    vec![TrinityGraphOp::CreateEdge {
                        id: edge.id.clone(),
                        kind: edge.kind.clone(),
                        source: edge.source.clone(),
                        target: edge.target.clone(),
                        properties: edge.properties.clone(),
                    }]
                })
                .unwrap_or_default(),
            TrinityGraphOp::Rename { id, name } => projection
                .nodes
                .iter()
                .find(|node| node.id == *id)
                .map(|node| vec![TrinityGraphOp::Rename {
                    id: id.clone(),
                    name: node.name.clone(),
                }])
                .unwrap_or_default(),
            TrinityGraphOp::Reposition { id, x, y } => projection
                .nodes
                .iter()
                .find(|node| node.id == *id)
                .map(|node| vec![TrinityGraphOp::Reposition {
                    id: id.clone(),
                    x: node.x,
                    y: node.y,
                }])
                .unwrap_or_default(),
            TrinityGraphOp::SetDataProperty { entity, key, value } => {
                let prior = entity_property_value(projection, entity, key);
                match (entity, prior) {
                    (EntityRef::Node(id), Some(old)) => vec![TrinityGraphOp::SetDataProperty {
                        entity: EntityRef::Node(id.clone()),
                        key: key.clone(),
                        value: old,
                    }],
                    (EntityRef::Edge(id), Some(old)) => vec![TrinityGraphOp::SetDataProperty {
                        entity: EntityRef::Edge(id.clone()),
                        key: key.clone(),
                        value: old,
                    }],
                    (entity, None) => vec![TrinityGraphOp::ClearDataProperty {
                        entity: entity.clone(),
                        key: key.clone(),
                    }],
                }
            }
            TrinityGraphOp::ClearDataProperty { entity, key } => entity_property_value(projection, entity, key)
                .map(|old| {
                    vec![TrinityGraphOp::SetDataProperty {
                        entity: entity.clone(),
                        key: key.clone(),
                        value: old,
                    }]
                })
                .unwrap_or_default(),
        }
    }
}

impl Default for NodeGeometryPatch {
    fn default() -> Self {
        Self {
            name: None,
            x: None,
            y: None,
            width: None,
            height: None,
        }
    }
}
// #endregion 🔖GraphOps

pub fn empty_trinity_graph_fixture() -> GraphFixtureV1 {
    GraphFixtureV1 {
        schema: GraphFixtureV1::SCHEMA.into(),
        name: "trinity".into(),
        manifest_id: Some("nakagin".into()),
        manifest: Manifest::nakagin_default(),
        camera: CameraV1::default(),
        nodes: Vec::new(),
        edges: Vec::new(),
        root_node_id: None,
    }
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct TrinityGraphDocumentVcs {
        store: RefCell<TrinityGraphStore>,
    }

    #[wasm_bindgen]
    impl TrinityGraphDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<TrinityGraphDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: TrinityGraphEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    TrinityGraphStore::new(envelope)
                }
                None => TrinityGraphStore::new(create_trinity_graph_envelope("trinity", empty_trinity_graph_fixture())),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .envelope_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn mini_fixture() -> GraphFixtureV1 {
        GraphFixtureV1 {
            schema: GraphFixtureV1::SCHEMA.into(),
            name: "mini".into(),
            manifest_id: Some("nakagin".into()),
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
    fn fixture_loads_manifest_id_only() {
        let json = r#"{"schema":"trinity.graph/v1","name":"mini","manifestId":"nakagin","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
        let graph = Graph::load_json(json).unwrap();
        assert!(graph.manifest.node_kind("Piece").is_some());
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

    #[test]
    fn derived_flat_position_covers_disconnected_components() {
        let fixture = GraphFixtureV1 {
            schema: GraphFixtureV1::SCHEMA.into(),
            name: "disconnected".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: CameraV1::default(),
            root_node_id: Some("root-a".into()),
            nodes: vec![
                Node {
                    id: "root-a".into(),
                    kind: "Piece".into(),
                    name: "a".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                Node {
                    id: "child-a".into(),
                    kind: "Piece".into(),
                    name: "a-child".into(),
                    x: 100.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
                Node {
                    id: "root-b".into(),
                    kind: "Piece".into(),
                    name: "b".into(),
                    x: 300.0,
                    y: 200.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                Node {
                    id: "child-b".into(),
                    kind: "Piece".into(),
                    name: "b-child".into(),
                    x: 400.0,
                    y: 200.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
            ],
            edges: vec![
                Edge {
                    id: "e-a".into(),
                    kind: "Connection".into(),
                    source: "root-a:out".into(),
                    target: "child-a:in".into(),
                    properties: {
                        let mut p = PropertyBag::new();
                        p.insert("u".into(), PropertyValue::Number(2.0));
                        p.insert("v".into(), PropertyValue::Number(1.0));
                        p
                    },
                },
                Edge {
                    id: "e-b".into(),
                    kind: "Connection".into(),
                    source: "root-b:out".into(),
                    target: "child-b:in".into(),
                    properties: {
                        let mut p = PropertyBag::new();
                        p.insert("u".into(), PropertyValue::Number(3.0));
                        p.insert("v".into(), PropertyValue::Number(-1.0));
                        p
                    },
                },
            ],
        };
        let mut g = Graph::from_fixture(fixture).unwrap();
        g.recompute_derived();
        let child_a = g.node("child-a").unwrap();
        let child_b = g.node("child-b").unwrap();
        let flat_a = child_a.properties.get("flatPosition").unwrap().as_object().unwrap();
        let flat_b = child_b.properties.get("flatPosition").unwrap().as_object().unwrap();
        assert_eq!(flat_a.get("u").and_then(PropertyValue::as_f64), Some(2.0));
        assert_eq!(flat_a.get("v").and_then(PropertyValue::as_f64), Some(1.0));
        assert_eq!(flat_b.get("u").and_then(PropertyValue::as_f64), Some(3.0));
        assert_eq!(flat_b.get("v").and_then(PropertyValue::as_f64), Some(-1.0));
    }

    #[test]
    fn graph_op_create_node_and_undo() {
        let fixture = mini_fixture();
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", fixture));
        dispatch_trinity_graph_ops(
            &mut store,
            vec![TrinityGraphOp::CreateNode {
                id: "new".into(),
                kind: "Piece".into(),
                name: "new-piece".into(),
                x: 200.0,
                y: 40.0,
                width: 80.0,
                height: 40.0,
                ports: vec![],
            }],
        )
        .expect("create");
        assert_eq!(store.projection().expect("projection").nodes.len(), 3);
        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").nodes.len(), 2);
    }

    #[test]
    fn graph_op_dispatch_validates_create_edge_batch_incrementally() {
        let mut fixture = mini_fixture();
        while fixture.nodes.len() < 9 {
            fixture.nodes.push(Node {
                id: format!("pad-{}", fixture.nodes.len()),
                kind: "Piece".into(),
                name: format!("pad-{}", fixture.nodes.len()),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![],
            });
        }
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", fixture));
        dispatch_trinity_graph_ops(
            &mut store,
            vec![
                TrinityGraphOp::CreateNode {
                    id: "x-9".into(),
                    kind: "Piece".into(),
                    name: "x".into(),
                    x: 1080.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    ports: vec![Port {
                        id: "out".into(),
                        kind: "Connector".into(),
                        direction: PortDirection::Out,
                        properties: PropertyBag::new(),
                    }],
                },
                TrinityGraphOp::CreateNode {
                    id: "y-10".into(),
                    kind: "Piece".into(),
                    name: "y".into(),
                    x: 1200.0,
                    y: 80.0,
                    width: 80.0,
                    height: 40.0,
                    ports: vec![Port {
                        id: "in".into(),
                        kind: "Connector".into(),
                        direction: PortDirection::In,
                        properties: PropertyBag::new(),
                    }],
                },
                TrinityGraphOp::CreateEdge {
                    id: "e-batch".into(),
                    kind: "Connection".into(),
                    source: port_key("x-9", "out"),
                    target: port_key("y-10", "in"),
                    properties: PropertyBag::new(),
                },
            ],
        )
        .expect("batch create edge");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.nodes.len(), 11);
        assert_eq!(projection.edges.len(), 2);
    }

    #[test]
    fn graph_op_rejects_unknown_node_kind() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_op(
            &TrinityGraphOp::CreateNode {
                id: "new".into(),
                kind: "Piece2".into(),
                name: "x".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                ports: vec![],
            },
            &fixture,
        )
        .expect_err("unknown kind");
        assert!(err.contains("unknown node kind"));
    }

    #[test]
    fn graph_op_rejects_derived_property_set() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_op(
            &TrinityGraphOp::SetDataProperty {
                entity: EntityRef::Node("root".into()),
                key: "flatPosition".into(),
                value: PropertyValue::Null,
            },
            &fixture,
        )
        .expect_err("derived");
        assert!(err.contains("derived"));
    }
}
// #endregion 🔖Tests
