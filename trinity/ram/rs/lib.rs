//! 🔺 In-memory trinity directed property port graph with compile-time manifest.

use mathematical_graph_manifest::{manifest_by_id, GraphManifest, ManifestValidationError, TrinityManifest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub use mathematical_graph_manifest::{ManifestValidator, PortDirection, PropertyBag, PropertyDef, PropertyKind, PropertyValue};

/// 📜 Compile-time trinity manifest (projection of {@link GraphManifest}).
pub type Manifest = TrinityManifest;

//#region ⚠️ Errors
/// ⚠️ Trinity graph fixture, manifest-validation, and mutation errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityRamError {
    /// 🧬 JSON (de)serialization failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// 🧭 VCS store/dispatch failure.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
    /// 📜 Compile-time manifest validation failure (path-qualified).
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
    UnknownPropertyInBag { path: String, key: String },
}

/// 🔀 [`ManifestValidationError`] carries no `std::error::Error` impl of its own (plain path/message struct), so this is a manual conversion rather than `#[from]`.
impl From<ManifestValidationError> for TrinityRamError {
    fn from(error: ManifestValidationError) -> Self {
        Self::Manifest(error)
    }
}
//#endregion ⚠️ Errors

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

/// 📦 `trinity.graph` fixture document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphFixture {
    pub schema: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[serde(default)]
    pub manifest: Manifest,
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

/// 🧠 In-memory trinity graph.
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

    /// 🧩 Build a `trinity.graph` fixture containing only the given node and edge ids.
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

// #region 🔖GraphOperations
use vcs::{apply_operation, create_document_vcs_envelope, CollectionDiff, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, ItemPatch, Operation, OperationDiff};

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
    /// 📷 Last-write-wins camera replacement (viewport pan/zoom), independent of the node/edge diff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Camera>,
    /// 📦 Whole-fixture replacement (preset load, node-graph drag import) — the base the rest of the diff layers onto.
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
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
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
        if other.camera.is_some() {
            self.camera = other.camera;
        }
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
    /// 📷 Replace the viewport camera (pan/zoom) — coalesced so a drag is a single undo step.
    SetCamera {
        camera: Camera,
    },
    /// 📦 Replace the whole fixture (preset load, node-graph drag import); the inverse restores the prior fixture.
    SetFixture {
        fixture: GraphFixture,
    },
}

pub type TrinityGraphEnvelope = DocumentVcsEnvelope<GraphFixture, TrinityGraphOperation>;
pub type TrinityGraphStore = DocumentVcsStore<GraphFixture, TrinityGraphOperation>;

pub fn create_trinity_graph_envelope(id: &str, fixture: GraphFixture) -> TrinityGraphEnvelope {
    create_document_vcs_envelope(TRINITY_GRAPH_SCHEMA, id, fixture, None)
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
        TrinityGraphOperation::SetCamera { .. } | TrinityGraphOperation::SetFixture { .. } => {}
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
    store.dispatch(DocumentVcsCommand::Apply { operations: operations, description: None }).map_err(TrinityRamError::from)
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
            TrinityGraphOperation::SetCamera { camera } => TrinityGraphDiff { camera: Some(camera.clone()), ..Default::default() },
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
            TrinityGraphOperation::SetCamera { .. } => vec![TrinityGraphOperation::SetCamera { camera: projection.camera.clone() }],
            TrinityGraphOperation::SetFixture { .. } => vec![TrinityGraphOperation::SetFixture { fixture: projection.clone() }],
        }
    }
}

// #endregion 🔖GraphOperations

//#region 🔖Dsl
use vcs::{DocumentDsl, OpText, TextError, TextSpan};

//#region 🔖DslLexer
/// 🔤 One token of the hand-rolled `.trinity` DSL / op-text lexer, shared by `🔖Dsl` and `🔖OpText`.
#[derive(Clone, Debug, PartialEq)]
enum DslTok {
    Ident(String),
    Number(f64),
    Str(String),
    Colon,
    At,
    Arrow,
    Comma,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Eof,
}

