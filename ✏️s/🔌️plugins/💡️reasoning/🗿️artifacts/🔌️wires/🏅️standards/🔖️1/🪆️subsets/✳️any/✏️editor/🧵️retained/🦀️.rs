//! 🧵️ Bounded Wires document publication for one released canvas drag.

use crate::{WiresSnapshot, WiresWorkingScene};
use dsl::DslValue;
use replication::value::bounded_clone::{DslValueCloneCloseStep, DslValueCloneCursor, DslValueCloneGrant, DslValueCloneLimits, DslValueCloneStep};
use replication::value::DslValueSource;
use std::hash::Hasher;
use std::sync::Arc;
use store::retirement::{OwnedValueRetirementFactory, RetireOwned, SharedValueRetirementFactory};
use store::{
    ArtifactCanonicalJson, ArtifactCanonicalJsonCursor, ArtifactCanonicalJsonNode, ArtifactCanonicalValue, ArtifactCanonicalValueAdmission, ArtifactCanonicalValueCloseStep, ArtifactCanonicalValueGrant, ArtifactCanonicalValueLimits,
    ArtifactCanonicalValueStep, ArtifactOwnedValueRetirementFactory, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation,
    ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPreparationStep, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemSealer, ErasedSnapshotRetirement, HistoryLane, SnapshotRetirementStep,
};

const MAXIMUM_RETAINED_BYTES: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
const MAXIMUM_WORK_ITEMS: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS;
const NONE: usize = usize::MAX;
const GRAPH_SCHEMA: &str = "s.stdio.semio.graph";
const NODE_PROPERTY: &str = "wires.node";

fn compose_checkpoint(base: ArtifactStoreOneItemCheckpoint, seal: ArtifactStoreOneItemCheckpoint) -> Result<ArtifactStoreOneItemCheckpoint, String> {
    let cursor = base.cursor.checked_add(seal.cursor).ok_or_else(|| "wires-publication.work-overflow".to_string())?;
    if cursor as usize > MAXIMUM_WORK_ITEMS {
        return Err("wires-publication.work-limit".into());
    }
    Ok(ArtifactStoreOneItemCheckpoint {
        cursor,
        completed_items: base.completed_items.checked_add(seal.completed_items).ok_or_else(|| "wires-publication.work-overflow".to_string())?,
        completed_bytes: base.completed_bytes.checked_add(seal.completed_bytes).ok_or_else(|| "wires-publication.byte-overflow".to_string())?,
        digest: seal.digest,
    })
}

