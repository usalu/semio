//! 🔺️ In-memory trinity directed property port graph with compile-time manifest.

use mathematical_graph_manifest::{manifest_by_id, GraphManifest, ManifestValidationError, TrinityManifest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub use mathematical_graph_manifest::{ManifestValidator, PortDirection, PropertyBag, PropertyDef, PropertyKind, PropertyValue};

/// 📜️ Compile-time trinity manifest (projection of {@link GraphManifest}).
pub type Manifest = TrinityManifest;

//#region ⚠️ Errors
/// ⚠️ Trinity graph fixture, manifest-validation, and mutation errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityRamError {
    /// 🧬️ JSON (de)serialization failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// 🧭️ VCS store/dispatch failure.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
    /// 📜️ Compile-time manifest validation failure (path-qualified).
    #[error("{}: {}", .0.path, .0.message)]
    Manifest(ManifestValidationError),
    #[error("expected schema {expected}, got {actual}")]
    SchemaMismatch { expected: &'static str, actual: String },
    #[error("unknown manifest id {0}")]
    UnknownManifestId(String),
    #[error("fixture missing manifest or manifestId")]
    ManifestMissing,
    #[error("node {0} not found")]
    NodeNotFound(String),
    #[error("edge {0} not found")]
    EdgeNotFound(String),
    #[error("node {0} already exists")]
    NodeAlreadyExists(String),
    #[error("edge {0} already exists")]
    EdgeAlreadyExists(String),
    #[error("invalid source port key {0}")]
    InvalidSourcePortKey(String),
    #[error("invalid target port key {0}")]
    InvalidTargetPortKey(String),
    #[error("source node {0} not found")]
    SourceNodeNotFound(String),
    #[error("target node {0} not found")]
    TargetNodeNotFound(String),
    #[error("nodes/{node_id}/ports/{port_kind}: port kind {port_kind} not declared on node kind {node_kind}")]
    PortKindNotDeclaredOnFixture { node_id: String, port_kind: String, node_kind: String },
    #[error("nodes/{node_id}/ports/{port_id}: port kind {port_kind} not declared on node kind {node_kind}")]
    PortKindNotDeclaredOnOperation { node_id: String, port_id: String, port_kind: String, node_kind: String },
    #[error("nodes/{kind}: unknown node kind {kind:?}")]
    UnknownNodeKind { kind: String },
    #[error("edges/{kind}: unknown edge kind {kind:?}")]
    UnknownEdgeKind { kind: String },
    #[error("ports/{kind}: unknown port kind {kind:?}")]
    UnknownPortKind { kind: String },
    #[error("{path}: unknown kind")]
    UnknownEntityKind { path: String },
    #[error("{path}: unknown property {key:?}")]
    UnknownPropertyAtPath { path: String, key: String },
    #[error("{path}: property {key:?} is derived and cannot be set")]
    DerivedPropertyReadonly { path: String, key: String },
    #[error("{path}/{name}: property type mismatch for {value_type}")]
    PropertyTypeMismatch { path: String, name: String, value_type: String },
    #[error("{path}/{key}: unknown property {key:?}")]
    UnknownPropertyInBag { path: String, key: String }
}

/// 🔀️ [`ManifestValidationError`] carries no `std::error::Error` impl of its own (plain path/message struct), so this is a manual conversion rather than `#[from]`.
impl From<ManifestValidationError> for TrinityRamError {
    fn from(error: ManifestValidationError) -> Self {
        Self::Manifest(error)
    }
}
//#endregion ⚠️ Errors

// #region 🔖️Runtime
/// 🔌️ Runtime port on a node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    pub id: String,
    pub kind: String,
    pub direction: PortDirection,
    #[serde(default)]
    pub properties: PropertyBag,
}

/// 🧩️ Runtime node (piece).
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

/// 🔗️ Runtime edge (connection).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub properties: PropertyBag,
}

/// 📷️ Camera for fixture documents.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 📦️ `trinity.graph` fixture document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphFixture {
    pub schema: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default)]
    pub manifest: Manifest,
    /// 🌱️ Seed-only initial viewport framing for curated examples — consumed once into an app's
    /// runtime camera when a fixture is first loaded, never written back to by any operation.
    pub camera: Camera,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_node_id: Option<String>,
}

impl GraphFixture {
    pub const SCHEMA: &'static str = "trinity.graph";

    pub fn validate_schema(&self) -> Result<(), TrinityRamError> {
        if self.schema != Self::SCHEMA {
            return Err(TrinityRamError::SchemaMismatch { expected: Self::SCHEMA, actual: self.schema.clone() });
        }
        Ok(())
    }

    pub fn to_json(&self) -> Result<String, TrinityRamError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn resolve_manifest(&mut self) -> Result<(), TrinityRamError> {
        if let Some(id) = self.manifest_id.as_deref() {
            self.manifest = manifest_by_id(id).ok_or_else(|| TrinityRamError::UnknownManifestId(id.to_string()))?.to_trinity_manifest();
            return Ok(());
        }
        if self.manifest.node_kinds.is_empty() && self.manifest.edge_kinds.is_empty() && self.manifest.port_kinds.is_empty() {
            return Err(TrinityRamError::ManifestMissing);
        }
        Ok(())
    }

    pub fn from_json(json: &str) -> Result<Self, TrinityRamError> {
        let mut fixture: Self = serde_json::from_str(json)?;
        fixture.validate_schema()?;
        fixture.resolve_manifest()?;
        Ok(fixture)
    }
}

/// 🧠️ In-memory trinity graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Graph {
    pub name: String,
    pub manifest: Manifest,
    pub camera: Camera,
    pub nodes: BTreeMap<String, Node>,
    pub edges: BTreeMap<String, Edge>,
    pub root_node_id: Option<String>,
}

impl Graph {
    pub fn from_fixture(mut fixture: GraphFixture) -> Result<Self, TrinityRamError> {
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
        Ok(Self { name: fixture.name, manifest: fixture.manifest, camera: fixture.camera, nodes, edges, root_node_id: fixture.root_node_id })
    }

    pub fn to_fixture(&self) -> GraphFixture {
        GraphFixture {
            schema: GraphFixture::SCHEMA.to_string(),
            name: self.name.clone(),
            manifest_id: Some("nakagin".into()),
            manifest: self.manifest.clone(),
            camera: self.camera.clone(),
            nodes: self.nodes.values().cloned().collect(),
            edges: self.edges.values().cloned().collect(),
            root_node_id: self.root_node_id.clone(),
        }
    }

    pub fn load_json(json: &str) -> Result<Self, TrinityRamError> {
        Self::from_fixture(GraphFixture::from_json(json)?)
    }

    pub fn fixture_json(&self) -> Result<String, TrinityRamError> {
        self.to_fixture().to_json()
    }