/// 🔎 Tokenizes one line. UUID-shaped ids embed `-` (e.g. `7dc5b737-3b6b-...`), so `-` only starts a
/// number (next char a digit) or the `->` arrow (next char `>`) at a token boundary; inside an already-
/// started identifier it is folded in as long as it is not immediately followed by `>`.
fn dsl_lex(line: &str, line_no: u32) -> Result<Vec<DslTok>, TextError> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        match c {
            b':' => {
                out.push(DslTok::Colon);
                i += 1;
            }
            b'@' => {
                out.push(DslTok::At);
                i += 1;
            }
            b',' => {
                out.push(DslTok::Comma);
                i += 1;
            }
            b'{' => {
                out.push(DslTok::LBrace);
                i += 1;
            }
            b'}' => {
                out.push(DslTok::RBrace);
                i += 1;
            }
            b'[' => {
                out.push(DslTok::LBracket);
                i += 1;
            }
            b']' => {
                out.push(DslTok::RBracket);
                i += 1;
            }
            b'-' if i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                out.push(DslTok::Arrow);
                i += 2;
            }
            b'-' if i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() => {
                let start = i;
                i += 1;
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                    i += 1;
                }
                let text = std::str::from_utf8(&bytes[start..i]).expect("ascii digits");
                let n: f64 = text.parse().map_err(|_| TextError::new(format!("invalid number '{text}'"), TextSpan::at(line_no, start as u32 + 1)))?;
                out.push(DslTok::Number(n));
            }
            b'\'' | b'"' => {
                let quote = c;
                i += 1;
                let mut text = String::new();
                loop {
                    if i >= bytes.len() {
                        return Err(TextError::new("unterminated string", TextSpan::at(line_no, i as u32 + 1)));
                    }
                    if bytes[i] == b'\\' && i + 1 < bytes.len() {
                        match bytes[i + 1] {
                            b'n' => text.push('\n'),
                            b'\\' => text.push('\\'),
                            other if other == quote => text.push(quote as char),
                            other => {
                                text.push('\\');
                                text.push(other as char);
                            }
                        }
                        i += 2;
                        continue;
                    }
                    if bytes[i] == quote {
                        i += 1;
                        break;
                    }
                    let rest = std::str::from_utf8(&bytes[i..]).unwrap_or("");
                    let ch = rest.chars().next().unwrap_or('\u{FFFD}');
                    text.push(ch);
                    i += ch.len_utf8();
                }
                out.push(DslTok::Str(text));
            }
            b'0'..=b'9' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                    i += 1;
                }
                let text = std::str::from_utf8(&bytes[start..i]).expect("ascii digits");
                let n: f64 = text.parse().map_err(|_| TextError::new(format!("invalid number '{text}'"), TextSpan::at(line_no, start as u32 + 1)))?;
                out.push(DslTok::Number(n));
            }
            _ => {
                let start = i;
                loop {
                    if i >= bytes.len() {
                        break;
                    }
                    let ch = bytes[i];
                    if ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'.' {
                        i += 1;
                        continue;
                    }
                    if ch == b'-' && i + 1 < bytes.len() && bytes[i + 1] != b'>' && (bytes[i + 1].is_ascii_alphanumeric() || bytes[i + 1] == b'_') {
                        i += 1;
                        continue;
                    }
                    break;
                }
                if i == start {
                    return Err(TextError::new(format!("unexpected character '{}'", c as char), TextSpan::at(line_no, start as u32 + 1)));
                }
                let text = std::str::from_utf8(&bytes[start..i]).expect("scanned bytes are ascii ident chars").to_string();
                out.push(DslTok::Ident(text));
            }
        }
    }
    out.push(DslTok::Eof);
    Ok(out)
}

/// 🧭 Cursor over a lexed line's tokens for the hand-rolled recursive-descent DSL/op-text parsers.
struct DslParser {
    tokens: Vec<DslTok>,
    pos: usize,
    line_no: u32,
}

impl DslParser {
    fn new(tokens: Vec<DslTok>, line_no: u32) -> Self {
        Self { tokens, pos: 0, line_no }
    }

    fn peek(&self) -> &DslTok {
        self.tokens.get(self.pos).unwrap_or(&DslTok::Eof)
    }

    fn bump(&mut self) -> DslTok {
        let tok = self.peek().clone();
        if !matches!(tok, DslTok::Eof) {
            self.pos += 1;
        }
        tok
    }