#[derive(Clone, Copy)]
struct NodeIndex {
    id: usize,
    kind: usize,
    label: usize,
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct EdgeIndex {
    id: usize,
    source: usize,
    target: usize,
    kind: usize,
}

store::artifact_retire_struct!(NodeIndex { id, kind, label, x, y });
store::artifact_retire_struct!(EdgeIndex { id, source, target, kind });

struct Candidate {
    wires_fixture: Option<DslValue>,
    meta: Option<DslValue>,
    nodes: Vec<DslValue>,
    edges: Vec<DslValue>,
    node_json: Vec<String>,
    edge_json: Vec<String>,
    node_index: Vec<NodeIndex>,
    edge_index: Vec<EdgeIndex>,
}

impl Candidate {
    fn new() -> Self {
        Self { wires_fixture: None, meta: None, nodes: Vec::new(), edges: Vec::new(), node_json: Vec::new(), edge_json: Vec::new(), node_index: Vec::new(), edge_index: Vec::new() }
    }
}

store::artifact_retire_struct!(Candidate { wires_fixture, meta, nodes, edges, node_json, edge_json, node_index, edge_index });

#[derive(Clone, Copy, PartialEq, Eq)]
enum SnapshotField {
    WiresFixture,
    Meta,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SceneField {
    Node(usize),
    Edge(usize),
}

enum WiresValueSource {
    Snapshot { base: store::SnapshotRead<WiresSnapshot>, field: SnapshotField },
    Scene { scene: Arc<WiresWorkingScene>, field: SceneField },
}

impl DslValueSource for WiresValueSource {
    fn value(&self) -> &DslValue {
        match self {
            Self::Snapshot { base, field: SnapshotField::WiresFixture } => &base.get().wires_fixture,
            Self::Snapshot { base, field: SnapshotField::Meta } => &base.get().meta,
            Self::Scene { scene, field: SceneField::Node(index) } => &scene.nodes[*index],
            Self::Scene { scene, field: SceneField::Edge(index) } => &scene.edges[*index],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CloneTarget {
    WiresFixture,
    Meta,
    Node(usize),
    Edge(usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    ReserveNodes,
    ReserveEdges,
    ReserveNodeJson,
    ReserveEdgeJson,
    ReserveNodeIndex,
    ReserveEdgeIndex,
    ScanNodes,
    ScanEdges,
    BeginClone,
    Clone,
    CloneClose,
    BeginAdmission,
    Admission,
    AdmissionClose,
    Commit,
    CountJson,
    ReserveJson,
    FillJson,
    Hash,
    BuildSeal,
    Seal,
    Ready,
    Closing,
    Closed,
}

#[derive(Default)]
struct Utf8Carry {
    bytes: [u8; 4],
    length: usize,
    expected: usize,
}

impl Utf8Carry {
    fn push(&mut self, target: &mut String, source: &[u8]) -> Result<(), String> {
        for byte in source {
            if self.length == 0 && *byte < 0x80 {
                target.push(char::from(*byte));
                continue;
            }
            if self.length == 0 {
                self.expected = match *byte {
                    0xc2..=0xdf => 2,
                    0xe0..=0xef => 3,
                    0xf0..=0xf4 => 4,
                    _ => return Err("wires-publication.invalid-utf8".into()),
                };
            }
            self.bytes[self.length] = *byte;
            self.length += 1;
            if self.length == self.expected {
                target.push_str(std::str::from_utf8(&self.bytes[..self.length]).map_err(|_| "wires-publication.invalid-utf8")?);
                self.length = 0;
                self.expected = 0;
            }
        }
        Ok(())
    }
}

struct Projection<'a> {
    candidate: &'a Candidate,
}

fn object_field(value: &DslValue, index: usize) -> Result<&DslValue, String> {
    let DslValue::Object(fields) = value else {
        return Err("wires-publication.projection-shape".into());
    };
    fields.get(index).map(|(_, value)| value).ok_or_else(|| "wires-publication.projection-path".into())
}

fn text_field(value: &DslValue, index: usize) -> Result<&str, String> {
    if index == NONE {
        return Ok("");
    }
    object_field(value, index)?.as_str().ok_or_else(|| "wires-publication.projection-text".into())
}

fn number_field(value: &DslValue, index: usize) -> Result<f64, String> {
    if index == NONE {
        return Ok(0.0);
    }
    object_field(value, index)?.as_f64().ok_or_else(|| "wires-publication.projection-number".into())
}

impl ArtifactCanonicalJson for Projection<'_> {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        use ArtifactCanonicalJsonNode as N;
        Ok(match path {
            [] => N::Object(3),
            [0] => N::String(GRAPH_SCHEMA),
            [1] => N::Array(self.candidate.nodes.len()),
            [2] => N::Array(self.candidate.edges.len()),
            [1, node] => {
                self.candidate.nodes.get(*node).ok_or_else(|| "wires-publication.projection-path".to_string())?;
                N::Object(6)
            }
            [1, node, 0] => {
                self.candidate.node_index.get(*node).ok_or_else(|| "wires-publication.projection-path".to_string())?;
                N::Object(1)
            }
            [1, node, 0, 0] => N::String(text_field(&self.candidate.nodes[*node], self.candidate.node_index[*node].id)?),
            [1, node, 1] => N::String(text_field(&self.candidate.nodes[*node], self.candidate.node_index[*node].kind)?),
            [1, node, 2] => N::String(text_field(&self.candidate.nodes[*node], self.candidate.node_index[*node].label)?),
            [1, node, 3] => {
                self.candidate.node_index.get(*node).ok_or_else(|| "wires-publication.projection-path".to_string())?;
                N::Object(2)
            }
            [1, node, 3, 0] => N::F64(number_field(&self.candidate.nodes[*node], self.candidate.node_index[*node].x)?),
            [1, node, 3, 1] => N::F64(number_field(&self.candidate.nodes[*node], self.candidate.node_index[*node].y)?),
            [1, _, 4] => N::Array(0),
            [1, node, 5] => {
                self.candidate.node_json.get(*node).ok_or_else(|| "wires-publication.projection-path".to_string())?;
                N::Array(1)
            }
            [1, _, 5, 0] => N::Object(2),
            [1, _, 5, 0, 0] => N::String(NODE_PROPERTY),
            [1, _, 5, 0, 1] => N::Object(2),
            [1, _, 5, 0, 1, 0] => N::String("str"),
            [1, node, 5, 0, 1, 1] => N::String(self.candidate.node_json.get(*node).ok_or_else(|| "wires-publication.projection-path".to_string())?),
            [2, edge] => {
                self.candidate.edges.get(*edge).ok_or_else(|| "wires-publication.projection-path".to_string())?;
                N::Object(5)
            }
            [2, edge, 0..=2] => {
                self.candidate.edge_index.get(*edge).ok_or_else(|| "wires-publication.projection-path".to_string())?;
                N::Object(1)
            }
            [2, edge, slot @ (0..=2), 0] => {
                let index = self.candidate.edge_index[*edge];
                let field = match slot {
                    0 => index.id,
                    1 => index.source,
                    _ => index.target,
                };
                N::String(text_field(&self.candidate.edges[*edge], field)?)
            }
            [2, edge, 3] => N::String(text_field(&self.candidate.edges[*edge], self.candidate.edge_index[*edge].kind)?),
            [2, edge, 4] => N::String(self.candidate.edge_json.get(*edge).ok_or_else(|| "wires-publication.projection-path".to_string())?),
            _ => return Err("wires-publication.projection-path".into()),
        })
    }

    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<&str, String> {
        let keys: &[&str] = match path {
            [] => &["schema", "nodes", "edges"],
            [1, _] => &["id", "kind", "label", "position", "ports", "properties"],
            [1, _, 0] | [2, _, 0..=2] => &["value"],
            [1, _, 3] => &["x", "y"],
            [1, _, 5, 0] => &["key", "value"],
            [1, _, 5, 0, 1] => &["kind", "value"],
            [2, _] => &["id", "source", "target", "kind", "label"],
            _ => return Err("wires-publication.projection-path".into()),
        };
        keys.get(index).copied().ok_or_else(|| "wires-publication.projection-path".into())
    }
}

impl ArtifactCanonicalJson for crate::op::WiresMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        use ArtifactCanonicalJsonNode as N;
        let Self::MoveNode(value) = self else {
            return Err("wires-publication.unsupported-mutation".into());
        };
        Ok(match path {
            [] => N::Object(4),
            [0] => N::String("moveNode"),
            [1] => N::String(&value.node_id),
            [2] => N::F64(value.new_x),
            [3] => N::F64(value.new_y),
            _ => return Err("wires-publication.mutation-path".into()),
        })
    }

    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<&str, String> {
        if path.is_empty() {
            return ["mutation", "nodeId", "newX", "newY"].get(index).copied().ok_or_else(|| "wires-publication.mutation-path".into());
        }
        Err("wires-publication.mutation-path".into())
    }
}

pub struct WiresStorePreparationFactory;

pub fn factory() -> Arc<dyn ArtifactStoreOneItemPreparationFactory<WiresSnapshot, crate::op::WiresMutation>> {
    Arc::new(WiresStorePreparationFactory)
}

impl ArtifactStoreOneItemPreparationFactory<WiresSnapshot, crate::op::WiresMutation> for WiresStorePreparationFactory {
    fn preflight(&self, mutation: &crate::op::WiresMutation, description: Option<&str>, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        let crate::op::WiresMutation::MoveNode(value) = mutation else {
            return Err("Wires retained publication only admits MoveNode".into());
        };
        if lane != HistoryLane::Document
            || value.node_id.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || !value.new_x.is_finite()
            || !value.new_y.is_finite()
            || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES)
        {
            return Err("Wires MoveNode exceeds its retained publication envelope".into());
        }
        Ok(ArtifactStoreOneItemFootprint { work_items: MAXIMUM_WORK_ITEMS, retained_bytes: MAXIMUM_RETAINED_BYTES })
    }