    /// 🧩️ Build a `trinity.graph` fixture containing only the given node and edge ids.
    pub fn subgraph_fixture(&self, node_ids: &BTreeSet<String>, edge_ids: &BTreeSet<String>) -> GraphFixture {
        let nodes: Vec<Node> = node_ids.iter().filter_map(|id| self.nodes.get(id).cloned()).collect();
        let edges: Vec<Edge> = edge_ids.iter().filter_map(|id| self.edges.get(id).cloned()).collect();
        let root_node_id = self.root_node_id.clone().filter(|id| node_ids.contains(id));
        GraphFixture { schema: GraphFixture::SCHEMA.to_string(), name: format!("{} subgraph", self.name), manifest_id: Some("nakagin".into()), manifest: self.manifest.clone(), camera: self.camera.clone(), nodes, edges, root_node_id }
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
        let edge_ids: Vec<String> = self.edges.iter().filter(|(_, e)| port_node_id(&e.source) == Some(id) || port_node_id(&e.target) == Some(id)).map(|(id, _)| id.clone()).collect();
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

    pub fn set_property(&mut self, entity: EntityRef, key: &str, value: PropertyValue) -> Result<(), TrinityRamError> {
        match entity {
            EntityRef::Node(id) => {
                let node = self.nodes.get_mut(&id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
                node.properties.insert(key.to_string(), value);
            }
            EntityRef::Edge(id) => {
                let edge = self.edges.get_mut(&id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
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
            let seed = remaining.iter().find(|id| !Self::has_incoming_from_remaining(self, id, &remaining)).cloned().unwrap_or_else(|| remaining.iter().next().expect("remaining non-empty").clone());
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
            port_node_id(&e.source).map(|src_node| remaining.contains(src_node)).unwrap_or(false)
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
fn validate_trinity_fixture(gm: &GraphManifest, fixture: &GraphFixture) -> Result<(), TrinityRamError> {
    let validator = ManifestValidator::new(gm);
    for node in &fixture.nodes {
        validator.validate_node_kind(&node.kind)?;
        validator.validate_node_properties(&node.kind, &node.properties)?;
        if let Some(node_def) = gm.node_kind(&node.kind) {
            for port in &node.ports {
                validator.validate_port_kind(&port.kind)?;
                if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == &port.kind) {
                    return Err(TrinityRamError::PortKindNotDeclaredOnFixture { node_id: node.id.clone(), port_kind: port.kind.clone(), node_kind: node.kind.clone() });
                }
            }
        }
    }
    for edge in &fixture.edges {
        validator.validate_edge_kind(&edge.kind)?;
        validator.validate_edge_properties(&edge.kind, &edge.properties)?;
    }
    Ok(())
}

/// 🎯️ Entity reference for mutations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "entity", content = "id")]
pub enum EntityRef {
    Node(String),
    Edge(String),
}

/// 🔑️ Parse `nodeId@portId` port key (`@` is the unified syntax's one port sigil — `:` is reserved for typing).
pub fn parse_port_key(key: &str) -> Option<(&str, &str)> {
    let (node, port) = key.split_once('@')?;
    if node.is_empty() || port.is_empty() {
        return None;
    }
    Some((node, port))
}

/// 🧩️ Node id from a port key.
pub fn port_node_id(key: &str) -> Option<&str> {
    parse_port_key(key).map(|(n, _)| n)
}

/// 🔌️ Port id from a port key.
pub fn port_port_id(key: &str) -> Option<&str> {
    parse_port_key(key).map(|(_, p)| p)
}

/// 🏗️ Build a port key.
pub fn port_key(node_id: &str, port_id: &str) -> String {
    format!("{node_id}@{port_id}")
}
// #endregion 🔖️Runtime

// #region 🔖️GraphOperations
use protocol::{Operation, OperationDiff};
use vcs::{apply_operation, CollectionDiff, ItemPatch};
use store::{create_document_envelope, DocumentCommand, DocumentEnvelope, DocumentStore};

pub const TRINITY_GRAPH_SCHEMA: &str = GraphFixture::SCHEMA;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    /// 📦️ Whole-fixture replacement (preset load, node-graph drag import) — the base the rest of the diff layers onto.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_fixture: Option<GraphFixture>,
    pub recompute_derived: bool,
}

impl OperationDiff<GraphFixture> for TrinityGraphDiff {
    fn apply(&self, projection: &GraphFixture) -> GraphFixture {
        let mut next = self.set_fixture.clone().unwrap_or_else(|| projection.clone());
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
        if let Some(fixture) = other.set_fixture {
            self.set_fixture = Some(fixture);
        }
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
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum TrinityGraphOperation {
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
    /// 📦️ Replace the whole fixture (preset load, node-graph drag import); the inverse restores the prior fixture.
    SetFixture {
        fixture: GraphFixture,
    }
}

pub type TrinityGraphEnvelope = DocumentEnvelope<GraphFixture, TrinityGraphOperation>;
pub type TrinityGraphStore = DocumentStore<GraphFixture, TrinityGraphOperation>;

pub fn create_trinity_graph_envelope(id: &str, fixture: GraphFixture) -> TrinityGraphEnvelope {
    create_document_envelope(TRINITY_GRAPH_SCHEMA, id, fixture, None)
}

pub fn validate_trinity_graph_operation(operation: &TrinityGraphOperation, fixture: &GraphFixture) -> Result<(), TrinityRamError> {
    match operation {
        TrinityGraphOperation::CreateNode { id, kind, ports, .. } => {
            if fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(TrinityRamError::NodeAlreadyExists(id.clone()));
            }
            validate_node_kind_trinity(&fixture.manifest, kind)?;
            if let Some(node_def) = fixture.manifest.node_kind(kind) {
                for port in ports {
                    validate_port_kind_trinity(&fixture.manifest, &port.kind)?;
                    if !node_def.port_kinds.is_empty() && !node_def.port_kinds.iter().any(|p| p == &port.kind) {
                        return Err(TrinityRamError::PortKindNotDeclaredOnOperation { node_id: id.clone(), port_id: port.id.clone(), port_kind: port.kind.clone(), node_kind: kind.clone() });
                    }
                }
            }
        }
        TrinityGraphOperation::DeleteNode { id } => {
            if !fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(TrinityRamError::NodeNotFound(id.clone()));
            }
        }
        TrinityGraphOperation::CreateEdge { id, kind, source, target, properties } => {
            if fixture.edges.iter().any(|edge| edge.id == *id) {
                return Err(TrinityRamError::EdgeAlreadyExists(id.clone()));
            }
            validate_edge_kind_trinity(&fixture.manifest, kind)?;
            validate_edge_properties_trinity(&fixture.manifest, kind, properties)?;
            let source_node = port_node_id(source).ok_or_else(|| TrinityRamError::InvalidSourcePortKey(source.clone()))?;
            let target_node = port_node_id(target).ok_or_else(|| TrinityRamError::InvalidTargetPortKey(target.clone()))?;
            if !fixture.nodes.iter().any(|node| node.id == source_node) {
                return Err(TrinityRamError::SourceNodeNotFound(source_node.to_string()));
            }
            if !fixture.nodes.iter().any(|node| node.id == target_node) {
                return Err(TrinityRamError::TargetNodeNotFound(target_node.to_string()));
            }
        }
        TrinityGraphOperation::DeleteEdge { id } => {
            if !fixture.edges.iter().any(|edge| edge.id == *id) {
                return Err(TrinityRamError::EdgeNotFound(id.clone()));
            }
        }
        TrinityGraphOperation::Rename { id, .. } | TrinityGraphOperation::Reposition { id, .. } => {
            if !fixture.nodes.iter().any(|node| node.id == *id) {
                return Err(TrinityRamError::NodeNotFound(id.clone()));
            }
        }
        TrinityGraphOperation::SetDataProperty { entity, key, value } => {
            validate_set_data_property(fixture, entity, key, value)?;
        }
        TrinityGraphOperation::ClearDataProperty { entity, key } => {
            validate_clear_data_property(fixture, entity, key)?;
        }
        TrinityGraphOperation::SetFixture { .. } => {}
    }
    Ok(())
}

pub fn apply_trinity_graph_operations(fixture: GraphFixture, operations: &[TrinityGraphOperation]) -> Result<GraphFixture, TrinityRamError> {
    let mut projection = fixture;
    for operation in operations {
        validate_trinity_graph_operation(operation, &projection)?;
        projection = apply_operation(&projection, operation);
    }
    Ok(projection)
}

pub fn dispatch_trinity_graph_operations(store: &mut TrinityGraphStore, operations: Vec<TrinityGraphOperation>) -> Result<(), TrinityRamError> {
    if operations.is_empty() {
        return Ok(());
    }
    let mut projection = store.projection()?;
    for operation in &operations {
        validate_trinity_graph_operation(operation, &projection)?;
        projection = apply_operation(&projection, operation);
    }
    store.dispatch(DocumentCommand::Apply { operations: operations, description: None }).map_err(TrinityRamError::from)
}

fn validate_clear_data_property(fixture: &GraphFixture, entity: &EntityRef, key: &str) -> Result<(), TrinityRamError> {
    match entity {
        EntityRef::Node(id) => {
            fixture.nodes.iter().find(|node| node.id == *id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
        }
        EntityRef::Edge(id) => {
            fixture.edges.iter().find(|edge| edge.id == *id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
        }
    }
    let _ = key;
    Ok(())
}

fn validate_set_data_property(fixture: &GraphFixture, entity: &EntityRef, key: &str, value: &PropertyValue) -> Result<(), TrinityRamError> {
    let (defs, path_prefix) = match entity {
        EntityRef::Node(id) => {
            let node = fixture.nodes.iter().find(|node| node.id == *id).ok_or_else(|| TrinityRamError::NodeNotFound(id.clone()))?;
            (fixture.manifest.node_kind(&node.kind).map(|def| &def.properties[..]), format!("nodes/{id}/properties/{key}"))
        }
        EntityRef::Edge(id) => {
            let edge = fixture.edges.iter().find(|edge| edge.id == *id).ok_or_else(|| TrinityRamError::EdgeNotFound(id.clone()))?;
            (fixture.manifest.edge_kind(&edge.kind).map(|def| &def.properties[..]), format!("edges/{id}/properties/{key}"))
        }
    };
    let Some(defs) = defs else {
        return Err(TrinityRamError::UnknownEntityKind { path: path_prefix });
    };
    let Some(def) = defs.iter().find(|def| def.name == key) else {
        return Err(TrinityRamError::UnknownPropertyAtPath { path: path_prefix, key: key.to_string() });
    };
    if def.kind == PropertyKind::Derived {
        return Err(TrinityRamError::DerivedPropertyReadonly { path: path_prefix, key: key.to_string() });
    }
    let mut bag = PropertyBag::new();
    bag.insert(key.to_string(), value.clone());
    validate_property_bag_trinity(&path_prefix, defs, &bag)
}

fn validate_node_kind_trinity(manifest: &Manifest, kind: &str) -> Result<(), TrinityRamError> {
    if manifest.node_kind(kind).is_some() {
        Ok(())
    } else {
        Err(TrinityRamError::UnknownNodeKind { kind: kind.to_string() })
    }
}

fn validate_edge_kind_trinity(manifest: &Manifest, kind: &str) -> Result<(), TrinityRamError> {
    if manifest.edge_kind(kind).is_some() {
        Ok(())
    } else {
        Err(TrinityRamError::UnknownEdgeKind { kind: kind.to_string() })
    }
}

fn validate_port_kind_trinity(manifest: &Manifest, kind: &str) -> Result<(), TrinityRamError> {
    if manifest.port_kind(kind).is_some() {
        Ok(())
    } else {
        Err(TrinityRamError::UnknownPortKind { kind: kind.to_string() })
    }
}

fn validate_edge_properties_trinity(manifest: &Manifest, kind: &str, properties: &PropertyBag) -> Result<(), TrinityRamError> {
    let Some(def) = manifest.edge_kind(kind) else {
        return validate_edge_kind_trinity(manifest, kind);
    };
    validate_property_bag_trinity(&format!("edges/{kind}/properties"), &def.properties, properties)
}

fn validate_property_bag_trinity(path: &str, defs: &[PropertyDef], bag: &PropertyBag) -> Result<(), TrinityRamError> {
    for def in defs {
        if def.kind == PropertyKind::Derived {
            continue;
        }
        let Some(value) = bag.get(&def.name) else {
            continue;
        };
        if !property_value_matches_type_trinity(value, def) {
            return Err(TrinityRamError::PropertyTypeMismatch { path: path.to_string(), name: def.name.clone(), value_type: def.value_type.id().to_string() });
        }
    }
    for key in bag.keys() {
        if !defs.iter().any(|def| def.name == *key) {
            return Err(TrinityRamError::UnknownPropertyInBag { path: path.to_string(), key: key.clone() });
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

fn remove_node_from_fixture(fixture: &mut GraphFixture, id: &str) {
    fixture.nodes.retain(|node| node.id != id);
    fixture.edges.retain(|edge| port_node_id(&edge.source) != Some(id) && port_node_id(&edge.target) != Some(id));
    if fixture.root_node_id.as_deref() == Some(id) {
        fixture.root_node_id = None;
    }
}

fn delete_node_snapshot(fixture: &GraphFixture, id: &str) -> (Option<Node>, Vec<Edge>) {
    let node = fixture.nodes.iter().find(|node| node.id == id).cloned();
    let edges: Vec<Edge> = fixture.edges.iter().filter(|edge| port_node_id(&edge.source) == Some(id) || port_node_id(&edge.target) == Some(id)).cloned().collect();
    (node, edges)
}

fn entity_property_value(fixture: &GraphFixture, entity: &EntityRef, key: &str) -> Option<PropertyValue> {
    match entity {
        EntityRef::Node(id) => fixture.nodes.iter().find(|node| node.id == *id).and_then(|node| node.properties.get(key).cloned()),
        EntityRef::Edge(id) => fixture.edges.iter().find(|edge| edge.id == *id).and_then(|edge| edge.properties.get(key).cloned()),
    }
}

impl Operation<GraphFixture> for TrinityGraphOperation {
    type Diff = TrinityGraphDiff;

    fn diff(&self, projection: &GraphFixture) -> TrinityGraphDiff {
        match self {
            TrinityGraphOperation::CreateNode { id, kind, name, x, y, width, height, ports } => TrinityGraphDiff {
                nodes: CollectionDiff { added: vec![Node { id: id.clone(), kind: kind.clone(), name: name.clone(), x: *x, y: *y, width: *width, height: *height, properties: PropertyBag::new(), ports: ports.clone() }], ..Default::default() },
                recompute_derived: true,
                ..Default::default()
            },
            TrinityGraphOperation::DeleteNode { id } => {
                let (node, edges) = delete_node_snapshot(projection, id);
                TrinityGraphDiff {
                    nodes: CollectionDiff { removed: node.as_ref().map(|node| vec![node.id.clone()]).unwrap_or_default(), ..Default::default() },
                    edges: CollectionDiff { removed: edges.iter().map(|edge| edge.id.clone()).collect(), ..Default::default() },
                    recompute_derived: true,
                    ..Default::default()
                }
            }
            TrinityGraphOperation::CreateEdge { id, kind, source, target, properties } => TrinityGraphDiff {
                edges: CollectionDiff { added: vec![Edge { id: id.clone(), kind: kind.clone(), source: source.clone(), target: target.clone(), properties: properties.clone() }], ..Default::default() },
                recompute_derived: true,
                ..Default::default()
            },
            TrinityGraphOperation::DeleteEdge { id } => TrinityGraphDiff { edges: CollectionDiff { removed: vec![id.clone()], ..Default::default() }, recompute_derived: true, ..Default::default() },
            TrinityGraphOperation::Rename { id, name } => {
                TrinityGraphDiff { nodes: CollectionDiff { modified: vec![ItemPatch { id: id.clone(), patch: NodeGeometryPatch { name: Some(name.clone()), ..Default::default() } }], ..Default::default() }, ..Default::default() }
            }
            TrinityGraphOperation::Reposition { id, x, y } => {
                TrinityGraphDiff { nodes: CollectionDiff { modified: vec![ItemPatch { id: id.clone(), patch: NodeGeometryPatch { x: Some(*x), y: Some(*y), ..Default::default() } }], ..Default::default() }, ..Default::default() }
            }
            TrinityGraphOperation::SetDataProperty { entity, key, value } => {
                let patch = PropertyPatch { key: key.clone(), value: Some(value.clone()) };
                let recompute = matches!(entity, EntityRef::Edge(_)) && (key == "u" || key == "v");
                match entity {
                    EntityRef::Node(id) => TrinityGraphDiff { node_properties: vec![ItemPatch { id: id.clone(), patch }], recompute_derived: key == "flatPosition", ..Default::default() },
                    EntityRef::Edge(id) => TrinityGraphDiff { edge_properties: vec![ItemPatch { id: id.clone(), patch }], recompute_derived: recompute, ..Default::default() },
                }
            }
            TrinityGraphOperation::ClearDataProperty { entity, key } => {
                let patch = PropertyPatch { key: key.clone(), value: None };
                match entity {
                    EntityRef::Node(id) => TrinityGraphDiff { node_properties: vec![ItemPatch { id: id.clone(), patch }], ..Default::default() },
                    EntityRef::Edge(id) => TrinityGraphDiff { edge_properties: vec![ItemPatch { id: id.clone(), patch }], recompute_derived: key == "u" || key == "v", ..Default::default() },
                }
            }
            TrinityGraphOperation::SetFixture { fixture } => TrinityGraphDiff { set_fixture: Some(fixture.clone()), recompute_derived: true, ..Default::default() },
        }
    }

    fn backwards(&self, projection: &GraphFixture) -> Vec<Self> {
        match self {
            TrinityGraphOperation::CreateNode { id, .. } => vec![TrinityGraphOperation::DeleteNode { id: id.clone() }],
            TrinityGraphOperation::DeleteNode { id } => {
                let (node, edges) = delete_node_snapshot(projection, id);
                let mut out = Vec::new();
                if let Some(node) = node {
                    out.push(TrinityGraphOperation::CreateNode { id: node.id, kind: node.kind, name: node.name, x: node.x, y: node.y, width: node.width, height: node.height, ports: node.ports });
                    for edge in edges {
                        out.push(TrinityGraphOperation::CreateEdge { id: edge.id, kind: edge.kind, source: edge.source, target: edge.target, properties: edge.properties });
                    }
                }
                out
            }
            TrinityGraphOperation::CreateEdge { id, .. } => vec![TrinityGraphOperation::DeleteEdge { id: id.clone() }],
            TrinityGraphOperation::DeleteEdge { id } => projection
                .edges
                .iter()
                .find(|edge| edge.id == *id)
                .map(|edge| vec![TrinityGraphOperation::CreateEdge { id: edge.id.clone(), kind: edge.kind.clone(), source: edge.source.clone(), target: edge.target.clone(), properties: edge.properties.clone() }])
                .unwrap_or_default(),
            TrinityGraphOperation::Rename { id, .. } => projection.nodes.iter().find(|node| node.id == *id).map(|node| vec![TrinityGraphOperation::Rename { id: id.clone(), name: node.name.clone() }]).unwrap_or_default(),
            TrinityGraphOperation::Reposition { id, .. } => projection.nodes.iter().find(|node| node.id == *id).map(|node| vec![TrinityGraphOperation::Reposition { id: id.clone(), x: node.x, y: node.y }]).unwrap_or_default(),
            TrinityGraphOperation::SetDataProperty { entity, key, .. } => {
                let prior = entity_property_value(projection, entity, key);
                match (entity, prior) {
                    (EntityRef::Node(id), Some(old)) => vec![TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node(id.clone()), key: key.clone(), value: old }],
                    (EntityRef::Edge(id), Some(old)) => vec![TrinityGraphOperation::SetDataProperty { entity: EntityRef::Edge(id.clone()), key: key.clone(), value: old }],
                    (entity, None) => vec![TrinityGraphOperation::ClearDataProperty { entity: entity.clone(), key: key.clone() }],
                }
            }
            TrinityGraphOperation::ClearDataProperty { entity, key } => entity_property_value(projection, entity, key).map(|old| vec![TrinityGraphOperation::SetDataProperty { entity: entity.clone(), key: key.clone(), value: old }]).unwrap_or_default(),
            TrinityGraphOperation::SetFixture { .. } => vec![TrinityGraphOperation::SetFixture { fixture: projection.clone() }],
        }
    }
}

// #endregion 🔖️GraphOperations

//#region 🔖️Dsl
use protocol::OpText;
use store::{DocumentDsl, DocumentPack, PackDecodeOptions, PackEncodeOptions, PackError, TextError, TextSpan};

//#region 🔖️DslMirrors
/// 🔒️ Local twin of `PortDirection` (foreign, re-exported from `mathematical_graph_manifest` and
/// consumed by `trinity_jack`/`semio_s_plugin_trinity`/`framework::*` — this crate does not own the freedom
/// to reshape it) purely so the DSL engine's derive macros have something local to bind: the orphan
/// rule blocks `impl dsl::DslField for PortDirection` directly in this crate. Converted at the
/// `Port`/`PortDsl` boundary via `From`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar)]
enum PortDirectionDsl {
    In,
    Out,
}

impl From<PortDirection> for PortDirectionDsl {
    fn from(value: PortDirection) -> Self {
        match value {
            PortDirection::In => PortDirectionDsl::In,
            PortDirection::Out => PortDirectionDsl::Out,
        }
    }
}

impl From<PortDirectionDsl> for PortDirection {
    fn from(value: PortDirectionDsl) -> Self {
        match value {
            PortDirectionDsl::In => PortDirection::In,
            PortDirectionDsl::Out => PortDirection::Out,
        }
    }
}

/// 🔌️ Local mirror of `Port` for DSL round-tripping — `Port.direction: PortDirection` is foreign, so
/// `Port` itself cannot derive `dsl::DslRecord` (orphan rule); this twin swaps in `PortDirectionDsl`.
/// `properties: PropertyBag` binds directly (no twin needed): `PropertyValue` already implements
/// `dsl::DslField` from the `mathematical_graph_manifest` prep step, so `BTreeMap<String,
/// PropertyValue>` reaches the engine's own blanket `Shape::Map` impl unchanged.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct PortDsl {
    id: String,
    kind: String,
    direction: PortDirectionDsl,
    properties: PropertyBag,
}

fn port_to_port_dsl(port: &Port) -> PortDsl {
    PortDsl { id: port.id.clone(), kind: port.kind.clone(), direction: port.direction.into(), properties: port.properties.clone() }
}

fn port_dsl_to_port(port: PortDsl) -> Port {
    Port { id: port.id, kind: port.kind, direction: port.direction.into(), properties: port.properties }
}

/// 🧩️ Local mirror of `Node` — needed only because `Node.ports: Vec<Port>` transitively carries
/// `Port`'s foreign `direction` field; every other `Node` field is already DSL-ready directly.
/// `ports` is itself `#[dsl(table)]`: `NodeDsl` is the row type of `GraphFixtureDsl.nodes`'s own
/// `#[dsl(table)]` column, so a table-shaped `ports` here is a table-within-a-table-row — the
/// engine now prints/parses that nested case as a braced-row AoS list (`parse_table_list`/
/// `print_table_list` in `dsl_schema`), so this is no longer the engine limitation it used to be.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct NodeDsl {
    id: String,
    kind: String,
    name: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    properties: PropertyBag,
    #[dsl(table)]
    ports: Vec<PortDsl>,
}

fn node_to_node_dsl(node: &Node) -> NodeDsl {
    NodeDsl {
        id: node.id.clone(),
        kind: node.kind.clone(),
        name: node.name.clone(),
        x: node.x,
        y: node.y,
        width: node.width,
        height: node.height,
        properties: node.properties.clone(),
        ports: node.ports.iter().map(port_to_port_dsl).collect(),
    }
}

fn node_dsl_to_node(node: NodeDsl) -> Node {
    Node { id: node.id, kind: node.kind, name: node.name, x: node.x, y: node.y, width: node.width, height: node.height, properties: node.properties, ports: node.ports.into_iter().map(port_dsl_to_port).collect() }
}

/// 📦️ Local mirror of `GraphFixture` for the `.trinity` document DSL. `manifest: Manifest` is
/// deliberately NOT a field here at all — the OLD hand-rolled grammar never round-tripped the full
/// compile-time `Manifest`/`TrinityManifest` schema through text either, only the `manifestId`
/// lookup key (see `GraphFixture::resolve_manifest`), and `Manifest` itself has no `dsl::` derive
/// from the prep step (only `PropertyValue`/`PropertyBag` do) so it would need its own local-twin
/// tree for no behavioral gain. `nodes: Vec<Node>` also can't bind directly (transitively foreign via
/// `Port.direction`), so it becomes `Vec<NodeDsl>` here; `edges`/`camera` bind directly since `Edge`
/// and `Camera` derive `dsl::DslRecord` themselves (no foreign fields of their own).
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "trinity", layout = "lines")]
struct GraphFixtureDsl {
    schema: String,
    name: String,
    manifest_id: Option<String>,
    #[dsl(block)]
    camera: Camera,
    #[dsl(table)]
    nodes: Vec<NodeDsl>,
    #[dsl(table)]
    edges: Vec<Edge>,
    root_node_id: Option<String>,
}

fn graph_fixture_to_dsl(fixture: &GraphFixture) -> GraphFixtureDsl {
    GraphFixtureDsl {
        schema: fixture.schema.clone(),
        name: fixture.name.clone(),
        manifest_id: fixture.manifest_id.clone(),
        camera: fixture.camera.clone(),
        nodes: fixture.nodes.iter().map(node_to_node_dsl).collect(),
        edges: fixture.edges.clone(),
        root_node_id: fixture.root_node_id.clone(),
    }
}

/// 🔁️ Reconstructs the real `manifest` field via `resolve_manifest` (looked up from `manifest_id`),
/// exactly like the OLD `parse_dsl` did — the DSL text never carries the manifest body itself.
fn graph_fixture_dsl_to_graph_fixture(parsed: GraphFixtureDsl) -> Result<GraphFixture, TrinityRamError> {
    let mut fixture = GraphFixture {
        schema: parsed.schema,
        name: parsed.name,
        manifest_id: parsed.manifest_id,
        manifest: Manifest::default(),
        camera: parsed.camera,
        nodes: parsed.nodes.into_iter().map(node_dsl_to_node).collect(),
        edges: parsed.edges,
        root_node_id: parsed.root_node_id,
    };
    fixture.resolve_manifest()?;
    Ok(fixture)
}

/// 🏷️ The `entity` half of `EntityRefDsl` — a plain 2-variant scalar tag (`dsl::DslScalar`, not
/// `DslEnum`): `EntityRefDsl` needs `dsl::DslField` (to bind as an ordinary record field on
/// `TrinityGraphOperationDsl`'s variants), and a `DslRecord` of `{ kind, id }` gets that directly,
/// unlike a tagged-variant `DslEnum` (which only yields `DslVariants`, fit for `#[dsl(statements)]`
/// collections, not a single required field).
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar)]
enum EntityKindDsl {
    Node,
    Edge,
}

/// 🎯️ Local twin of `EntityRef` purely for the DSL engine's tuple-variant limitation: a
/// `#[derive(dsl::DslEnum)]` single-field UNNAMED variant (`Node(String)`) delegates to its inner
/// type's own `Shape::Record` (see `dsl_derive::dsl_variants_codegen`'s doc comment), which panics
/// for a primitive inner type like `String`. `EntityRef` itself keeps its real tuple-variant shape
/// unchanged — it's a public re-exported type `trinity_jack` constructs as `EntityRef::Node(id)`, and
/// this crate does not own the freedom to reshape it — so this flat `{ kind, id }` twin exists
/// solely to give the derive something it can bind, converted at the op-text boundary via `From`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct EntityRefDsl {
    kind: EntityKindDsl,
    id: String,
}

impl From<&EntityRef> for EntityRefDsl {
    fn from(value: &EntityRef) -> Self {
        match value {
            EntityRef::Node(id) => EntityRefDsl { kind: EntityKindDsl::Node, id: id.clone() },
            EntityRef::Edge(id) => EntityRefDsl { kind: EntityKindDsl::Edge, id: id.clone() },
        }
    }
}

impl From<EntityRefDsl> for EntityRef {
    fn from(value: EntityRefDsl) -> Self {
        match value.kind {
            EntityKindDsl::Node => EntityRef::Node(value.id),
            EntityKindDsl::Edge => EntityRef::Edge(value.id),
        }
    }
}

/// ⚡️ Local mirror of `TrinityGraphOperation` for `protocol::OpText` — `entity: EntityRef` and
/// `ports`/`fixture` fields transitively carry the same foreign/tuple-variant shapes handled above,
/// so the real enum can't derive `dsl::DslOps` directly. `fixture: GraphFixture` binds through
/// `GraphFixture`'s own hand-written `dsl::DslField` impl (below) unchanged — a nested `Record`
/// field renders fully inline (no embedded newlines) regardless of the outer document's `Lines`
/// layout, satisfying `OpText::print_op`'s one-line law without any manual escaping.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum TrinityGraphOperationDsl {
    CreateNode { id: String, kind: String, name: String, x: f64, y: f64, width: f64, height: f64, #[dsl(table)] ports: Vec<PortDsl> },
    DeleteNode { id: String },
    CreateEdge { id: String, kind: String, source: String, target: String, properties: PropertyBag },
    DeleteEdge { id: String },
    Rename { id: String, name: String },
    Reposition { id: String, x: f64, y: f64 },
    SetDataProperty { entity: EntityRefDsl, key: String, value: PropertyValue },
    ClearDataProperty { entity: EntityRefDsl, key: String },
    SetFixture { fixture: GraphFixture }
}

fn trinity_graph_operation_to_dsl(operation: &TrinityGraphOperation) -> TrinityGraphOperationDsl {
    match operation {
        TrinityGraphOperation::CreateNode { id, kind, name, x, y, width, height, ports } => {
            TrinityGraphOperationDsl::CreateNode { id: id.clone(), kind: kind.clone(), name: name.clone(), x: *x, y: *y, width: *width, height: *height, ports: ports.iter().map(port_to_port_dsl).collect() }
        }
        TrinityGraphOperation::DeleteNode { id } => TrinityGraphOperationDsl::DeleteNode { id: id.clone() },
        TrinityGraphOperation::CreateEdge { id, kind, source, target, properties } => {
            TrinityGraphOperationDsl::CreateEdge { id: id.clone(), kind: kind.clone(), source: source.clone(), target: target.clone(), properties: properties.clone() }
        }
        TrinityGraphOperation::DeleteEdge { id } => TrinityGraphOperationDsl::DeleteEdge { id: id.clone() },
        TrinityGraphOperation::Rename { id, name } => TrinityGraphOperationDsl::Rename { id: id.clone(), name: name.clone() },
        TrinityGraphOperation::Reposition { id, x, y } => TrinityGraphOperationDsl::Reposition { id: id.clone(), x: *x, y: *y },
        TrinityGraphOperation::SetDataProperty { entity, key, value } => TrinityGraphOperationDsl::SetDataProperty { entity: entity.into(), key: key.clone(), value: value.clone() },
        TrinityGraphOperation::ClearDataProperty { entity, key } => TrinityGraphOperationDsl::ClearDataProperty { entity: entity.into(), key: key.clone() },
        TrinityGraphOperation::SetFixture { fixture } => TrinityGraphOperationDsl::SetFixture { fixture: fixture.clone() },
    }
}

fn trinity_graph_operation_from_dsl(operation: TrinityGraphOperationDsl) -> TrinityGraphOperation {
    match operation {
        TrinityGraphOperationDsl::CreateNode { id, kind, name, x, y, width, height, ports } => {
            TrinityGraphOperation::CreateNode { id, kind, name, x, y, width, height, ports: ports.into_iter().map(port_dsl_to_port).collect() }
        }
        TrinityGraphOperationDsl::DeleteNode { id } => TrinityGraphOperation::DeleteNode { id },
        TrinityGraphOperationDsl::CreateEdge { id, kind, source, target, properties } => TrinityGraphOperation::CreateEdge { id, kind, source, target, properties },
        TrinityGraphOperationDsl::DeleteEdge { id } => TrinityGraphOperation::DeleteEdge { id },
        TrinityGraphOperationDsl::Rename { id, name } => TrinityGraphOperation::Rename { id, name },
        TrinityGraphOperationDsl::Reposition { id, x, y } => TrinityGraphOperation::Reposition { id, x, y },
        TrinityGraphOperationDsl::SetDataProperty { entity, key, value } => TrinityGraphOperation::SetDataProperty { entity: entity.into(), key, value },
        TrinityGraphOperationDsl::ClearDataProperty { entity, key } => TrinityGraphOperation::ClearDataProperty { entity: entity.into(), key },
        TrinityGraphOperationDsl::SetFixture { fixture } => TrinityGraphOperation::SetFixture { fixture },
    }
}
//#endregion 🔖️DslMirrors

//#region 🔖️DslDocument
/// 📜️ `.trinity` textual notation for a whole [`GraphFixture`] (`store::DocumentDsl`), delegating to
/// the derive-generated `GraphFixtureDsl` mirror (see `🔖️DslMirrors`). Also hand-implements
/// `dsl::DslField` (normally auto-emitted alongside `#[derive(dsl::DslDocument)]`) so `GraphFixture`
/// can be nested as an ordinary field too — `TrinityGraphOperation::SetFixture` embeds a whole
/// fixture snapshot.
impl DocumentDsl for GraphFixture {
    const EXTENSION: &'static str = "trinity";

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let parsed = <GraphFixtureDsl as DocumentDsl>::parse_dsl(text)?;
        graph_fixture_dsl_to_graph_fixture(parsed).map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <GraphFixtureDsl as DocumentDsl>::print_dsl(&graph_fixture_to_dsl(self))
    }
}

impl dsl::DslField for GraphFixture {
    fn shape() -> dsl::Shape {
        <GraphFixtureDsl as dsl::DslField>::shape()
    }