    fn err(&self, message: impl Into<String>) -> TextError {
        TextError::new(message.into(), TextSpan::at(self.line_no, self.pos as u32 + 1))
    }

    fn expect_tok(&mut self, expected: DslTok, label: &str) -> Result<(), TextError> {
        let got = self.bump();
        if std::mem::discriminant(&got) == std::mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(self.err(format!("expected {label}, got {got:?}")))
        }
    }

    fn expect_ident(&mut self) -> Result<String, TextError> {
        match self.bump() {
            DslTok::Ident(s) => Ok(s),
            other => Err(self.err(format!("expected identifier, got {other:?}"))),
        }
    }

    fn expect_str(&mut self) -> Result<String, TextError> {
        match self.bump() {
            DslTok::Str(s) => Ok(s),
            other => Err(self.err(format!("expected string, got {other:?}"))),
        }
    }

    fn expect_number(&mut self) -> Result<f64, TextError> {
        match self.bump() {
            DslTok::Number(n) => Ok(n),
            other => Err(self.err(format!("expected number, got {other:?}"))),
        }
    }

    fn expect_eof(&mut self) -> Result<(), TextError> {
        if matches!(self.peek(), DslTok::Eof) {
            Ok(())
        } else {
            Err(self.err(format!("unexpected trailing token {:?}", self.peek())))
        }
    }
}
//#endregion 🔖DslLexer

//#region 🔖DslValue
/// 📝 Prints a property value using `mathematical_graph_dsl::wire`'s literal style (`'str'`, bare
/// number/bool/null, `{k: v}`, `[v, v]`). Exposed `pub` so `trinity_rewrite`'s own `RewriteRuleState`
/// DSL can reuse it for `parameter_bindings` values instead of hand-rolling a second copy.
pub fn print_property_value(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Null => "null".into(),
        PropertyValue::Bool(b) => b.to_string(),
        PropertyValue::Number(n) => n.to_string(),
        PropertyValue::String(s) => format!("'{}'", escape_quoted(s, '\'')),
        PropertyValue::Object(map) => {
            let inner = map.iter().map(|(k, v)| format!("{k}: {}", print_property_value(v))).collect::<Vec<_>>().join(", ");
            format!("{{{inner}}}")
        }
        PropertyValue::Array(items) => {
            let inner = items.iter().map(print_property_value).collect::<Vec<_>>().join(", ");
            format!("[{inner}]")
        }
    }
}

/// 🔍 Parses one property-value expression, recursing into nested `{...}`/`[...]` — unlike
/// `mathematical_graph_dsl::wire::dag_from_wire_literal`'s `parse_value`, which only reads scalars,
/// this is required here: trinity fixtures carry nested object properties (`position: {x, y, z}`).
fn parse_property_value(p: &mut DslParser) -> Result<PropertyValue, TextError> {
    match p.bump() {
        DslTok::Str(s) => Ok(PropertyValue::String(s)),
        DslTok::Number(n) => Ok(PropertyValue::Number(n)),
        DslTok::Ident(s) if s == "true" => Ok(PropertyValue::Bool(true)),
        DslTok::Ident(s) if s == "false" => Ok(PropertyValue::Bool(false)),
        DslTok::Ident(s) if s == "null" => Ok(PropertyValue::Null),
        DslTok::LBrace => {
            let mut map = BTreeMap::new();
            while !matches!(p.peek(), DslTok::RBrace) {
                let key = p.expect_ident()?;
                p.expect_tok(DslTok::Colon, "':'")?;
                let value = parse_property_value(p)?;
                map.insert(key, value);
                if matches!(p.peek(), DslTok::Comma) {
                    p.bump();
                }
            }
            p.bump();
            Ok(PropertyValue::Object(map))
        }
        DslTok::LBracket => {
            let mut items = Vec::new();
            while !matches!(p.peek(), DslTok::RBracket) {
                items.push(parse_property_value(p)?);
                if matches!(p.peek(), DslTok::Comma) {
                    p.bump();
                }
            }
            p.bump();
            Ok(PropertyValue::Array(items))
        }
        other => Err(p.err(format!("expected a property value, got {other:?}"))),
    }
}