    fn begin(
        &self,
        request: ArtifactStoreOneItemPreparationRequest<WiresSnapshot, crate::op::WiresMutation>,
    ) -> Result<Box<dyn ArtifactStoreOneItemPreparation<WiresSnapshot, crate::op::WiresMutation>>, ArtifactStoreOneItemPreparationRequest<WiresSnapshot, crate::op::WiresMutation>> {
        if request.lane != HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || !matches!(request.mutation, crate::op::WiresMutation::MoveNode(_))
        {
            return Err(request);
        }
        let Some(scene) = request.base.get().content.local_owner::<WiresWorkingScene>() else {
            return Err(request);
        };
        Ok(Box::new(WiresStorePreparation {
            base: Some(request.base),
            scene: Some(scene),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            candidate: Some(Candidate::new()),
            phase: Phase::ReserveNodes,
            node_cursor: 0,
            edge_cursor: 0,
            field_cursor: 0,
            pending_node: NodeIndex { id: NONE, kind: NONE, label: NONE, x: NONE, y: NONE },
            pending_edge: EdgeIndex { id: NONE, source: NONE, target: NONE, kind: NONE },
            target_node: None,
            old_x: None,
            old_y: None,
            clone_target: None,
            clone_cursor: None,
            pending_value: None,
            admission: None,
            canonical: None,
            json_cursor: ArtifactCanonicalJsonCursor::default(),
            json_length: 0,
            json: None,
            utf8: Utf8Carry::default(),
            hasher: std::collections::hash_map::DefaultHasher::new(),
            sealer: None,
            seal_base: None,
            checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes: 0,
            cancelled: false,
            closing: false,
            active_retirement: None,
        }))
    }
}

struct WiresStorePreparation {
    base: Option<store::SnapshotRead<WiresSnapshot>>,
    scene: Option<Arc<WiresWorkingScene>>,
    mutation: Option<crate::op::WiresMutation>,
    description: Option<String>,
    authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<Candidate>,
    phase: Phase,
    node_cursor: usize,
    edge_cursor: usize,
    field_cursor: usize,
    pending_node: NodeIndex,
    pending_edge: EdgeIndex,
    target_node: Option<usize>,
    old_x: Option<f64>,
    old_y: Option<f64>,
    clone_target: Option<CloneTarget>,
    clone_cursor: Option<DslValueCloneCursor<WiresValueSource>>,
    pending_value: Option<DslValue>,
    admission: Option<ArtifactCanonicalValueAdmission<Arc<DslValue>>>,
    canonical: Option<ArtifactCanonicalValue<Arc<DslValue>>>,
    json_cursor: ArtifactCanonicalJsonCursor,
    json_length: usize,
    json: Option<String>,
    utf8: Utf8Carry,
    hasher: std::collections::hash_map::DefaultHasher,
    sealer: Option<ArtifactStoreOneItemSealer<WiresSnapshot, crate::op::WiresMutation>>,
    seal_base: Option<ArtifactStoreOneItemCheckpoint>,
    checkpoint: ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
    active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
}

impl WiresStorePreparation {
    fn candidate(&self) -> Result<&Candidate, String> {
        self.candidate.as_ref().ok_or_else(|| "wires-publication.candidate-missing".into())
    }

    fn candidate_mut(&mut self) -> Result<&mut Candidate, String> {
        self.candidate.as_mut().ok_or_else(|| "wires-publication.candidate-missing".into())
    }

    fn record(&mut self, items: usize, bytes: usize) -> Result<(), String> {
        let cursor = usize::try_from(self.checkpoint.cursor).unwrap_or(usize::MAX).checked_add(items.max(usize::from(bytes > 0))).ok_or_else(|| "wires-publication.work-overflow".to_string())?;
        if cursor > MAXIMUM_WORK_ITEMS {
            return Err("wires-publication.work-limit".into());
        }
        self.checkpoint.cursor = cursor as u32;
        self.checkpoint.completed_items = self.checkpoint.completed_items.saturating_add(items as u32);
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.checked_add(bytes as u64).ok_or_else(|| "wires-publication.byte-overflow".to_string())?;
        Ok(())
    }

    fn reserve<T>(values: &mut Vec<T>, length: usize, remaining: usize) -> Result<usize, String> {
        let requested = length.checked_mul(size_of::<T>()).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
        if requested > remaining {
            return Err("wires-publication.retained-limit".into());
        }
        values.try_reserve_exact(length).map_err(|_| "wires-publication.allocation-failed".to_string())?;
        let actual = values.capacity().checked_mul(size_of::<T>()).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
        if actual > remaining {
            return Err("wires-publication.retained-limit".into());
        }
        Ok(actual)
    }