    fn to_value(&self) -> dsl::FieldValue {
        <GraphFixtureDsl as dsl::DslField>::to_value(&graph_fixture_to_dsl(self))
    }

    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        let parsed = <GraphFixtureDsl as dsl::DslField>::from_value(value)?;
        graph_fixture_dsl_to_graph_fixture(parsed).map_err(|error| error.to_string())
    }
}
//#endregion 🔖️DslDocument

//#region 🔖️Pack
/// 📦️ Binary pack notation for a whole [`GraphFixture`] (`store::DocumentPack`), hand-implemented
/// exactly like `impl DocumentDsl for GraphFixture` above (`GraphFixture` itself does not derive
/// `dsl::DslDocument`, only the `GraphFixtureDsl` mirror does — see `🔖️DslMirrors`), delegating
/// through the same mirror + `graph_fixture_to_dsl`/`graph_fixture_dsl_to_graph_fixture` pair.
impl DocumentPack for GraphFixture {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        <GraphFixtureDsl as DocumentPack>::encode_pack_with(&graph_fixture_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let parsed = <GraphFixtureDsl as DocumentPack>::decode_pack_with(bytes, options)?;
        graph_fixture_dsl_to_graph_fixture(parsed)
            .map_err(|error| store::text_error_to_pack_error(TextError::new(error.to_string(), TextSpan::at(1, 1))))
    }
}
//#endregion 🔖️Pack
//#endregion 🔖️Dsl