/// 🔍 Parses a single standalone property-value expression from a whole string. `pub` so
/// `trinity_rewrite` can decode its `RewriteRuleState.parameter_bindings` values without depending on
/// this module's private lexer/parser types.
pub fn parse_property_value_line(text: &str) -> Result<PropertyValue, TextError> {
    let tokens = dsl_lex(text, 1)?;
    let mut p = DslParser::new(tokens, 1);
    let value = parse_property_value(&mut p)?;
    p.expect_eof()?;
    Ok(value)
}

fn print_property_bag(bag: &PropertyBag) -> String {
    if bag.is_empty() {
        return String::new();
    }
    let inner = bag.iter().map(|(k, v)| format!("{k}: {}", print_property_value(v))).collect::<Vec<_>>().join(", ");
    format!("{{{inner}}}")
}

fn parse_property_bag(p: &mut DslParser) -> Result<PropertyBag, TextError> {
    let mut bag = PropertyBag::new();
    if !matches!(p.peek(), DslTok::LBrace) {
        return Ok(bag);
    }
    p.bump();
    while !matches!(p.peek(), DslTok::RBrace) {
        let key = p.expect_ident()?;
        p.expect_tok(DslTok::Colon, "':'")?;
        let value = parse_property_value(p)?;
        bag.insert(key, value);
        if matches!(p.peek(), DslTok::Comma) {
            p.bump();
        }
    }
    p.bump();
    Ok(bag)
}

fn escape_quoted(value: &str, quote: char) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(ch),
        }
    }
    out
}

fn quote_text(value: &str) -> String {
    format!("\"{}\"", escape_quoted(value, '"'))
}
//#endregion 🔖DslValue

//#region 🔖DslEntities
fn print_port_direction(direction: PortDirection) -> &'static str {
    match direction {
        PortDirection::In => "in",
        PortDirection::Out => "out",
    }
}

fn parse_port_direction(p: &mut DslParser) -> Result<PortDirection, TextError> {
    match p.expect_ident()?.as_str() {
        "in" => Ok(PortDirection::In),
        "out" => Ok(PortDirection::Out),
        other => Err(p.err(format!("expected port direction 'in'/'out', got '{other}'"))),
    }
}

fn print_port(port: &Port) -> String {
    let mut out = format!("{}:{}:{}", port.id, port.kind, print_port_direction(port.direction));
    let props = print_property_bag(&port.properties);
    if !props.is_empty() {
        out.push_str(&props);
    }
    out
}

fn parse_port(p: &mut DslParser) -> Result<Port, TextError> {
    let id = p.expect_ident()?;
    p.expect_tok(DslTok::Colon, "':'")?;
    let kind = p.expect_ident()?;
    p.expect_tok(DslTok::Colon, "':'")?;
    let direction = parse_port_direction(p)?;
    let properties = parse_property_bag(p)?;
    Ok(Port { id, kind, direction, properties })
}

fn print_ports_list(ports: &[Port]) -> String {
    format!("[{}]", ports.iter().map(print_port).collect::<Vec<_>>().join(", "))
}

fn parse_ports_list(p: &mut DslParser) -> Result<Vec<Port>, TextError> {
    let mut ports = Vec::new();
    if !matches!(p.peek(), DslTok::LBracket) {
        return Ok(ports);
    }
    p.bump();
    while !matches!(p.peek(), DslTok::RBracket) {
        ports.push(parse_port(p)?);
        if matches!(p.peek(), DslTok::Comma) {
            p.bump();
        }
    }
    p.bump();
    Ok(ports)
}

/// 📝 Prints one `node` line: `node id:Kind "name" x y w h {props} [ports]` — geometry/name/multi-port
/// fields a bare `mathematical_graph_dsl::wire::WireNode` (id/kind/one optional port) cannot carry, so
/// this is a from-scratch grammar in the same lexical style rather than a call into `wire`.
fn print_node_line(node: &Node) -> String {
    let mut out = format!("node {}:{} {} {} {} {} {}", node.id, node.kind, quote_text(&node.name), node.x, node.y, node.width, node.height);
    let props = print_property_bag(&node.properties);
    if !props.is_empty() {
        out.push(' ');
        out.push_str(&props);
    }
    if !node.ports.is_empty() {
        out.push(' ');
        out.push_str(&print_ports_list(&node.ports));
    }
    out
}