    fn reserve_phase(&mut self, phase: Phase) -> Result<(), String> {
        let scene = self.scene.as_ref().ok_or_else(|| "wires-publication.scene-missing".to_string())?;
        let nodes = scene.nodes.len();
        let edges = scene.edges.len();
        if nodes.saturating_add(edges) > MAXIMUM_WORK_ITEMS / 8 {
            return Err("wires-publication.scene-item-limit".into());
        }
        let remaining = MAXIMUM_RETAINED_BYTES.checked_sub(self.retained_bytes).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
        let bytes = match phase {
            Phase::ReserveNodes => Self::reserve(&mut self.candidate_mut()?.nodes, nodes, remaining)?,
            Phase::ReserveEdges => Self::reserve(&mut self.candidate_mut()?.edges, edges, remaining)?,
            Phase::ReserveNodeJson => Self::reserve(&mut self.candidate_mut()?.node_json, nodes, remaining)?,
            Phase::ReserveEdgeJson => Self::reserve(&mut self.candidate_mut()?.edge_json, edges, remaining)?,
            Phase::ReserveNodeIndex => Self::reserve(&mut self.candidate_mut()?.node_index, nodes, remaining)?,
            Phase::ReserveEdgeIndex => Self::reserve(&mut self.candidate_mut()?.edge_index, edges, remaining)?,
            _ => return Err("wires-publication.reserve-phase".into()),
        };
        self.retained_bytes += bytes;
        Ok(())
    }

    fn next_clone_target(&self) -> Option<CloneTarget> {
        let candidate = self.candidate().ok()?;
        let scene = self.scene.as_ref()?;
        if candidate.wires_fixture.is_none() {
            Some(CloneTarget::WiresFixture)
        } else if candidate.meta.is_none() {
            Some(CloneTarget::Meta)
        } else if candidate.nodes.len() < scene.nodes.len() {
            Some(CloneTarget::Node(candidate.nodes.len()))
        } else if candidate.edges.len() < scene.edges.len() {
            Some(CloneTarget::Edge(candidate.edges.len()))
        } else {
            None
        }
    }

    fn begin_clone(&mut self) -> Result<(), String> {
        let Some(target) = self.next_clone_target() else {
            self.phase = Phase::Hash;
            self.json_cursor = ArtifactCanonicalJsonCursor::default();
            return Ok(());
        };
        let source = match target {
            CloneTarget::WiresFixture => WiresValueSource::Snapshot { base: self.base.take().ok_or_else(|| "wires-publication.base-missing".to_string())?, field: SnapshotField::WiresFixture },
            CloneTarget::Meta => WiresValueSource::Snapshot { base: self.base.take().ok_or_else(|| "wires-publication.base-missing".to_string())?, field: SnapshotField::Meta },
            CloneTarget::Node(index) => WiresValueSource::Scene { scene: Arc::clone(self.scene.as_ref().ok_or_else(|| "wires-publication.scene-missing".to_string())?), field: SceneField::Node(index) },
            CloneTarget::Edge(index) => WiresValueSource::Scene { scene: Arc::clone(self.scene.as_ref().ok_or_else(|| "wires-publication.scene-missing".to_string())?), field: SceneField::Edge(index) },
        };
        let remaining = MAXIMUM_RETAINED_BYTES.checked_sub(self.retained_bytes).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
        self.clone_cursor = Some(DslValueCloneCursor::new(source, DslValueCloneLimits { maximum_depth: 64, maximum_retained_bytes: remaining }).map_err(|(_, reason)| reason.to_string())?);
        self.clone_target = Some(target);
        self.phase = Phase::Clone;
        Ok(())
    }

    fn restore_source(&mut self, source: WiresValueSource) -> Result<(), String> {
        match source {
            WiresValueSource::Snapshot { base, .. } => {
                if self.base.replace(base).is_some() {
                    return Err("wires-publication.duplicate-base".into());
                }
            }
            WiresValueSource::Scene { scene, .. } => drop(scene),
        }
        Ok(())
    }

    fn patch_target(&self, target: CloneTarget, value: &mut DslValue) -> Result<(), String> {
        let CloneTarget::Node(index) = target else {
            return Ok(());
        };
        if self.target_node != Some(index) {
            return Ok(());
        }
        let crate::op::WiresMutation::MoveNode(mutation) = self.mutation.as_ref().ok_or_else(|| "wires-publication.mutation-missing".to_string())? else {
            return Err("wires-publication.mutation-kind".into());
        };
        let metadata = *self.candidate()?.node_index.get(index).ok_or_else(|| "wires-publication.node-index".to_string())?;
        let DslValue::Object(fields) = value else {
            return Err("wires-publication.node-shape".into());
        };
        if metadata.x == NONE || metadata.y == NONE {
            return Err("wires-publication.position-missing".into());
        }
        fields[metadata.x].1 = DslValue::float(mutation.new_x);
        fields[metadata.y].1 = DslValue::float(mutation.new_y);
        Ok(())
    }

    fn commit(&mut self) -> Result<(), String> {
        let canonical = self.canonical.take().ok_or_else(|| "wires-publication.canonical-missing".to_string())?;
        let value = Arc::try_unwrap(canonical.into_source()).map_err(|_| "wires-publication.value-alias".to_string())?;
        let target = self.clone_target.ok_or_else(|| "wires-publication.clone-target".to_string())?;
        match target {
            CloneTarget::WiresFixture => self.candidate_mut()?.wires_fixture = Some(value),
            CloneTarget::Meta => self.candidate_mut()?.meta = Some(value),
            CloneTarget::Node(_) => {
                let json = self.json.take().ok_or_else(|| "wires-publication.json-missing".to_string())?;
                self.candidate_mut()?.nodes.push(value);
                self.candidate_mut()?.node_json.push(json);
            }
            CloneTarget::Edge(_) => {
                let json = self.json.take().ok_or_else(|| "wires-publication.json-missing".to_string())?;
                self.candidate_mut()?.edges.push(value);
                self.candidate_mut()?.edge_json.push(json);
            }
        }
        self.clone_target = None;
        self.phase = Phase::BeginClone;
        Ok(())
    }