//#region 🔖️OpText
/// ⚡️ One-line textual notation for [`TrinityGraphOperation`] (`protocol::OpText`), delegating to the
/// derive-generated `TrinityGraphOperationDsl` mirror (see `🔖️DslMirrors`).
impl OpText for TrinityGraphOperation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        <TrinityGraphOperationDsl as OpText>::parse_op(line).map(trinity_graph_operation_from_dsl)
    }

    fn print_op(&self) -> String {
        <TrinityGraphOperationDsl as OpText>::print_op(&trinity_graph_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `TrinityGraphOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for TrinityGraphOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        trinity_graph_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        TrinityGraphOperationDsl::decode_op(bytes).map(trinity_graph_operation_from_dsl)
    }
}
//#endregion 🔖️OpText

pub fn empty_trinity_graph_fixture() -> GraphFixture {
    GraphFixture { schema: GraphFixture::SCHEMA.into(), name: "trinity".into(), manifest_id: Some("nakagin".into()), manifest: Manifest::nakagin_default(), camera: Camera::default(), nodes: Vec::new(), edges: Vec::new(), root_node_id: None }
}

//#region 🔖️WasmBridge
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
                    let envelope: TrinityGraphEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    TrinityGraphStore::new(envelope)
                }
                None => TrinityGraphStore::new(create_trinity_graph_envelope("trinity", empty_trinity_graph_fixture())),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖️WasmBridge

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn mini_fixture() -> GraphFixture {
        GraphFixture {
            schema: GraphFixture::SCHEMA.into(),
            name: "mini".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
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
                source: "root@out-a".into(),
                target: "child@in-a".into(),
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
        let json = r#"{"schema":"trinity.graph","name":"mini","manifestId":"nakagin","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
        let graph = Graph::load_json(json).unwrap();
        assert!(graph.manifest.node_kind("Piece").is_some());
    }

    #[test]
    fn fixture_round_trip() {
        let fixture = mini_fixture();
        let json = fixture.to_json().unwrap();
        let back = GraphFixture::from_json(&json).unwrap();
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
        let fixture = GraphFixture {
            schema: GraphFixture::SCHEMA.into(),
            name: "disconnected".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
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
                    source: "root-a@out".into(),
                    target: "child-a@in".into(),
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
                    source: "root-b@out".into(),
                    target: "child-b@in".into(),
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
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::CreateNode { id: "new".into(), kind: "Piece".into(), name: "new-piece".into(), x: 200.0, y: 40.0, width: 80.0, height: 40.0, ports: vec![] }]).expect("create");
        assert_eq!(store.projection().expect("projection").nodes.len(), 3);
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").nodes.len(), 2);
    }

    #[test]
    fn graph_op_dispatch_validates_create_edge_batch_incrementally() {
        let mut fixture = mini_fixture();
        while fixture.nodes.len() < 9 {
            fixture.nodes.push(Node { id: format!("pad-{}", fixture.nodes.len()), kind: "Piece".into(), name: format!("pad-{}", fixture.nodes.len()), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: vec![] });
        }
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", fixture));
        dispatch_trinity_graph_operations(
            &mut store,
            vec![
                TrinityGraphOperation::CreateNode {
                    id: "x-9".into(),
                    kind: "Piece".into(),
                    name: "x".into(),
                    x: 1080.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                TrinityGraphOperation::CreateNode {
                    id: "y-10".into(),
                    kind: "Piece".into(),
                    name: "y".into(),
                    x: 1200.0,
                    y: 80.0,
                    width: 80.0,
                    height: 40.0,
                    ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
                TrinityGraphOperation::CreateEdge { id: "e-batch".into(), kind: "Connection".into(), source: port_key("x-9", "out"), target: port_key("y-10", "in"), properties: PropertyBag::new() },
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
        let err = validate_trinity_graph_operation(&TrinityGraphOperation::CreateNode { id: "new".into(), kind: "Piece2".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, ports: vec![] }, &fixture).expect_err("unknown kind");
        assert!(err.to_string().contains("unknown node kind"));
    }

    #[test]
    fn graph_op_rejects_derived_property_set() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "flatPosition".into(), value: PropertyValue::Null }, &fixture).expect_err("derived");
        assert!(err.to_string().contains("derived"));
    }

    //#region 🔖️DslTests
    use store::test_support::{assert_dsl_pack_equivalence, assert_dsl_round_trip, assert_document_pack_round_trip, assert_document_text_round_trip, assert_op_line_round_trip};

    #[test]
    fn dsl_round_trip_mini_fixture() {
        assert_dsl_round_trip(&mini_fixture());
        assert_dsl_pack_equivalence(&mini_fixture());
    }

    #[test]
    fn dsl_round_trip_nakagin_fixture() {
        let fixture = GraphFixture::parse_dsl(include_str!("../../../../../../../✏️s/🔌️plugin/🔱️trinity/📚️example/🔱️nakagin-capsule-tower.trinity")).expect("nakagin fixture parses");
        assert_dsl_round_trip(&fixture);
        assert_dsl_pack_equivalence(&fixture);
    }

    #[test]
    fn dsl_round_trip_branch_chain_fixture() {
        let fixture = GraphFixture::parse_dsl(include_str!("../../../../../../../✏️s/🔌️plugin/🔱️trinity/📚️example/🔱️branch-chain.trinity")).expect("branch-chain fixture parses");
        assert_dsl_round_trip(&fixture);
        assert_dsl_pack_equivalence(&fixture);
    }

    #[test]
    fn op_text_round_trip_create_node() {
        assert_op_line_round_trip(&TrinityGraphOperation::CreateNode {
            id: "new".into(),
            kind: "Piece".into(),
            name: "new-piece".into(),
            x: 200.0,
            y: 40.0,
            width: 80.0,
            height: 40.0,
            ports: vec![Port { id: "p1".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
        });
    }

    #[test]
    fn op_text_round_trip_delete_node() {
        assert_op_line_round_trip(&TrinityGraphOperation::DeleteNode { id: "root".into() });
    }

    #[test]
    fn op_text_round_trip_create_edge() {
        let mut properties = PropertyBag::new();
        properties.insert("u".into(), PropertyValue::Number(1.2));
        let mut nested = BTreeMap::new();
        nested.insert("x".into(), PropertyValue::Number(0.0));
        properties.insert("meta".into(), PropertyValue::Object(nested));
        assert_op_line_round_trip(&TrinityGraphOperation::CreateEdge { id: "e2".into(), kind: "Connection".into(), source: port_key("root", "out-a"), target: port_key("child", "in-a"), properties });
    }

    #[test]
    fn op_text_round_trip_delete_edge() {
        assert_op_line_round_trip(&TrinityGraphOperation::DeleteEdge { id: "e1".into() });
    }

    #[test]
    fn op_text_round_trip_rename() {
        assert_op_line_round_trip(&TrinityGraphOperation::Rename { id: "root".into(), name: "renamed \"piece\"".into() });
    }

    #[test]
    fn op_text_round_trip_reposition() {
        assert_op_line_round_trip(&TrinityGraphOperation::Reposition { id: "root".into(), x: 10.0, y: -20.5 });
    }

    #[test]
    fn op_text_round_trip_set_data_property() {
        assert_op_line_round_trip(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("hi 'there'".into()) });
    }

    #[test]
    fn op_text_round_trip_clear_data_property() {
        assert_op_line_round_trip(&TrinityGraphOperation::ClearDataProperty { entity: EntityRef::Edge("e1".into()), key: "u".into() });
    }

    #[test]
    fn op_text_round_trip_set_fixture() {
        assert_op_line_round_trip(&TrinityGraphOperation::SetFixture { fixture: mini_fixture() });
    }

    #[test]
    fn document_text_round_trip_graph_store() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::Rename { id: "root".into(), name: "renamed".into() }]).expect("apply");
        assert_document_text_round_trip(&store);
        assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖️DslTests

    //#region 🔖️SchemaAndManifestTests
    #[test]
    fn from_json_rejects_wrong_schema() {
        let json = r#"{"schema":"bogus","name":"x","camera":{"x":0,"y":0,"zoom":1},"nodes":[],"edges":[]}"#;
        let err = GraphFixture::from_json(json).expect_err("schema mismatch");
        assert!(err.to_string().contains("expected schema trinity.graph"));
    }

    #[test]
    fn resolve_manifest_errors_when_missing_and_empty() {
        let mut fixture = GraphFixture { schema: GraphFixture::SCHEMA.into(), name: "x".into(), manifest_id: None, manifest: Manifest::default(), camera: Camera::default(), nodes: vec![], edges: vec![], root_node_id: None };
        let err = fixture.resolve_manifest().expect_err("missing manifest");
        assert!(matches!(err, TrinityRamError::ManifestMissing));
    }

    #[test]
    fn resolve_manifest_errors_on_unknown_id() {
        let mut fixture = GraphFixture { schema: GraphFixture::SCHEMA.into(), name: "x".into(), manifest_id: Some("nope".into()), manifest: Manifest::default(), camera: Camera::default(), nodes: vec![], edges: vec![], root_node_id: None };
        let err = fixture.resolve_manifest().expect_err("unknown manifest id");
        assert!(err.to_string().contains("unknown manifest id nope"));
    }

    #[test]
    fn graph_from_fixture_rejects_port_kind_not_declared_on_node_kind() {
        let mut fixture = mini_fixture();
        fixture.nodes[0].ports.push(Port { id: "bad".into(), kind: "core circular bottom".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
        let err = Graph::from_fixture(fixture).expect_err("undeclared port kind");
        assert!(matches!(err, TrinityRamError::PortKindNotDeclaredOnFixture { .. }));
        assert!(err.to_string().contains("root"));
    }
    //#endregion 🔖️SchemaAndManifestTests

    //#region 🔖️GraphAccessorTests
    #[test]
    fn graph_accessors_and_mutators() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        assert!(g.node("root").is_some());
        assert!(g.node("ghost").is_none());
        assert!(g.edge("e1").is_some());
        g.node_mut("root").unwrap().name = "renamed".into();
        assert_eq!(g.node("root").unwrap().name, "renamed");

        g.add_node(Node { id: "extra".into(), kind: "Piece".into(), name: "extra".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, properties: PropertyBag::new(), ports: vec![] });
        assert!(g.node("extra").is_some());

        g.add_edge(Edge { id: "e2".into(), kind: "Connection".into(), source: "root@out-a".into(), target: "extra@in-a".into(), properties: PropertyBag::new() });
        assert!(g.edge("e2").is_some());
        assert!(g.remove_edge("e2"));
        assert!(!g.remove_edge("e2"));
    }

    #[test]
    fn graph_remove_node_clears_root_node_id() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        assert!(g.remove_node("root"));
        assert!(g.edges.is_empty());
        assert!(g.nodes.contains_key("child"));
        assert!(g.root_node_id.is_none());
        assert!(!g.remove_node("root"));
    }

    #[test]
    fn graph_set_property_success_and_errors() {
        let mut g = Graph::from_fixture(mini_fixture()).unwrap();
        g.set_property(EntityRef::Node("root".into()), "label", PropertyValue::String("hi".into())).expect("set node prop");
        assert_eq!(g.node("root").unwrap().properties.get("label"), Some(&PropertyValue::String("hi".into())));
        let err = g.set_property(EntityRef::Node("ghost".into()), "label", PropertyValue::Null).expect_err("missing node");
        assert!(matches!(err, TrinityRamError::NodeNotFound(_)));

        g.set_property(EntityRef::Edge("e1".into()), "gap", PropertyValue::Number(1.0)).expect("set edge prop");
        assert_eq!(g.edge("e1").unwrap().properties.get("gap"), Some(&PropertyValue::Number(1.0)));
        let err = g.set_property(EntityRef::Edge("ghost".into()), "gap", PropertyValue::Null).expect_err("missing edge");
        assert!(matches!(err, TrinityRamError::EdgeNotFound(_)));
    }

    #[test]
    fn graph_to_fixture_and_fixture_json() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let fixture = g.to_fixture();
        assert_eq!(fixture.nodes.len(), 2);
        assert_eq!(fixture.manifest_id.as_deref(), Some("nakagin"));
        let json = g.fixture_json().expect("fixture json");
        assert!(json.contains("\"schema\""));
    }

    #[test]
    fn subgraph_fixture_filters_entities_and_keeps_root_when_included() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let node_ids: BTreeSet<String> = ["root".to_string()].into_iter().collect();
        let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
        assert_eq!(sub.nodes.len(), 1);
        assert!(sub.edges.is_empty());
        assert_eq!(sub.root_node_id.as_deref(), Some("root"));
        assert!(sub.name.contains("subgraph"));
    }

    #[test]
    fn subgraph_fixture_drops_root_when_not_included() {
        let g = Graph::from_fixture(mini_fixture()).unwrap();
        let node_ids: BTreeSet<String> = ["child".to_string()].into_iter().collect();
        let sub = g.subgraph_fixture(&node_ids, &BTreeSet::new());
        assert!(sub.root_node_id.is_none());
    }

    #[test]
    fn recompute_derived_noop_on_empty_graph() {
        let mut g = Graph::from_fixture(empty_trinity_graph_fixture()).unwrap();
        g.recompute_derived();
        assert!(g.nodes.is_empty());
    }

    #[test]
    fn derived_flat_position_handles_cycles_without_looping() {
        let fixture = GraphFixture {
            schema: GraphFixture::SCHEMA.into(),
            name: "cycle".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
            root_node_id: Some("a".into()),
            nodes: vec![
                Node { id: "a".into(), kind: "Piece".into(), name: "a".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, properties: PropertyBag::new(), ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }] },
                Node { id: "b".into(), kind: "Piece".into(), name: "b".into(), x: 0.0, y: 0.0, width: 10.0, height: 10.0, properties: PropertyBag::new(), ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }] },
            ],
            edges: vec![
                Edge { id: "ab".into(), kind: "Connection".into(), source: "a@out".into(), target: "b@out".into(), properties: { let mut p = PropertyBag::new(); p.insert("u".into(), PropertyValue::Number(1.0)); p.insert("v".into(), PropertyValue::Number(0.0)); p } },
                Edge { id: "ba".into(), kind: "Connection".into(), source: "b@out".into(), target: "a@out".into(), properties: PropertyBag::new() },
            ],
        };
        let mut g = Graph::from_fixture(fixture).unwrap();
        g.recompute_derived();
        assert!(g.node("a").unwrap().properties.get("flatPosition").is_some());
        assert!(g.node("b").unwrap().properties.get("flatPosition").is_some());
    }

    #[test]
    fn port_key_helpers_handle_malformed_keys() {
        assert_eq!(parse_port_key("node@port"), Some(("node", "port")));
        assert_eq!(parse_port_key("noport"), None);
        assert_eq!(parse_port_key("@port"), None);
        assert_eq!(parse_port_key("node@"), None);
        assert_eq!(port_node_id("node@port"), Some("node"));
        assert_eq!(port_port_id("node@port"), Some("port"));
        assert_eq!(port_key("a", "b"), "a@b");
    }
    //#endregion 🔖️GraphAccessorTests

    //#region 🔖️GraphOperationValidationTests
    #[test]
    fn graph_op_rejects_port_kind_not_declared_on_operation() {
        let mut fixture = mini_fixture();
        // 🔀️ Nakagin's trinity-projected manifest only resolves a direction for `Connector`, so it is
        // the sole valid trinity port kind there; a second directioned port kind is hand-crafted here
        // to exercise the "known port kind, but not declared on this node kind" branch.
        fixture.manifest = TrinityManifest {
            node_kinds: vec![mathematical_graph_manifest::TrinityNodeKindDef { name: "Piece".into(), properties: vec![], port_kinds: vec!["Connector".into()] }],
            edge_kinds: vec![mathematical_graph_manifest::TrinityEdgeKindDef { name: "Connection".into(), properties: vec![] }],
            port_kinds: vec![
                mathematical_graph_manifest::TrinityPortKindDef { name: "Connector".into(), direction: PortDirection::Out, properties: vec![] },
                mathematical_graph_manifest::TrinityPortKindDef { name: "Other".into(), direction: PortDirection::In, properties: vec![] },
            ],
        };
        let op = TrinityGraphOperation::CreateNode { id: "new".into(), kind: "Piece".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, ports: vec![Port { id: "p".into(), kind: "Other".into(), direction: PortDirection::In, properties: PropertyBag::new() }] };
        let err = validate_trinity_graph_operation(&op, &fixture).expect_err("bad port kind");
        assert!(matches!(err, TrinityRamError::PortKindNotDeclaredOnOperation { .. }));
    }

    #[test]
    fn graph_op_create_edge_rejects_invalid_port_keys() {
        let fixture = mini_fixture();
        let bad_source = TrinityGraphOperation::CreateEdge { id: "e2".into(), kind: "Connection".into(), source: "noAt".into(), target: port_key("child", "in-a"), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&bad_source, &fixture), Err(TrinityRamError::InvalidSourcePortKey(_))));
        let bad_target = TrinityGraphOperation::CreateEdge { id: "e3".into(), kind: "Connection".into(), source: port_key("root", "out-a"), target: "noAt".into(), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&bad_target, &fixture), Err(TrinityRamError::InvalidTargetPortKey(_))));
    }

    #[test]
    fn graph_op_create_edge_rejects_missing_source_and_target_nodes() {
        let fixture = mini_fixture();
        let missing_source = TrinityGraphOperation::CreateEdge { id: "e2".into(), kind: "Connection".into(), source: port_key("ghost", "out"), target: port_key("child", "in-a"), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&missing_source, &fixture), Err(TrinityRamError::SourceNodeNotFound(_))));
        let missing_target = TrinityGraphOperation::CreateEdge { id: "e3".into(), kind: "Connection".into(), source: port_key("root", "out-a"), target: port_key("ghost", "in"), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&missing_target, &fixture), Err(TrinityRamError::TargetNodeNotFound(_))));
    }

    #[test]
    fn graph_op_rejects_duplicate_node_and_edge_ids() {
        let fixture = mini_fixture();
        let dup_node = TrinityGraphOperation::CreateNode { id: "root".into(), kind: "Piece".into(), name: "x".into(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, ports: vec![] };
        assert!(matches!(validate_trinity_graph_operation(&dup_node, &fixture), Err(TrinityRamError::NodeAlreadyExists(_))));
        let dup_edge = TrinityGraphOperation::CreateEdge { id: "e1".into(), kind: "Connection".into(), source: port_key("root", "out-a"), target: port_key("child", "in-a"), properties: PropertyBag::new() };
        assert!(matches!(validate_trinity_graph_operation(&dup_edge, &fixture), Err(TrinityRamError::EdgeAlreadyExists(_))));
    }

    #[test]
    fn graph_op_rejects_missing_entities_on_delete_rename_reposition() {
        let fixture = mini_fixture();
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::DeleteNode { id: "ghost".into() }, &fixture), Err(TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::DeleteEdge { id: "ghost".into() }, &fixture), Err(TrinityRamError::EdgeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::Rename { id: "ghost".into(), name: "x".into() }, &fixture), Err(TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::Reposition { id: "ghost".into(), x: 0.0, y: 0.0 }, &fixture), Err(TrinityRamError::NodeNotFound(_))));
    }

    #[test]
    fn graph_op_set_data_property_rejects_unknown_entity_kind() {
        let mut fixture = mini_fixture();
        fixture.nodes[0].kind = "Ghost".into();
        let err = validate_trinity_graph_operation(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("x".into()) }, &fixture).expect_err("unknown entity kind");
        assert!(matches!(err, TrinityRamError::UnknownEntityKind { .. }));
    }

    #[test]
    fn graph_op_set_data_property_rejects_unknown_property_key() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "bogus".into(), value: PropertyValue::Null }, &fixture).expect_err("unknown key");
        assert!(matches!(err, TrinityRamError::UnknownPropertyAtPath { .. }));
    }

    #[test]
    fn graph_op_set_data_property_rejects_type_mismatch() {
        let fixture = mini_fixture();
        let err = validate_trinity_graph_operation(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::Number(1.0) }, &fixture).expect_err("type mismatch");
        assert!(matches!(err, TrinityRamError::PropertyTypeMismatch { .. }));
    }

    #[test]
    fn graph_op_clear_data_property_rejects_missing_entities() {
        let fixture = mini_fixture();
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::ClearDataProperty { entity: EntityRef::Node("ghost".into()), key: "label".into() }, &fixture), Err(TrinityRamError::NodeNotFound(_))));
        assert!(matches!(validate_trinity_graph_operation(&TrinityGraphOperation::ClearDataProperty { entity: EntityRef::Edge("ghost".into()), key: "u".into() }, &fixture), Err(TrinityRamError::EdgeNotFound(_))));
    }

    #[test]
    fn apply_trinity_graph_operations_applies_valid_sequence_and_rejects_invalid() {
        let fixture = mini_fixture();
        let ok = apply_trinity_graph_operations(fixture.clone(), &[TrinityGraphOperation::Rename { id: "root".into(), name: "renamed".into() }]).expect("rename applies");
        assert_eq!(ok.nodes.iter().find(|n| n.id == "root").unwrap().name, "renamed");

        let err = apply_trinity_graph_operations(fixture, &[TrinityGraphOperation::DeleteNode { id: "ghost".into() }]).expect_err("missing node");
        assert!(matches!(err, TrinityRamError::NodeNotFound(_)));
    }

    #[test]
    fn dispatch_trinity_graph_operations_noop_on_empty() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        let generation_before = store.generation();
        dispatch_trinity_graph_operations(&mut store, vec![]).expect("empty ops ok");
        assert_eq!(store.generation(), generation_before);
    }
    //#endregion 🔖️GraphOperationValidationTests

    //#region 🔖️GraphOperationUndoTests
    #[test]
    fn graph_op_reposition_and_rename_undo_restore_prior_values() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::Reposition { id: "root".into(), x: 50.0, y: 60.0 }]).expect("reposition");
        assert_eq!(store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().x, 50.0);
        store.dispatch(DocumentCommand::Undo).expect("undo reposition");
        assert_eq!(store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().x, 0.0);

        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::Rename { id: "root".into(), name: "renamed".into() }]).expect("rename");
        store.dispatch(DocumentCommand::Undo).expect("undo rename");
        assert_eq!(store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().name, "core");
    }

    #[test]
    fn graph_op_delete_edge_undo_recreates_edge() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::DeleteEdge { id: "e1".into() }]).expect("delete edge");
        assert!(store.projection().unwrap().edges.is_empty());
        store.dispatch(DocumentCommand::Undo).expect("undo delete edge");
        assert_eq!(store.projection().unwrap().edges.len(), 1);
    }

    #[test]
    fn graph_op_delete_node_undo_restores_node_and_incident_edges() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::DeleteNode { id: "root".into() }]).expect("delete node");
        let projection = store.projection().unwrap();
        assert_eq!(projection.nodes.len(), 1);
        assert!(projection.edges.is_empty());
        store.dispatch(DocumentCommand::Undo).expect("undo delete node");
        let projection = store.projection().unwrap();
        assert_eq!(projection.nodes.len(), 2);
        assert_eq!(projection.edges.len(), 1);
    }

    #[test]
    fn graph_op_set_and_clear_data_property_undo_round_trip() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("first".into()) }]).expect("set");
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("second".into()) }]).expect("set again");
        store.dispatch(DocumentCommand::Undo).expect("undo second set");
        let value = store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.get("label").cloned();
        assert_eq!(value, Some(PropertyValue::String("first".into())));

        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::ClearDataProperty { entity: EntityRef::Node("root".into()), key: "label".into() }]).expect("clear");
        assert!(store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.get("label").is_none());
        store.dispatch(DocumentCommand::Undo).expect("undo clear");
        let value = store.projection().unwrap().nodes.iter().find(|n| n.id == "root").unwrap().properties.get("label").cloned();
        assert_eq!(value, Some(PropertyValue::String("first".into())));
    }

    /// 🌱️ `camera` is now a seed-only field on `GraphFixture` (never touched by any operation — see
    /// `nodeGraphViewport`'s runtime-only handling in the jack/rewrite apps), so this only exercises
    /// `SetFixture`'s undo; it no longer asserts camera-as-a-document-operation behavior.
    #[test]
    fn graph_op_set_fixture_undo() {
        let mut store = TrinityGraphStore::new(create_trinity_graph_envelope("test", mini_fixture()));
        assert_eq!(store.projection().unwrap().camera, Camera::default());

        let replacement = GraphFixture { name: "replacement".into(), ..mini_fixture() };
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::SetFixture { fixture: replacement }]).expect("set fixture");
        assert_eq!(store.projection().unwrap().name, "replacement");
        store.dispatch(DocumentCommand::Undo).expect("undo set fixture");
        assert_eq!(store.projection().unwrap().name, "mini");
    }
    //#endregion 🔖️GraphOperationUndoTests

    //#region 🔖️TrinityGraphDiffTests
    #[test]
    fn trinity_graph_diff_absorb_merges_fields() {
        let mut diff = TrinityGraphDiff { recompute_derived: false, ..Default::default() };
        let other = TrinityGraphDiff {
            recompute_derived: true,
            nodes: CollectionDiff { added: vec![Node { id: "x".into(), kind: "Piece".into(), name: "x".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, properties: PropertyBag::new(), ports: vec![] }], ..Default::default() },
            ..Default::default()
        };
        diff.absorb(other);
        assert!(diff.recompute_derived);
        assert_eq!(diff.nodes.added.len(), 1);
    }

    #[test]
    fn trinity_graph_diff_apply_uses_set_fixture_as_base_and_recomputes() {
        let base = mini_fixture();
        let mut replacement = base.clone();
        replacement.name = "swapped".into();
        let diff = TrinityGraphDiff { set_fixture: Some(replacement), recompute_derived: true, ..Default::default() };
        let applied = diff.apply(&base);
        assert_eq!(applied.name, "swapped");
        assert!(applied.nodes.iter().any(|n| n.properties.contains_key("flatPosition")));
    }
    //#endregion 🔖️TrinityGraphDiffTests

    //#region 🔖️DslInternalsTests
    #[test]
    fn parse_dsl_rejects_unknown_keyword() {
        // 🔀️ The `dsl::` derive engine parses `GraphFixtureDsl` as a structured `key=value` record
        // (see `🔖️DslMirrors`), not a line-by-line bare-keyword dispatch like the OLD hand-rolled
        // grammar — so garbage input now fails with a field-shape mismatch instead of an "unknown
        // dsl line keyword" message. Still asserts the same underlying contract: malformed text is
        // rejected, not silently accepted.
        let err = GraphFixture::parse_dsl("bogus line").expect_err("unknown keyword");
        assert!(err.message.contains("expected"));
    }

    #[test]
    fn parse_op_rejects_unknown_keyword() {
        let err = TrinityGraphOperation::parse_op("bogusOp x").expect_err("unknown op");
        assert!(err.message.contains("unknown operation line"));
    }
    //#endregion 🔖️DslInternalsTests
}
// #endregion 🔖️Tests