fn parse_node_fields(p: &mut DslParser) -> Result<Node, TextError> {
    let id = p.expect_ident()?;
    p.expect_tok(DslTok::Colon, "':'")?;
    let kind = p.expect_ident()?;
    let name = p.expect_str()?;
    let x = p.expect_number()?;
    let y = p.expect_number()?;
    let width = p.expect_number()?;
    let height = p.expect_number()?;
    let properties = parse_property_bag(p)?;
    let ports = parse_ports_list(p)?;
    Ok(Node { id, kind, name, x, y, width, height, properties, ports })
}

/// 📝 Prints one `edge` line reusing `mathematical_graph_dsl::wire`'s connector notation verbatim:
/// `edge id:Kind from:FromKind@fromPort->to:ToKind@toPort {props}` (node kinds are looked up for
/// readability only, exactly as `wire_literal_from_dag` does — they are dropped again on parse).
fn print_edge_line(edge: &Edge, nodes: &[Node]) -> String {
    let src_node = port_node_id(&edge.source).unwrap_or("node");
    let src_port = port_port_id(&edge.source).unwrap_or("");
    let tgt_node = port_node_id(&edge.target).unwrap_or("node");
    let tgt_port = port_port_id(&edge.target).unwrap_or("");
    let src_kind = nodes.iter().find(|n| n.id == src_node).map(|n| n.kind.as_str()).unwrap_or("node");
    let tgt_kind = nodes.iter().find(|n| n.id == tgt_node).map(|n| n.kind.as_str()).unwrap_or("node");
    let mut out = format!("edge {}:{} {}:{}@{}->{}:{}@{}", edge.id, edge.kind, src_node, src_kind, src_port, tgt_node, tgt_kind, tgt_port);
    let props = print_property_bag(&edge.properties);
    if !props.is_empty() {
        out.push(' ');
        out.push_str(&props);
    }
    out
}

fn parse_port_ref(p: &mut DslParser) -> Result<(String, String), TextError> {
    let id = p.expect_ident()?;
    p.expect_tok(DslTok::Colon, "':'")?;
    let _kind = p.expect_ident()?;
    p.expect_tok(DslTok::At, "'@'")?;
    let port = p.expect_ident()?;
    Ok((id, port))
}

fn parse_edge_fields(p: &mut DslParser) -> Result<Edge, TextError> {
    let id = p.expect_ident()?;
    p.expect_tok(DslTok::Colon, "':'")?;
    let kind = p.expect_ident()?;
    let (src_node, src_port) = parse_port_ref(p)?;
    p.expect_tok(DslTok::Arrow, "'->'")?;
    let (tgt_node, tgt_port) = parse_port_ref(p)?;
    let properties = parse_property_bag(p)?;
    Ok(Edge { id, kind, source: port_key(&src_node, &src_port), target: port_key(&tgt_node, &tgt_port), properties })
}

fn parse_plain_port_ref(p: &mut DslParser) -> Result<String, TextError> {
    let id = p.expect_ident()?;
    p.expect_tok(DslTok::Colon, "':'")?;
    let port = p.expect_ident()?;
    Ok(port_key(&id, &port))
}

fn entity_kind_and_id(entity: &EntityRef) -> (&'static str, &str) {
    match entity {
        EntityRef::Node(id) => ("node", id.as_str()),
        EntityRef::Edge(id) => ("edge", id.as_str()),
    }
}

fn parse_entity_and_key(p: &mut DslParser) -> Result<(EntityRef, String), TextError> {
    let kind = p.expect_ident()?;
    p.expect_tok(DslTok::Colon, "':'")?;
    let id = p.expect_ident()?;
    let key = p.expect_ident()?;
    let entity = match kind.as_str() {
        "node" => EntityRef::Node(id),
        "edge" => EntityRef::Edge(id),
        other => return Err(p.err(format!("unknown entity kind '{other}'"))),
    };
    Ok((entity, key))
}
//#endregion 🔖DslEntities