    fn build_edit(&mut self, content_hash: u64) -> Result<(), String> {
        let old_x = self.old_x.ok_or_else(|| "wires-publication.target-missing".to_string())?;
        let old_y = self.old_y.ok_or_else(|| "wires-publication.target-missing".to_string())?;
        let forward = self.mutation.take().ok_or_else(|| "wires-publication.mutation-missing".to_string())?;
        let crate::op::WiresMutation::MoveNode(value) = &forward else {
            return Err("wires-publication.mutation-kind".into());
        };
        let inverse = crate::mutations::move_node(value.node_id.clone(), old_x, old_y);
        let authority = self.authority.as_ref().ok_or_else(|| "wires-publication.authority-missing".to_string())?;
        let id = format!("wires-move-{}", authority.next_sequence_number());
        if id.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err("wires-publication.edit-id-limit".into());
        }
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![forward],
            inverse: vec![inverse],
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                dependencies: Vec::new(),
                base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())),
                timestamp: authority.next_clock(),
                undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: authority.group_id().map(str::to_string),
                origin: Default::default(),
            }],
            description: self.description.take(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
        let mut candidate = self.candidate.take().ok_or_else(|| "wires-publication.candidate-missing".to_string())?;
        let content = crate::wires_content_child_from_hash(content_hash).with_local_owner(Arc::new(WiresWorkingScene { nodes: std::mem::take(&mut candidate.nodes), edges: std::mem::take(&mut candidate.edges) }));
        let post = WiresSnapshot { wires_fixture: candidate.wires_fixture.take().ok_or_else(|| "wires-publication.fixture-missing".to_string())?, content, meta: candidate.meta.take().ok_or_else(|| "wires-publication.meta-missing".to_string())? };
        self.active_retirement = Some(OwnedValueRetirementFactory::<Candidate>::default().retire_owned(candidate));
        self.sealer = Some(authority.begin_one_item_seal(edit, Arc::new(post), Arc::new(OwnedValueRetirementFactory::<crate::op::WiresMutation>::default()), Arc::new(SharedValueRetirementFactory::<WiresSnapshot>::default())));
        self.phase = Phase::Seal;
        Ok(())
    }

    fn close_active(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<Option<SnapshotRetirementStep>, String> {
        let Some(active) = self.active_retirement.as_mut() else {
            return Ok(None);
        };
        let step = active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
        if matches!(step, SnapshotRetirementStep::Complete) {
            if !active.terminal_is_empty() {
                return Err("wires-publication.retirement-witness".into());
            }
            self.active_retirement = None;
            return Ok(Some(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        Ok(Some(step))
    }

    fn retire_owned<T: RetireOwned>(&mut self, value: T) {
        self.active_retirement = Some(OwnedValueRetirementFactory::<T>::default().retire_owned(value));
    }
}

impl ArtifactStoreOneItemPreparation<WiresSnapshot, crate::op::WiresMutation> for WiresStorePreparation {
    fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.phase == Phase::Ready {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.phase {
            Phase::ReserveNodes | Phase::ReserveEdges | Phase::ReserveNodeJson | Phase::ReserveEdgeJson | Phase::ReserveNodeIndex | Phase::ReserveEdgeIndex => {
                let current = self.phase;
                self.reserve_phase(current)?;
                self.phase = match current {
                    Phase::ReserveNodes => Phase::ReserveEdges,
                    Phase::ReserveEdges => Phase::ReserveNodeJson,
                    Phase::ReserveNodeJson => Phase::ReserveEdgeJson,
                    Phase::ReserveEdgeJson => Phase::ReserveNodeIndex,
                    Phase::ReserveNodeIndex => Phase::ReserveEdgeIndex,
                    _ => Phase::ScanNodes,
                };
                self.record(1, 0)?;
            }
            Phase::ScanNodes => {
                let scene = self.scene.as_ref().ok_or_else(|| "wires-publication.scene-missing".to_string())?;
                let Some(node) = scene.nodes.get(self.node_cursor) else {
                    self.phase = Phase::ScanEdges;
                    self.field_cursor = 0;
                    self.record(1, 0)?;
                    return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
                };
                let DslValue::Object(fields) = node else {
                    return Err("wires-publication.node-shape".into());
                };
                if let Some((key, value)) = fields.get(self.field_cursor) {
                    let field = self.field_cursor;
                    self.field_cursor += 1;
                    match key.as_str() {
                        "id" => self.pending_node.id = field,
                        "nodeKind" => self.pending_node.kind = field,
                        "text" => self.pending_node.label = field,
                        "x" => self.pending_node.x = field,
                        "y" => self.pending_node.y = field,
                        _ => {}
                    }
                    if key == "id" {
                        if let Some(id) = value.as_str() {
                            let expected = match self.mutation.as_ref().ok_or_else(|| "wires-publication.mutation-missing".to_string())? {
                                crate::op::WiresMutation::MoveNode(value) => value.node_id.as_str(),
                                _ => return Err("wires-publication.mutation-kind".into()),
                            };
                            if id == expected && self.target_node.is_none() {
                                self.target_node = Some(self.node_cursor);
                            }
                        }
                    }
                    self.record(1, 0)?;
                } else {
                    if self.target_node == Some(self.node_cursor) {
                        self.old_x = Some(number_field(node, self.pending_node.x)?);
                        self.old_y = Some(number_field(node, self.pending_node.y)?);
                        if !self.old_x.is_some_and(f64::is_finite) || !self.old_y.is_some_and(f64::is_finite) {
                            return Err("wires-publication.position-non-finite".into());
                        }
                    }
                    let pending = self.pending_node;
                    self.candidate_mut()?.node_index.push(pending);
                    self.pending_node = NodeIndex { id: NONE, kind: NONE, label: NONE, x: NONE, y: NONE };
                    self.node_cursor += 1;
                    self.field_cursor = 0;
                    self.record(1, 0)?;
                }
            }
            Phase::ScanEdges => {
                let scene = self.scene.as_ref().ok_or_else(|| "wires-publication.scene-missing".to_string())?;
                let Some(edge) = scene.edges.get(self.edge_cursor) else {
                    if self.target_node.is_none() {
                        return Err("wires-publication.target-missing".into());
                    }
                    self.phase = Phase::BeginClone;
                    self.record(1, 0)?;
                    return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
                };
                let DslValue::Object(fields) = edge else {
                    return Err("wires-publication.edge-shape".into());
                };
                if let Some((key, _)) = fields.get(self.field_cursor) {
                    let field = self.field_cursor;
                    self.field_cursor += 1;
                    match key.as_str() {
                        "id" => self.pending_edge.id = field,
                        "source" => self.pending_edge.source = field,
                        "target" => self.pending_edge.target = field,
                        "edgeKind" => self.pending_edge.kind = field,
                        _ => {}
                    }
                    self.record(1, 0)?;
                } else {
                    let pending = self.pending_edge;
                    self.candidate_mut()?.edge_index.push(pending);
                    self.pending_edge = EdgeIndex { id: NONE, source: NONE, target: NONE, kind: NONE };
                    self.edge_cursor += 1;
                    self.field_cursor = 0;
                    self.record(1, 0)?;
                }
            }
            Phase::BeginClone => {
                self.begin_clone()?;
                self.record(1, 0)?;
            }
            Phase::Clone => {
                let result = self
                    .clone_cursor
                    .as_mut()
                    .ok_or_else(|| "wires-publication.clone-missing".to_string())?
                    .advance(DslValueCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
                match result {
                    DslValueCloneStep::Blocked(_) => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
                    DslValueCloneStep::Progress { receipt, .. } => self.record(receipt.structural_items, receipt.copied_bytes)?,
                    DslValueCloneStep::Complete { receipt, checkpoint } => {
                        self.record(receipt.structural_items, receipt.copied_bytes)?;
                        self.retained_bytes = self.retained_bytes.checked_add(checkpoint.retained_bytes).filter(|bytes| *bytes <= MAXIMUM_RETAINED_BYTES).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
                        let mut value = self.clone_cursor.as_mut().and_then(DslValueCloneCursor::take_value).ok_or_else(|| "wires-publication.clone-value-missing".to_string())?;
                        self.patch_target(self.clone_target.ok_or_else(|| "wires-publication.clone-target".to_string())?, &mut value)?;
                        self.pending_value = Some(value);
                        self.phase = Phase::CloneClose;
                    }
                }
            }
            Phase::CloneClose => {
                let cursor = self.clone_cursor.as_mut().ok_or_else(|| "wires-publication.clone-missing".to_string())?;
                match cursor.close_step(DslValueCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }) {
                    DslValueCloneCloseStep::Blocked(_) => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
                    DslValueCloneCloseStep::Progress { receipt, .. } => self.record(receipt.structural_items, receipt.copied_bytes)?,
                    DslValueCloneCloseStep::Returned { source, receipt, .. } => {
                        self.record(receipt.structural_items, receipt.copied_bytes)?;
                        self.restore_source(source)?;
                        let cursor = self.clone_cursor.take().ok_or_else(|| "wires-publication.clone-missing".to_string())?;
                        if !cursor.terminal_is_empty() {
                            return Err("wires-publication.clone-witness".into());
                        }
                        self.phase = Phase::BeginAdmission;
                    }
                    DslValueCloneCloseStep::Complete(_) => {
                        let cursor = self.clone_cursor.take().ok_or_else(|| "wires-publication.clone-missing".to_string())?;
                        if !cursor.terminal_is_empty() {
                            return Err("wires-publication.clone-witness".into());
                        }
                        self.phase = Phase::BeginAdmission;
                    }
                }
            }
            Phase::BeginAdmission => {
                let value = Arc::new(self.pending_value.take().ok_or_else(|| "wires-publication.value-missing".to_string())?);
                let limits = ArtifactCanonicalValueLimits {
                    maximum_depth: 64,
                    maximum_retained_bytes: MAXIMUM_RETAINED_BYTES.checked_sub(self.retained_bytes).ok_or_else(|| "wires-publication.retained-limit".to_string())?,
                    maximum_work_items: MAXIMUM_WORK_ITEMS.saturating_sub(self.checkpoint.cursor as usize).max(1),
                };
                self.admission = Some(ArtifactCanonicalValueAdmission::new(value, limits).map_err(|(_, reason)| reason.to_string())?);
                self.phase = Phase::Admission;
                self.record(1, 0)?;
            }
            Phase::Admission => {
                let (processed_bytes, step) = {
                    let admission = self.admission.as_mut().ok_or_else(|| "wires-publication.admission-missing".to_string())?;
                    let processed_bytes = admission.checkpoint().processed_bytes;
                    let step = admission.advance(ArtifactCanonicalValueGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
                    (processed_bytes, step)
                };
                match step {
                    ArtifactCanonicalValueStep::Blocked => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
                    ArtifactCanonicalValueStep::Progress(checkpoint) => self.record(1, checkpoint.processed_bytes.saturating_sub(processed_bytes))?,
                    ArtifactCanonicalValueStep::Complete(checkpoint) => {
                        self.record(1, checkpoint.processed_bytes.saturating_sub(processed_bytes))?;
                        self.canonical = self.admission.as_mut().and_then(ArtifactCanonicalValueAdmission::take_value);
                        self.phase = Phase::AdmissionClose;
                    }
                }
            }
            Phase::AdmissionClose => {
                let admission = self.admission.as_mut().ok_or_else(|| "wires-publication.admission-missing".to_string())?;
                match admission.close_step(ArtifactCanonicalValueGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }) {
                    ArtifactCanonicalValueCloseStep::Blocked(_) => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
                    ArtifactCanonicalValueCloseStep::Progress { structural_items, .. } => self.record(structural_items, 0)?,
                    ArtifactCanonicalValueCloseStep::Returned { source, structural_items, .. } => {
                        self.record(structural_items, 0)?;
                        self.retire_owned(Arc::try_unwrap(source).map_err(|_| "wires-publication.value-alias".to_string())?);
                    }
                    ArtifactCanonicalValueCloseStep::Complete(_) => {
                        let admission = self.admission.take().ok_or_else(|| "wires-publication.admission-missing".to_string())?;
                        if !admission.terminal_is_empty() {
                            return Err("wires-publication.admission-witness".into());
                        }
                        self.phase = if matches!(self.clone_target, Some(CloneTarget::Node(_) | CloneTarget::Edge(_))) {
                            self.json_cursor = ArtifactCanonicalJsonCursor::default();
                            self.json_length = 0;
                            Phase::CountJson
                        } else {
                            Phase::Commit
                        };
                    }
                }
            }
            Phase::Commit => {
                self.commit()?;
                self.record(1, 0)?;
            }
            Phase::CountJson => {
                let canonical = self.canonical.as_ref().ok_or_else(|| "wires-publication.canonical-missing".to_string())?;
                let mut output = [0; store::ARTIFACT_CANONICAL_JSON_CHUNK_BYTES];
                let output_length = grant.maximum_bytes.min(output.len());
                let written = self.json_cursor.encode_chunk(canonical, &mut output[..output_length]).map_err(|error| error.reason)?;
                if written == 0 && !self.json_cursor.is_complete() {
                    return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
                }
                self.json_length = self.json_length.checked_add(written).ok_or_else(|| "wires-publication.byte-overflow".to_string())?;
                self.record(usize::from(written == 0), written)?;
                if self.json_cursor.is_complete() {
                    self.phase = Phase::ReserveJson;
                }
            }
            Phase::ReserveJson => {
                let remaining = MAXIMUM_RETAINED_BYTES.checked_sub(self.retained_bytes).ok_or_else(|| "wires-publication.retained-limit".to_string())?;
                if self.json_length > remaining {
                    return Err("wires-publication.retained-limit".into());
                }
                let mut json = String::new();
                json.try_reserve_exact(self.json_length).map_err(|_| "wires-publication.allocation-failed".to_string())?;
                if json.capacity() > remaining {
                    return Err("wires-publication.retained-limit".into());
                }
                self.retained_bytes += json.capacity();
                self.json = Some(json);
                self.utf8 = Utf8Carry::default();
                self.json_cursor = ArtifactCanonicalJsonCursor::default();
                self.phase = Phase::FillJson;
                self.record(1, 0)?;
            }
            Phase::FillJson => {
                let canonical = self.canonical.as_ref().ok_or_else(|| "wires-publication.canonical-missing".to_string())?;
                let mut output = [0; store::ARTIFACT_CANONICAL_JSON_CHUNK_BYTES];
                let output_length = grant.maximum_bytes.min(output.len());
                let written = self.json_cursor.encode_chunk(canonical, &mut output[..output_length]).map_err(|error| error.reason)?;
                if written == 0 && !self.json_cursor.is_complete() {
                    return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
                }
                self.utf8.push(self.json.as_mut().ok_or_else(|| "wires-publication.json-missing".to_string())?, &output[..written])?;
                self.record(usize::from(written == 0), written)?;
                if self.json_cursor.is_complete() {
                    if self.utf8.length != 0 || self.json.as_ref().map(String::len) != Some(self.json_length) {
                        return Err("wires-publication.json-length".into());
                    }
                    self.phase = Phase::Commit;
                }
            }
            Phase::Hash => {
                let mut output = [0; store::ARTIFACT_CANONICAL_JSON_CHUNK_BYTES];
                let output_length = grant.maximum_bytes.min(output.len());
                let candidate = self.candidate.as_ref().ok_or_else(|| "wires-publication.candidate-missing".to_string())?;
                let projection = Projection { candidate };
                let written = self.json_cursor.encode_chunk(&projection, &mut output[..output_length]).map_err(|error| error.reason)?;
                if written == 0 && !self.json_cursor.is_complete() {
                    return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
                }
                self.hasher.write(&output[..written]);
                self.record(usize::from(written == 0), written)?;
                if self.json_cursor.is_complete() {
                    self.hasher.write_u8(0xff);
                    self.phase = Phase::BuildSeal;
                }
            }
            Phase::BuildSeal => {
                let content_hash = self.hasher.finish();
                self.build_edit(content_hash)?;
                self.record(1, 0)?;
                self.seal_base = Some(self.checkpoint);
            }
            Phase::Seal => {
                if let Some(step) = self.close_active(grant)? {
                    if !matches!(step, SnapshotRetirementStep::Complete) {
                        return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
                    }
                }
                let sealer = self.sealer.as_mut().ok_or_else(|| "wires-publication.sealer-missing".to_string())?;
                match sealer.advance(grant)? {
                    ArtifactStoreOneItemPreparationStep::Blocked => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
                    ArtifactStoreOneItemPreparationStep::Progress(checkpoint) => {
                        let base = self.seal_base.ok_or_else(|| "wires-publication.seal-base-missing".to_string())?;
                        self.checkpoint = compose_checkpoint(base, checkpoint)?;
                    }
                    ArtifactStoreOneItemPreparationStep::Prepared(checkpoint) => {
                        let base = self.seal_base.ok_or_else(|| "wires-publication.seal-base-missing".to_string())?;
                        self.checkpoint = compose_checkpoint(base, checkpoint)?;
                        self.phase = Phase::Ready;
                        return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
                    }
                }
            }
            Phase::Ready => return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
            Phase::Closing | Phase::Closed => return Ok(ArtifactStoreOneItemPreparationStep::Blocked),
        }
        Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<WiresSnapshot, crate::op::WiresMutation>> {
        self.sealer.as_ref().and_then(ArtifactStoreOneItemSealer::prepared)
    }

    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<WiresSnapshot, crate::op::WiresMutation>> {
        self.sealer.as_mut().and_then(ArtifactStoreOneItemSealer::take_prepared)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(cursor) = self.clone_cursor.as_mut() {
            cursor.cancel();
        }
        if let Some(admission) = self.admission.as_mut() {
            admission.cancel();
        }
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn begin_close(&mut self) {
        self.cancel();
        self.closing = true;
        self.phase = Phase::Closing;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.begin_close();
        }
    }

    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(step) = self.close_active(grant)? {
            return Ok(step);
        }
        if let Some(sealer) = self.sealer.as_mut() {
            let step = sealer.close_step(grant)?;
            if matches!(step, SnapshotRetirementStep::Complete) {
                if !sealer.terminal_is_empty() {
                    return Err("wires-publication.sealer-witness".into());
                }
                self.sealer = None;
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(cursor) = self.clone_cursor.as_mut() {
            return Ok(match cursor.close_step(DslValueCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }) {
                DslValueCloneCloseStep::Blocked(_) => SnapshotRetirementStep::Blocked,
                DslValueCloneCloseStep::Progress { receipt, .. } => SnapshotRetirementStep::Pending { released_items: receipt.structural_items, released_bytes: receipt.copied_bytes },
                DslValueCloneCloseStep::Returned { source, receipt, .. } => {
                    self.restore_source(source)?;
                    let cursor = self.clone_cursor.take().ok_or_else(|| "wires-publication.clone-missing".to_string())?;
                    if !cursor.terminal_is_empty() {
                        return Err("wires-publication.clone-witness".into());
                    }
                    SnapshotRetirementStep::Pending { released_items: receipt.structural_items, released_bytes: receipt.copied_bytes }
                }
                DslValueCloneCloseStep::Complete(_) => {
                    let cursor = self.clone_cursor.take().ok_or_else(|| "wires-publication.clone-missing".to_string())?;
                    if !cursor.terminal_is_empty() {
                        return Err("wires-publication.clone-witness".into());
                    }
                    SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
            });
        }
        if let Some(admission) = self.admission.as_mut() {
            return Ok(match admission.close_step(ArtifactCanonicalValueGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }) {
                ArtifactCanonicalValueCloseStep::Blocked(_) => SnapshotRetirementStep::Blocked,
                ArtifactCanonicalValueCloseStep::Progress { structural_items, .. } => SnapshotRetirementStep::Pending { released_items: structural_items, released_bytes: 0 },
                ArtifactCanonicalValueCloseStep::Returned { source, structural_items, .. } => {
                    let admission = self.admission.take().ok_or_else(|| "wires-publication.admission-missing".to_string())?;
                    if !admission.terminal_is_empty() {
                        return Err("wires-publication.admission-witness".into());
                    }
                    self.retire_owned(Arc::try_unwrap(source).map_err(|_| "wires-publication.value-alias".to_string())?);
                    SnapshotRetirementStep::Pending { released_items: structural_items, released_bytes: 0 }
                }
                ArtifactCanonicalValueCloseStep::Complete(_) => {
                    let admission = self.admission.take().ok_or_else(|| "wires-publication.admission-missing".to_string())?;
                    if !admission.terminal_is_empty() {
                        return Err("wires-publication.admission-witness".into());
                    }
                    SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }
                }
            });
        }
        if let Some(canonical) = self.canonical.take() {
            self.retire_owned(Arc::try_unwrap(canonical.into_source()).map_err(|_| "wires-publication.value-alias".to_string())?);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(value) = self.pending_value.take() {
            self.retire_owned(value);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(candidate) = self.candidate.take() {
            self.retire_owned(candidate);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(mutation) = self.mutation.take() {
            self.retire_owned(mutation);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(description) = self.description.take() {
            self.retire_owned(description);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(json) = self.json.take() {
            self.retire_owned(json);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.scene.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("wires-publication.base-return".into());
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.take() {
            self.active_retirement = Some(authority.retire());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.phase = Phase::Closed;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == Phase::Closed
            && self.base.is_none()
            && self.scene.is_none()
            && self.mutation.is_none()
            && self.description.is_none()
            && self.authority.is_none()
            && self.candidate.is_none()
            && self.clone_cursor.is_none()
            && self.pending_value.is_none()
            && self.admission.is_none()
            && self.canonical.is_none()
            && self.json.is_none()
            && self.sealer.is_none()
            && self.active_retirement.is_none()
    }
}

impl Drop for WiresStorePreparation {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Wires publication dropped before exact owners reached terminal-empty");
    }
}