//#region 🔖DslDocument
/// 📜 Handcrafted `.trinity` textual notation for a whole [`GraphFixture`] (`vcs::DocumentDsl`):
/// `manifest`/`name`/`camera`/`root` header lines, then one `node`/`edge` line per entity. Adapts
/// `mathematical_graph_dsl::wire`'s `id:Kind@port` connector style for edges; nodes need their own
/// grammar (see {@link print_node_line}) since `WireNode` cannot express geometry/name/multiple ports.
impl DocumentDsl for GraphFixture {
    const EXTENSION: &'static str = "trinity";

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let mut manifest_id: Option<String> = None;
        let mut name = String::new();
        let mut camera = Camera::default();
        let mut root_node_id: Option<String> = None;
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for (index, raw_line) in text.lines().enumerate() {
            let line_no = index as u32 + 1;
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }
            let tokens = dsl_lex(line, line_no)?;
            let mut p = DslParser::new(tokens, line_no);
            let keyword = p.expect_ident()?;
            match keyword.as_str() {
                "manifest" => {
                    let value = p.expect_ident()?;
                    manifest_id = if value == "-" { None } else { Some(value) };
                }
                "name" => name = p.expect_str()?,
                "camera" => camera = Camera { x: p.expect_number()?, y: p.expect_number()?, zoom: p.expect_number()? },
                "root" => root_node_id = Some(p.expect_ident()?),
                "node" => nodes.push(parse_node_fields(&mut p)?),
                "edge" => edges.push(parse_edge_fields(&mut p)?),
                other => return Err(TextError::new(format!("unknown dsl line keyword '{other}'"), TextSpan::at(line_no, 1))),
            }
            p.expect_eof()?;
        }

        let mut fixture = GraphFixture { schema: GraphFixture::SCHEMA.to_string(), name, manifest_id, manifest: Manifest::default(), camera, nodes, edges, root_node_id };
        fixture.resolve_manifest().map_err(|error| TextError::new(error.to_string(), TextSpan::at(1, 1)))?;
        Ok(fixture)
    }

    fn print_dsl(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("manifest {}", self.manifest_id.as_deref().unwrap_or("-")));
        lines.push(format!("name {}", quote_text(&self.name)));
        lines.push(format!("camera {} {} {}", self.camera.x, self.camera.y, self.camera.zoom));
        if let Some(root) = &self.root_node_id {
            lines.push(format!("root {root}"));
        }
        for node in &self.nodes {
            lines.push(print_node_line(node));
        }
        for edge in &self.edges {
            lines.push(print_edge_line(edge, &self.nodes));
        }
        lines.join("\n")
    }
}
//#endregion 🔖DslDocument
//#endregion 🔖Dsl

//#region 🔖OpText
/// ⚡ Handcrafted one-line textual notation for [`TrinityGraphOperation`] (`vcs::OpText`) — one keyword
/// per variant followed by its fields in the same lexical style as `🔖Dsl`; `SetFixture` embeds a whole
/// `print_dsl()` document inline via an escaped quoted field (escaping turns its newlines into `\n`, so
/// the printed op line itself never contains one, satisfying `OpText::print_op`'s one-line law).
impl OpText for TrinityGraphOperation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let tokens = dsl_lex(line, 1)?;
        let mut p = DslParser::new(tokens, 1);
        let keyword = p.expect_ident()?;
        let operation = match keyword.as_str() {
            "createNode" => {
                let id = p.expect_ident()?;
                p.expect_tok(DslTok::Colon, "':'")?;
                let kind = p.expect_ident()?;
                let x = p.expect_number()?;
                let y = p.expect_number()?;
                let width = p.expect_number()?;
                let height = p.expect_number()?;
                let ports = parse_ports_list(&mut p)?;
                let name = p.expect_str()?;
                TrinityGraphOperation::CreateNode { id, kind, name, x, y, width, height, ports }
            }
            "deleteNode" => TrinityGraphOperation::DeleteNode { id: p.expect_ident()? },
            "createEdge" => {
                let id = p.expect_ident()?;
                p.expect_tok(DslTok::Colon, "':'")?;
                let kind = p.expect_ident()?;
                let source = parse_plain_port_ref(&mut p)?;
                p.expect_tok(DslTok::Arrow, "'->'")?;
                let target = parse_plain_port_ref(&mut p)?;
                let properties = parse_property_bag(&mut p)?;
                TrinityGraphOperation::CreateEdge { id, kind, source, target, properties }
            }
            "deleteEdge" => TrinityGraphOperation::DeleteEdge { id: p.expect_ident()? },
            "rename" => {
                let id = p.expect_ident()?;
                let name = p.expect_str()?;
                TrinityGraphOperation::Rename { id, name }
            }
            "reposition" => {
                let id = p.expect_ident()?;
                let x = p.expect_number()?;
                let y = p.expect_number()?;
                TrinityGraphOperation::Reposition { id, x, y }
            }
            "setDataProperty" => {
                let (entity, key) = parse_entity_and_key(&mut p)?;
                let value = parse_property_value(&mut p)?;
                TrinityGraphOperation::SetDataProperty { entity, key, value }
            }
            "clearDataProperty" => {
                let (entity, key) = parse_entity_and_key(&mut p)?;
                TrinityGraphOperation::ClearDataProperty { entity, key }
            }
            "setCamera" => {
                let camera = Camera { x: p.expect_number()?, y: p.expect_number()?, zoom: p.expect_number()? };
                TrinityGraphOperation::SetCamera { camera }
            }
            "setFixture" => {
                let text = p.expect_str()?;
                let fixture = GraphFixture::parse_dsl(&text)?;
                TrinityGraphOperation::SetFixture { fixture }
            }
            other => return Err(TextError::new(format!("unknown op keyword '{other}'"), TextSpan::at(1, 1))),
        };
        p.expect_eof()?;
        Ok(operation)
    }

    fn print_op(&self) -> String {
        match self {
            TrinityGraphOperation::CreateNode { id, kind, name, x, y, width, height, ports } => {
                let mut out = format!("createNode {id}:{kind} {x} {y} {width} {height}");
                if !ports.is_empty() {
                    out.push(' ');
                    out.push_str(&print_ports_list(ports));
                }
                out.push(' ');
                out.push_str(&quote_text(name));
                out
            }
            TrinityGraphOperation::DeleteNode { id } => format!("deleteNode {id}"),
            TrinityGraphOperation::CreateEdge { id, kind, source, target, properties } => {
                let mut out = format!("createEdge {id}:{kind} {source}->{target}");
                let props = print_property_bag(properties);
                if !props.is_empty() {
                    out.push(' ');
                    out.push_str(&props);
                }
                out
            }
            TrinityGraphOperation::DeleteEdge { id } => format!("deleteEdge {id}"),
            TrinityGraphOperation::Rename { id, name } => format!("rename {id} {}", quote_text(name)),
            TrinityGraphOperation::Reposition { id, x, y } => format!("reposition {id} {x} {y}"),
            TrinityGraphOperation::SetDataProperty { entity, key, value } => {
                let (kind, id) = entity_kind_and_id(entity);
                format!("setDataProperty {kind}:{id} {key} {}", print_property_value(value))
            }
            TrinityGraphOperation::ClearDataProperty { entity, key } => {
                let (kind, id) = entity_kind_and_id(entity);
                format!("clearDataProperty {kind}:{id} {key}")
            }
            TrinityGraphOperation::SetCamera { camera } => format!("setCamera {} {} {}", camera.x, camera.y, camera.zoom),
            TrinityGraphOperation::SetFixture { fixture } => format!("setFixture {}", quote_text(&fixture.print_dsl())),
        }
    }
}
//#endregion 🔖OpText

pub fn empty_trinity_graph_fixture() -> GraphFixture {
    GraphFixture { schema: GraphFixture::SCHEMA.into(), name: "trinity".into(), manifest_id: Some("nakagin".into()), manifest: Manifest::nakagin_default(), camera: Camera::default(), nodes: Vec::new(), edges: Vec::new(), root_node_id: None }
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
                    let envelope: TrinityGraphEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    TrinityGraphStore::new(envelope)
                }
                None => TrinityGraphStore::new(create_trinity_graph_envelope("trinity", empty_trinity_graph_fixture())),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
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
//#endregion 🔖WasmBridge

// #region 🔖Tests
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
        dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::CreateNode { id: "new".into(), kind: "Piece".into(), name: "new-piece".into(), x: 200.0, y: 40.0, width: 80.0, height: 40.0, ports: vec![] }]).expect("create");
        assert_eq!(store.projection().expect("projection").nodes.len(), 3);
        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
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
}
// #endregion 🔖Tests
