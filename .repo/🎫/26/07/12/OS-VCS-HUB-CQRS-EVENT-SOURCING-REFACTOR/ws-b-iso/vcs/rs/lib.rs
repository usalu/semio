//! 🗄️ Generic document VCS engine — Operation/Edit/Change/Checkpoint/Alternative, materialize-by-replay, backbone.

use semio_framework_core::{
    ActorId, DocumentDiff, DocumentId, DocumentVersion, HybridLogicalTimestamp, InverseOperation,
    MergeStrategyKind, OpEnvelope, OperationId, PayloadHash, SchemaId, SchemaVersion, UndoPolicy,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// @emoji 🆔 Allocates stable ids for document VCS entities.
pub fn create_document_vcs_id(prefix: &str) -> String {
    let n = ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

//#region 🔖Schemas
/// @emoji 🔗 Identifies the channel a document synchronizes through, when one is attached.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentBackboneRef {
    pub uri: String,
}

/// @emoji 🔗 Builds a backbone reference from a channel URI.
pub fn document_backbone_ref(uri: &str) -> DocumentBackboneRef {
    DocumentBackboneRef { uri: uri.to_string() }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationMeta {
    pub operation_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    pub base_version: u64,
    pub author_id: String,
    pub timestamp: HybridLogicalTimestamp,
    pub undo_policy: UndoPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit<Op> {
    pub id: String,
    /// @emoji 🖋️ Authoring actor id. Local edits carry the dispatching actor; ingested edits carry
    /// the incoming {@link OpEnvelope}'s actor. Drives {@link UndoPolicy} (foreign edits are never
    /// undone locally). `None` means unauthored/legacy and is treated as local.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub forwards: Vec<Op>,
    pub backwards: Vec<Op>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation_meta: Vec<OperationMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// @emoji 🪢 Gesture identity used by `AmendLast` to absorb follow-up operations into this edit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coalesce_key: Option<String>,
    pub sequence_number: i32,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub id: String,
    pub edit_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub saved_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checkpoint {
    pub id: String,
    pub change_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentVcs<P, Op> {
    pub initial_projection: P,
    pub edits: Vec<Edit<Op>>,
    pub changes: Vec<Change>,
    pub checkpoints: Vec<Checkpoint>,
    pub alternatives: Vec<Alternative>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentVcsEnvelope<P, Op> {
    pub schema: String,
    pub id: String,
    pub vcs: DocumentVcs<P, Op>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backbone: Option<DocumentBackboneRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentVcsCommand<Op> {
    Apply {
        operations: Vec<Op>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    Undo,
    Redo,
    UndoWithPolicy {
        policy: UndoPolicy,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        semantic_command: Option<String>,
    },
    CommitCheckpoint {
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        authors: Vec<Author>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        alternative_id: String,
    },
    CheckoutCheckpoint {
        #[serde(rename = "checkpointId")]
        checkpoint_id: String,
    },
    AmendLast {
        operations: Vec<Op>,
        /// @emoji 🪢 Matches the last uncommitted edit's `coalesce_key` to absorb into it instead of creating a new edit.
        coalesce_key: Option<String>,
    },
}
//#endregion 🔖Schemas

//#region 🔖Errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum VcsError {
    #[error("unknown edit id: {0}")]
    UnknownEdit(String),
    #[error("unknown change id: {0}")]
    UnknownChange(String),
    #[error("unknown alternative id: {0}")]
    UnknownAlternative(String),
    #[error("no checkpoint for alternative")]
    NoCheckpoint,
    #[error("empty apply command")]
    EmptyApply,
    #[error("nothing to undo")]
    NothingToUndo,
    #[error("cannot undo edit authored by another actor: {0}")]
    ForeignEdit(String),
    #[error("nothing to redo")]
    NothingToRedo,
    #[error("serialize error: {0}")]
    Serialize(String),
    #[error("deserialize error: {0}")]
    Deserialize(String),
    #[error("backbone error: {0}")]
    Backbone(String),
    #[error("remote sync not implemented")]
    RemoteSyncNotImplemented,
}
//#endregion 🔖Errors

//#region 🔖CollectionDiff
/// @emoji 🧩 Sparse collection patch entry (mirrors compose `XModified`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {
    pub id: TId,
    pub patch: TPatch,
}

/// @emoji 🧩 Sparse collection diff (mirrors compose `XCollectionDiff`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDiff<TId, TPatch, TAdded> {
    pub removed: Vec<TId>,
    pub modified: Vec<ItemPatch<TId, TPatch>>,
    pub added: Vec<TAdded>,
}

impl<TId, TPatch, TAdded> Default for CollectionDiff<TId, TPatch, TAdded> {
    fn default() -> Self {
        Self {
            removed: Vec::new(),
            modified: Vec::new(),
            added: Vec::new(),
        }
    }
}
//#endregion 🔖CollectionDiff

//#region 🔖CollectionOp
/// @emoji 🏷️ Identifies an item within a `Vec` by a stable id, for generic collection ops.
pub trait Identified<TId> {
    fn id(&self) -> &TId;
}

/// @emoji 🩹 Applies a patch in place and returns the patch that undoes it (captured from prior state).
pub trait Patchable<TPatch> {
    fn apply_patch(&mut self, patch: &TPatch) -> TPatch;
}

/// @emoji 🧺 Generic ordered-collection operation (add/remove/move/patch) with mechanical pre-state inverses.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionOp<TId, TItem, TPatch> {
    Add { index: usize, item: TItem },
    Remove { id: TId },
    Move { id: TId, to_index: usize },
    Patch { id: TId, patch: TPatch },
}

/// @emoji ▶️ Applies a `CollectionOp` to a `Vec` in place.
pub fn apply_collection_op<TId, TItem, TPatch>(items: &mut Vec<TItem>, op: &CollectionOp<TId, TItem, TPatch>)
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match op {
        CollectionOp::Add { index, item } => {
            let at = (*index).min(items.len());
            items.insert(at, item.clone());
        }
        CollectionOp::Remove { id } => {
            items.retain(|item| item.id() != id);
        }
        CollectionOp::Move { id, to_index } => {
            if let Some(from) = items.iter().position(|item| item.id() == id) {
                let item = items.remove(from);
                let at = (*to_index).min(items.len());
                items.insert(at, item);
            }
        }
        CollectionOp::Patch { id, patch } => {
            if let Some(item) = items.iter_mut().find(|item| item.id() == id) {
                item.apply_patch(patch);
            }
        }
    }
}

/// @emoji ↩️ Computes the inverse `CollectionOp` from the pre-state `items`. Panics if `op` targets
/// an id absent from `items` (Remove/Move/Patch always target an existing item by construction).
pub fn invert_collection_op<TId, TItem, TPatch>(
    items: &[TItem],
    op: &CollectionOp<TId, TItem, TPatch>,
) -> CollectionOp<TId, TItem, TPatch>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    match op {
        CollectionOp::Add { item, .. } => CollectionOp::Remove { id: item.id().clone() },
        CollectionOp::Remove { id } => {
            let index = items
                .iter()
                .position(|item| item.id() == id)
                .expect("remove target must exist in pre-state");
            CollectionOp::Add {
                index,
                item: items[index].clone(),
            }
        }
        CollectionOp::Move { id, .. } => {
            let index = items
                .iter()
                .position(|item| item.id() == id)
                .expect("move target must exist in pre-state");
            CollectionOp::Move {
                id: id.clone(),
                to_index: index,
            }
        }
        CollectionOp::Patch { id, patch } => {
            let mut prior = items
                .iter()
                .find(|item| item.id() == id)
                .cloned()
                .expect("patch target must exist in pre-state");
            let inverse_patch = prior.apply_patch(patch);
            CollectionOp::Patch {
                id: id.clone(),
                patch: inverse_patch,
            }
        }
    }
}

/// @emoji 🧮 Projects a `CollectionOp` onto a sparse {@link CollectionDiff}, so a plugin's
/// `Operation::diff` can produce a diff in one call instead of hand-writing `removed`/`modified`/
/// `added`. `Add` → `added`, `Remove` → `removed`, `Patch` → `modified`. `CollectionDiff` has no
/// positional-move channel, so `Move` is encoded as `removed` + `added` (delete then re-add by
/// identity); a plugin that keeps items keyed by id reconstructs order from item identity.
pub fn collection_diff_from_op<TId, TItem, TPatch>(
    items: &[TItem],
    op: &CollectionOp<TId, TItem, TPatch>,
) -> CollectionDiff<TId, TPatch, TItem>
where
    TId: PartialEq + Clone,
    TItem: Identified<TId> + Clone,
    TPatch: Clone,
{
    let mut diff = CollectionDiff::default();
    match op {
        CollectionOp::Add { item, .. } => diff.added.push(item.clone()),
        CollectionOp::Remove { id } => diff.removed.push(id.clone()),
        CollectionOp::Patch { id, patch } => diff.modified.push(ItemPatch {
            id: id.clone(),
            patch: patch.clone(),
        }),
        CollectionOp::Move { id, .. } => {
            if let Some(item) = items.iter().find(|item| item.id() == id) {
                diff.removed.push(id.clone());
                diff.added.push(item.clone());
            }
        }
    }
    diff
}
//#endregion 🔖CollectionOp

//#region 🔖Operation
/// @emoji 📦 Centralized projection mutation — one `apply` per technology.
pub trait OperationDiff<P>: Clone + Default + Serialize + DeserializeOwned {
    fn apply(&self, projection: &P) -> P;
    fn absorb(&mut self, other: Self);
}

/// @emoji 🔁 Stored operation: emits a diff and computes backwards from pre-state.
pub trait Operation<P>: Clone + Serialize + DeserializeOwned {
    type Diff: OperationDiff<P>;
    fn diff(&self, projection: &P) -> Self::Diff;
    fn backwards(&self, projection: &P) -> Vec<Self>;
    fn operation_id(&self) -> Option<String> {
        None
    }
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
    fn base_version(&self) -> u64 {
        0
    }
    fn author_id(&self) -> Option<String> {
        None
    }
    fn timestamp(&self) -> Option<HybridLogicalTimestamp> {
        None
    }
    fn undo_policy(&self) -> UndoPolicy {
        UndoPolicy::ExactBaseOnly
    }
    fn merge_strategy(&self) -> MergeStrategyKind {
        MergeStrategyKind::LwwRegister
    }
}

pub fn apply_operation<P, Op>(projection: &P, operation: &Op) -> P
where
    Op: Operation<P>,
{
    operation.diff(projection).apply(projection)
}

pub fn absorb_diff<P, Op>(projection: &P, existing: &mut Op::Diff, incoming: Op::Diff)
where
    Op: Operation<P>,
{
    existing.absorb(incoming);
}
//#endregion 🔖Operation

//#region 🔖MergeStrategy
pub fn merge_concurrent_diffs<P, Op>(
    projection: &P,
    strategy: MergeStrategyKind,
    existing: &mut Op::Diff,
    incoming: Op::Diff,
) where
    Op: Operation<P>,
{
    match strategy {
        MergeStrategyKind::LwwRegister | MergeStrategyKind::OrderedSequence
        | MergeStrategyKind::TextSequence | MergeStrategyKind::TombstonedGraphSet
        | MergeStrategyKind::ContentAddressedBlob => {
            existing.absorb(incoming);
        }
    }
}

pub fn reconcile_alternative<P, Op>(
    envelope: &mut DocumentVcsEnvelope<P, Op>,
    alternative_name: &str,
    checkpoint_message: Option<String>,
    authors: Vec<Author>,
) -> Result<String, VcsError>
where
    P: Clone + Serialize + DeserializeOwned,
    Op: Clone + Serialize + DeserializeOwned,
{
    if envelope.vcs.checkpoints.is_empty() {
        return Err(VcsError::NoCheckpoint);
    }
    let checkpoint_id = envelope
        .vcs
        .checkpoints
        .last()
        .map(|checkpoint| checkpoint.id.clone())
        .ok_or(VcsError::NoCheckpoint)?;
    let alternative_id = create_document_vcs_id("alternative");
    envelope.vcs.alternatives.push(Alternative {
        id: alternative_id.clone(),
        name: alternative_name.to_string(),
        checkpoint_ids: vec![checkpoint_id],
    });
    if let Some(message) = checkpoint_message {
        let change = Change {
            id: create_document_vcs_id("change"),
            edit_ids: Vec::new(),
            description: Some(message),
            saved_at: now_iso(),
        };
        let parent = envelope.vcs.checkpoints.last();
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        change_ids.push(change.id.clone());
        envelope.vcs.changes.push(change);
        envelope.vcs.checkpoints.push(Checkpoint {
            id: create_document_vcs_id("checkpoint"),
            change_ids,
            parent_id: parent.map(|checkpoint| checkpoint.id.clone()),
            authors,
            message: Some("reconciled".into()),
            timestamp: now_iso(),
        });
    }
    Ok(alternative_id)
}
//#endregion 🔖MergeStrategy

//#region 🔖Materialize
pub fn create_document_vcs_envelope<P, Op>(
    schema: &str,
    id: &str,
    initial_projection: P,
    backbone: Option<DocumentBackboneRef>,
) -> DocumentVcsEnvelope<P, Op>
where
    P: Clone,
{
    DocumentVcsEnvelope {
        schema: schema.into(),
        id: id.into(),
        vcs: DocumentVcs {
            initial_projection,
            edits: Vec::new(),
            changes: Vec::new(),
            checkpoints: Vec::new(),
            alternatives: Vec::new(),
        },
        backbone,
        active_alternative_id: None,
    }
}

pub fn edit_ids_for_changes<P, Op>(envelope: &DocumentVcsEnvelope<P, Op>, change_ids: &[String]) -> Vec<String>
where
    Op: Clone,
    P: Clone,
{
    let mut edit_ids = Vec::new();
    for change_id in change_ids {
        if let Some(change) = envelope.vcs.changes.iter().find(|entry| entry.id == *change_id) {
            edit_ids.extend(change.edit_ids.iter().cloned());
        }
    }
    edit_ids
}

pub fn materialize_document_projection<P, Op>(
    envelope: &DocumentVcsEnvelope<P, Op>,
    applied_edit_ids: &[String],
) -> Result<P, VcsError>
where
    P: Clone,
    Op: Operation<P>,
{
    let mut projection = envelope.vcs.initial_projection.clone();
    for edit_id in applied_edit_ids {
        let edit = envelope
            .vcs
            .edits
            .iter()
            .find(|entry| entry.id == *edit_id)
            .ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for operation in &edit.forwards {
            projection = apply_operation(&projection, operation);
        }
    }
    Ok(projection)
}

fn now_iso() -> String {
    format!("{}", now_ms())
}

fn now_ms() -> u64 {
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as u64)
            .unwrap_or(0);
    }
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now() as u64
    }
}

fn uncommitted_edit_ids<P, Op>(envelope: &DocumentVcsEnvelope<P, Op>, applied_edit_ids: &[String]) -> Vec<String>
where
    Op: Clone,
    P: Clone,
{
    let committed: std::collections::HashSet<String> = envelope
        .vcs
        .changes
        .iter()
        .flat_map(|change| change.edit_ids.iter().cloned())
        .collect();
    applied_edit_ids
        .iter()
        .filter(|id| !committed.contains(*id))
        .cloned()
        .collect()
}

//#endregion 🔖Materialize

//#region 🔖History
/// @emoji 📜 One row of a checkpoint history/ancestor graph. Mirrors premigration `HistoryColumn`
/// (`vcs/core/js/internal.ts`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryColumn {
    pub checkpoint_id: String,
    pub timestamp: String,
    pub labels: Vec<String>,
    pub authors: Vec<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub lane: usize,
    pub alternative_ids: Vec<String>,
}

fn checkpoint_alternatives<'a, P, Op>(
    envelope: &'a DocumentVcsEnvelope<P, Op>,
    checkpoint_id: &str,
) -> Vec<&'a Alternative> {
    envelope
        .vcs
        .alternatives
        .iter()
        .filter(|alternative| alternative.checkpoint_ids.iter().any(|id| id == checkpoint_id))
        .collect()
}

fn is_checkpoint_main_only<P, Op>(envelope: &DocumentVcsEnvelope<P, Op>, checkpoint_id: &str) -> bool {
    checkpoint_alternatives(envelope, checkpoint_id).is_empty()
}

fn has_main_only_descendant<P, Op>(
    envelope: &DocumentVcsEnvelope<P, Op>,
    children_of: &HashMap<String, Vec<String>>,
    checkpoint_id: &str,
    seen: &mut HashSet<String>,
) -> bool {
    if !seen.insert(checkpoint_id.to_string()) {
        return false;
    }
    for child_id in children_of.get(checkpoint_id).into_iter().flatten() {
        if is_checkpoint_main_only(envelope, child_id) || has_main_only_descendant(envelope, children_of, child_id, seen) {
            return true;
        }
    }
    false
}

/// @emoji 🛤️ Assigns each checkpoint a swimlane: alternatives get lanes `1..n` in array order, lane
/// `0` is the main trunk. A checkpoint sits on lane 0 if it belongs to no alternative or has any
/// main-only descendant (cycle-guarded DFS); otherwise it takes its single alternative's lane, or
/// the minimum lane among several. Mirrors premigration `assignHistoryCheckpointLanes`.
fn assign_history_checkpoint_lanes<P, Op>(envelope: &DocumentVcsEnvelope<P, Op>) -> HashMap<String, usize> {
    let mut lane_by_alternative: HashMap<String, usize> = HashMap::new();
    for (index, alternative) in envelope.vcs.alternatives.iter().enumerate() {
        lane_by_alternative.insert(alternative.id.clone(), index + 1);
    }
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if let Some(parent_id) = &checkpoint.parent_id {
            children_of.entry(parent_id.clone()).or_default().push(checkpoint.id.clone());
        }
    }
    let mut lane_by_checkpoint_id: HashMap<String, usize> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if checkpoint.parent_id.is_none() {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let mut seen = HashSet::new();
        if is_checkpoint_main_only(envelope, &checkpoint.id)
            || has_main_only_descendant(envelope, &children_of, &checkpoint.id, &mut seen)
        {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
        let lanes: Vec<usize> = alternatives
            .iter()
            .map(|alternative| *lane_by_alternative.get(&alternative.id).unwrap_or(&0))
            .collect();
        let lane = if lanes.len() == 1 {
            lanes[0]
        } else {
            lanes.into_iter().min().unwrap_or(0)
        };
        lane_by_checkpoint_id.insert(checkpoint.id.clone(), lane);
    }
    lane_by_checkpoint_id
}

/// @emoji 📜 Builds the ancestor-graph rows for a checkpoint history view: newest checkpoint first,
/// each carrying its swimlane, labels (alternative names, `"main"` fallback on the newest unlabeled
/// row), and authors. Mirrors premigration `buildHistoryColumns`.
pub fn build_history_columns<P, Op>(envelope: &DocumentVcsEnvelope<P, Op>) -> Vec<HistoryColumn> {
    let lane_by_checkpoint_id = assign_history_checkpoint_lanes(envelope);
    envelope
        .vcs
        .checkpoints
        .iter()
        .rev()
        .enumerate()
        .map(|(index, checkpoint)| {
            let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
            let alternative_ids: Vec<String> = alternatives.iter().map(|alternative| alternative.id.clone()).collect();
            let mut labels: Vec<String> = alternatives.iter().map(|alternative| alternative.name.clone()).collect();
            if labels.is_empty() && index == 0 {
                labels.push("main".into());
            }
            HistoryColumn {
                checkpoint_id: checkpoint.id.clone(),
                timestamp: checkpoint.timestamp.clone(),
                labels,
                authors: checkpoint.authors.clone(),
                parent_checkpoint_id: checkpoint.parent_id.clone(),
                description: checkpoint.message.clone(),
                lane: *lane_by_checkpoint_id.get(&checkpoint.id).unwrap_or(&0),
                alternative_ids,
            }
        })
        .collect()
}
//#endregion 🔖History

//#region 🔖DocumentVcsStore
pub struct DocumentVcsStore<P, Op>
where
    P: Clone + Serialize + DeserializeOwned,
    Op: Clone + Serialize + DeserializeOwned + Operation<P>,
{
    envelope: DocumentVcsEnvelope<P, Op>,
    backbone: Option<Box<dyn Backbone>>,
    dag: semio_framework_core::OpDag,
    applied_edit_ids: Vec<String>,
    redo_edit_ids: Vec<String>,
    edit_sequence: i32,
    generation: u64,
    /// @emoji 🧭 The checkpoint new commits parent onto; advances on commit/checkout/switch. Not
    /// part of the wire envelope — callers that reconstruct the store per call (e.g. a WASM plugin)
    /// must save/restore it themselves via {@link current_checkpoint_id}/{@link set_current_checkpoint_id}.
    current_checkpoint_id: Option<String>,
    /// @emoji 🖋️ Identity of the local actor driving this store. Set from each local `Apply`/
    /// `AmendLast`'s operation author; compared against `Edit.actor` so undo never touches foreign
    /// edits. Not part of the wire envelope — callers that reconstruct the store per call must
    /// save/restore it via {@link local_actor_id}/{@link set_local_actor_id}.
    local_actor_id: Option<String>,
}

/// @emoji 🖋️ Derives an edit's authoring actor from its per-operation metadata (the author of its
/// first operation), so a local edit records who produced it for later `UndoPolicy` classification.
fn edit_actor_from_meta(operation_meta: &[OperationMeta]) -> Option<String> {
    operation_meta.first().map(|meta| meta.author_id.clone())
}

/// @emoji 🔌 Auto-attaches a backbone from a deserialized envelope. Inside the wasm sandbox this
/// resolves the injected {@link PortBackbone} (a pure in-memory queue relayed to the host). On
/// native targets it never resolves IO — backbone attachment is an explicit `attach_backbone`
/// call made by the caller (the `framework/sync` actor layer), so deserializing an envelope never
/// performs filesystem/HTTP work in this crate.
fn auto_attach_backbone<P, Op>(envelope: &DocumentVcsEnvelope<P, Op>) -> Option<Box<dyn Backbone>> {
    #[cfg(target_arch = "wasm32")]
    {
        return envelope
            .backbone
            .as_ref()
            .and_then(|entry| resolve_backbone(&entry.uri).ok());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = envelope;
        None
    }
}

impl<P, Op> DocumentVcsStore<P, Op>
where
    P: Clone + Serialize + DeserializeOwned,
    Op: Clone + Serialize + DeserializeOwned + Operation<P>,
{
    pub fn new(envelope: DocumentVcsEnvelope<P, Op>) -> Self {
        let backbone = auto_attach_backbone(&envelope);
        let current_checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        Self {
            envelope,
            backbone,
            dag: semio_framework_core::OpDag::new(),
            applied_edit_ids: Vec::new(),
            redo_edit_ids: Vec::new(),
            edit_sequence: 0,
            generation: 0,
            current_checkpoint_id,
            local_actor_id: None,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn envelope(&self) -> &DocumentVcsEnvelope<P, Op> {
        &self.envelope
    }

    pub fn applied_edit_ids(&self) -> &[String] {
        &self.applied_edit_ids
    }

    /// @emoji ↪️ Pending redo stack (edit ids undone since the last fresh `Apply`).
    pub fn redo_edit_ids(&self) -> &[String] {
        &self.redo_edit_ids
    }

    /// @emoji 🧭 The checkpoint new commits currently parent onto (defaults to the latest checkpoint
    /// on construction/`set_state`; advances on commit/checkout/switch).
    pub fn current_checkpoint_id(&self) -> Option<&str> {
        self.current_checkpoint_id.as_deref()
    }

    /// @emoji 🧭 Restores the checkout position after reconstructing the store from a serialized
    /// envelope (`set_state` resets it to the latest checkpoint, which is wrong once a caller has
    /// checked out an older one).
    pub fn set_current_checkpoint_id(&mut self, checkpoint_id: Option<String>) {
        self.current_checkpoint_id = checkpoint_id;
    }

    /// @emoji 🖋️ The local actor id used to distinguish this store's own edits from ingested ones.
    /// Not part of the wire envelope — a caller reconstructing the store per call must save/restore
    /// it via {@link set_local_actor_id} for `UndoPolicy` to keep classifying foreign edits.
    pub fn local_actor_id(&self) -> Option<&str> {
        self.local_actor_id.as_deref()
    }

    /// @emoji 🖋️ Sets the local actor id (see {@link local_actor_id}). Called automatically from each
    /// local `Apply`/`AmendLast`; callers that reconstruct the store per dispatch restore it here.
    pub fn set_local_actor_id(&mut self, actor_id: Option<String>) {
        self.local_actor_id = actor_id;
    }

    /// @emoji 🔧 The most recently created/amended edit's `(forwards, backwards, per-operation meta)`.
    /// Used right after `dispatch(Apply{..})`/`AmendLast` to build a `KernelOperation`/`ActionResult`
    /// with a true inverse from the just-recorded `Edit.backwards`.
    pub fn edit_operations(&self) -> Option<(&[Op], &[Op], &[OperationMeta])> {
        self.envelope.vcs.edits.last().map(|edit| {
            (
                edit.forwards.as_slice(),
                edit.backwards.as_slice(),
                edit.operation_meta.as_slice(),
            )
        })
    }

    /// @emoji 📜 Ancestor-graph rows for this store's checkpoint history. See {@link build_history_columns}.
    pub fn history_columns(&self) -> Vec<HistoryColumn> {
        build_history_columns(&self.envelope)
    }

    pub fn set_envelope(&mut self, envelope: DocumentVcsEnvelope<P, Op>, applied_edit_ids: Vec<String>) {
        self.set_state(envelope, applied_edit_ids, Vec::new());
    }

    /// @emoji 💾 Restores full store state including the redo stack, so `Redo` survives
    /// round-tripping through a serialized envelope (e.g. one `dispatch` call per request).
    pub fn set_state(
        &mut self,
        envelope: DocumentVcsEnvelope<P, Op>,
        applied_edit_ids: Vec<String>,
        redo_edit_ids: Vec<String>,
    ) {
        self.backbone = auto_attach_backbone(&envelope);
        self.edit_sequence = envelope
            .vcs
            .edits
            .iter()
            .map(|edit| edit.sequence_number)
            .max()
            .unwrap_or(0);
        self.current_checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        self.envelope = envelope;
        self.applied_edit_ids = applied_edit_ids;
        self.redo_edit_ids = redo_edit_ids;
        self.bump();
    }

    /// @emoji 🧭 Restores applied edits + checkout position for `checkpoint_id`, clearing redo.
    /// Shared by `createAlternative`/`switchAlternative`/`checkoutCheckpoint`. Mirrors premigration
    /// `checkoutCheckpointInternal`.
    fn checkout_checkpoint_internal(&mut self, checkpoint_id: String) {
        let applied = self
            .envelope
            .vcs
            .checkpoints
            .iter()
            .find(|checkpoint| checkpoint.id == checkpoint_id)
            .map(|checkpoint| edit_ids_for_changes(&self.envelope, &checkpoint.change_ids))
            .unwrap_or_default();
        self.applied_edit_ids = applied;
        self.redo_edit_ids.clear();
        self.current_checkpoint_id = Some(checkpoint_id);
    }

    pub fn projection(&self) -> Result<P, VcsError> {
        materialize_document_projection(&self.envelope, &self.applied_edit_ids)
    }

    pub fn dispatch(&mut self, command: DocumentVcsCommand<Op>) -> Result<(), VcsError> {
        self.pump()?;
        let is_apply = matches!(command, DocumentVcsCommand::Apply { .. });
        self.dispatch_inner(command)?;
        self.flush_outbound(is_apply)
    }

    fn dispatch_inner(&mut self, command: DocumentVcsCommand<Op>) -> Result<(), VcsError> {
        match command {
            DocumentVcsCommand::Undo => self.dispatch(DocumentVcsCommand::UndoWithPolicy {
                policy: UndoPolicy::ExactBaseOnly,
                semantic_command: None,
            }),
            DocumentVcsCommand::UndoWithPolicy {
                policy,
                semantic_command,
            } => match policy {
                UndoPolicy::ExactBaseOnly => {
                    let last = self.applied_edit_ids.last().cloned().ok_or(VcsError::NothingToUndo)?;
                    if !self.edit_is_local(&last) {
                        return Err(VcsError::ForeignEdit(last));
                    }
                    self.applied_edit_ids.pop();
                    self.redo_edit_ids.push(last);
                    self.bump();
                    Ok(())
                }
                UndoPolicy::TransformAgainstConcurrent => {
                    let position = self
                        .applied_edit_ids
                        .iter()
                        .rposition(|id| self.edit_is_local(id))
                        .ok_or(VcsError::NothingToUndo)?;
                    let removed = self.applied_edit_ids.remove(position);
                    self.redo_edit_ids.push(removed);
                    self.bump();
                    Ok(())
                }
                UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                    // Requires a compensating command to invert non-mechanically; no such mechanism
                    // exists in this crate yet, so this preserves the prior behavior of undoing the
                    // local tail once a `semantic_command` is supplied. A real compensating-action
                    // implementation is out of scope here (see WS-D plugin contract).
                    if semantic_command.is_none() {
                        return Err(VcsError::Backbone(
                            "semantic undo requires compensating command".into(),
                        ));
                    }
                    let last = self.applied_edit_ids.last().cloned().ok_or(VcsError::NothingToUndo)?;
                    if !self.edit_is_local(&last) {
                        return Err(VcsError::ForeignEdit(last));
                    }
                    self.applied_edit_ids.pop();
                    self.redo_edit_ids.push(last);
                    self.bump();
                    Ok(())
                }
            },
            DocumentVcsCommand::Redo => {
                let next = self.redo_edit_ids.pop().ok_or(VcsError::NothingToRedo)?;
                self.applied_edit_ids.push(next);
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::CommitCheckpoint { message, authors } => {
                let pending = uncommitted_edit_ids(&self.envelope, &self.applied_edit_ids);
                if pending.is_empty() {
                    return Ok(());
                }
                let change = Change {
                    id: create_document_vcs_id("change"),
                    edit_ids: pending,
                    description: message.clone(),
                    saved_at: now_iso(),
                };
                let parent = self
                    .current_checkpoint_id
                    .as_ref()
                    .and_then(|id| self.envelope.vcs.checkpoints.iter().find(|cp| cp.id == *id));
                let mut change_ids = parent.map(|cp| cp.change_ids.clone()).unwrap_or_default();
                let parent_id = parent.map(|cp| cp.id.clone());
                change_ids.push(change.id.clone());
                let checkpoint = Checkpoint {
                    id: create_document_vcs_id("checkpoint"),
                    change_ids,
                    parent_id,
                    authors,
                    message,
                    timestamp: now_iso(),
                };
                let checkpoint_id = checkpoint.id.clone();
                self.envelope.vcs.changes.push(change);
                self.envelope.vcs.checkpoints.push(checkpoint);
                if let Some(alternative_id) = self.envelope.active_alternative_id.clone() {
                    if let Some(alternative) = self
                        .envelope
                        .vcs
                        .alternatives
                        .iter_mut()
                        .find(|alt| alt.id == alternative_id)
                    {
                        alternative.checkpoint_ids.push(checkpoint_id.clone());
                    }
                }
                self.current_checkpoint_id = Some(checkpoint_id);
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::CreateAlternative { name } => {
                if self.envelope.vcs.checkpoints.is_empty() {
                    self.dispatch(DocumentVcsCommand::CommitCheckpoint {
                        message: None,
                        authors: Vec::new(),
                    })?;
                }
                let checkpoint_id = self
                    .current_checkpoint_id
                    .clone()
                    .or_else(|| self.envelope.vcs.checkpoints.last().map(|cp| cp.id.clone()))
                    .ok_or(VcsError::NoCheckpoint)?;
                let alt_id = create_document_vcs_id("alternative");
                self.envelope.vcs.alternatives.push(Alternative {
                    id: alt_id.clone(),
                    name,
                    checkpoint_ids: vec![checkpoint_id.clone()],
                });
                self.envelope.active_alternative_id = Some(alt_id);
                self.checkout_checkpoint_internal(checkpoint_id);
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::SwitchAlternative { alternative_id } => {
                let alternative = self
                    .envelope
                    .vcs
                    .alternatives
                    .iter()
                    .find(|alt| alt.id == alternative_id)
                    .ok_or_else(|| VcsError::UnknownAlternative(alternative_id.clone()))?
                    .clone();
                let checkpoint_id = alternative
                    .checkpoint_ids
                    .last()
                    .ok_or(VcsError::NoCheckpoint)?
                    .clone();
                if !self.envelope.vcs.checkpoints.iter().any(|cp| cp.id == checkpoint_id) {
                    return Err(VcsError::NoCheckpoint);
                }
                self.checkout_checkpoint_internal(checkpoint_id);
                self.envelope.active_alternative_id = Some(alternative_id);
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id } => {
                if !self.envelope.vcs.checkpoints.iter().any(|cp| cp.id == checkpoint_id) {
                    return Err(VcsError::UnknownChange(checkpoint_id.clone()));
                }
                self.checkout_checkpoint_internal(checkpoint_id.clone());
                self.envelope.active_alternative_id = self
                    .envelope
                    .vcs
                    .alternatives
                    .iter()
                    .find(|alt| alt.checkpoint_ids.last() == Some(&checkpoint_id))
                    .map(|alt| alt.id.clone());
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::Apply {
                operations,
                description,
            } => {
                if operations.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let started_at = now_iso();
                let pre_projection = self.projection()?;
                let (forwards, backwards, operation_meta, _post) =
                    Self::replay_operations(&pre_projection, operations);
                let actor = edit_actor_from_meta(&operation_meta);
                self.local_actor_id = actor.clone();
                self.edit_sequence += 1;
                let edit = Edit {
                    id: create_document_vcs_id("edit"),
                    actor,
                    forwards,
                    backwards,
                    operation_meta,
                    description,
                    coalesce_key: None,
                    sequence_number: self.edit_sequence,
                    started_at,
                    finished_at: Some(now_iso()),
                };
                self.applied_edit_ids.push(edit.id.clone());
                self.envelope.vcs.edits.push(edit);
                self.redo_edit_ids.clear();
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::AmendLast {
                operations,
                coalesce_key,
            } => {
                if operations.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let amend_target = self.applied_edit_ids.last().cloned().filter(|last_id| {
                    coalesce_key.is_some()
                        && uncommitted_edit_ids(&self.envelope, &self.applied_edit_ids).contains(last_id)
                        && self
                            .envelope
                            .vcs
                            .edits
                            .iter()
                            .find(|edit| edit.id == *last_id)
                            .map(|edit| edit.coalesce_key == coalesce_key)
                            .unwrap_or(false)
                });
                if let Some(edit_id) = amend_target {
                    let pre_ids = &self.applied_edit_ids[..self.applied_edit_ids.len() - 1];
                    let pre_projection = materialize_document_projection(&self.envelope, pre_ids)?;
                    let mut combined = self
                        .envelope
                        .vcs
                        .edits
                        .iter()
                        .find(|edit| edit.id == edit_id)
                        .map(|edit| edit.forwards.clone())
                        .unwrap_or_default();
                    combined.extend(operations);
                    let (forwards, backwards, operation_meta, _post) =
                        Self::replay_operations(&pre_projection, combined);
                    if let Some(edit) = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                        edit.forwards = forwards;
                        edit.backwards = backwards;
                        edit.operation_meta = operation_meta;
                        edit.finished_at = Some(now_iso());
                    }
                    self.redo_edit_ids.clear();
                    self.bump();
                    Ok(())
                } else {
                    let started_at = now_iso();
                    let pre_projection = self.projection()?;
                    let (forwards, backwards, operation_meta, _post) =
                        Self::replay_operations(&pre_projection, operations);
                    let actor = edit_actor_from_meta(&operation_meta);
                    self.local_actor_id = actor.clone();
                    self.edit_sequence += 1;
                    let edit = Edit {
                        id: create_document_vcs_id("edit"),
                        actor,
                        forwards,
                        backwards,
                        operation_meta,
                        description: None,
                        coalesce_key,
                        sequence_number: self.edit_sequence,
                        started_at,
                        finished_at: Some(now_iso()),
                    };
                    self.applied_edit_ids.push(edit.id.clone());
                    self.envelope.vcs.edits.push(edit);
                    self.redo_edit_ids.clear();
                    self.bump();
                    Ok(())
                }
            }
        }
    }

    /// @emoji 🔂 Replays `operations` over `pre_projection`, returning forwards, reversed-backwards,
    /// per-operation metadata, and the resulting projection. Shared by `Apply` and `AmendLast`.
    fn replay_operations(pre_projection: &P, operations: Vec<Op>) -> (Vec<Op>, Vec<Op>, Vec<OperationMeta>, P) {
        let mut projection = pre_projection.clone();
        let mut forwards = Vec::with_capacity(operations.len());
        let mut backwards = Vec::new();
        let mut operation_meta = Vec::with_capacity(operations.len());
        for operation in operations {
            let mut back = operation.backwards(&projection);
            back.reverse();
            backwards.extend(back);
            operation_meta.push(OperationMeta {
                operation_id: operation
                    .operation_id()
                    .unwrap_or_else(|| create_document_vcs_id("operation")),
                dependencies: operation.dependencies(),
                base_version: operation.base_version(),
                author_id: operation.author_id().unwrap_or_else(|| "local".into()),
                timestamp: operation
                    .timestamp()
                    .unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: operation.undo_policy(),
                payload_hash: Some(semio_framework_hash::hash_bytes(
                    &serde_json::to_vec(&operation).unwrap_or_default(),
                )),
            });
            projection = apply_operation(&projection, &operation);
            forwards.push(operation);
        }
        (forwards, backwards, operation_meta, projection)
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        let command: DocumentVcsCommand<Op> =
            serde_json::from_str(command_json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
        self.dispatch(command)
    }

    pub fn envelope_json(&self) -> Result<String, VcsError> {
        serde_json::to_string(&self.envelope).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    pub fn projection_json(&self) -> Result<String, VcsError> {
        let projection = self.projection()?;
        serde_json::to_string(&projection).map_err(|e| VcsError::Serialize(e.to_string()))
    }

    /// @emoji 🔗 Attaches a backbone channel, reconciling any already-persisted state before
    /// seeding it with this store's current snapshot.
    pub fn attach_backbone(&mut self, backbone: Box<dyn Backbone>) -> Result<(), VcsError> {
        self.envelope.backbone = Some(backbone.descriptor());
        self.backbone = Some(backbone);
        self.pump()?;
        self.flush_outbound(false)?;
        self.bump();
        Ok(())
    }

    /// @emoji 🔗 Resolves a backbone URI and attaches it. Only available inside the wasm sandbox,
    /// where every scheme forwards to the host over the injected {@link BackboneChannelPort} (a pure
    /// queue). On native targets, callers attach an explicit `Box<dyn Backbone>` via
    /// {@link attach_backbone} — the `framework/sync` actor layer owns all IO-performing endpoints.
    #[cfg(target_arch = "wasm32")]
    pub fn attach_backbone_uri(&mut self, uri: &str) -> Result<(), VcsError> {
        self.attach_backbone(resolve_backbone(uri)?)
    }

    /// @emoji ✂️ Detaches the backbone; the WIP graph stays in memory, simply unsynchronized.
    pub fn detach_backbone(&mut self) -> Option<Box<dyn Backbone>> {
        self.envelope.backbone = None;
        self.bump();
        self.backbone.take()
    }

    pub fn backbone_ref(&self) -> Option<&DocumentBackboneRef> {
        self.envelope.backbone.as_ref()
    }

    /// @emoji 📡 Drains inbound backbone messages into the edit timeline. Safe to call anytime;
    /// `dispatch` already calls this before every command.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.pump()
    }

    /// @emoji 🕸️ Feeds a remote {@link OpEnvelope} through the causal DAG, applying it (and any
    /// now-unblocked dependents) into the edit timeline. Closes the sync gap between
    /// `framework/sync`'s `OpDag` and the vcs edit history.
    pub fn ingest_remote(&mut self, envelope: OpEnvelope) -> Result<(), VcsError> {
        self.dag
            .insert(envelope)
            .map_err(|error| VcsError::Backbone(error.to_string()))?;
        for envelope in self.dag.drain_applied_envelopes() {
            self.ingest_envelope(envelope)?;
        }
        Ok(())
    }

    fn ingest_envelope(&mut self, envelope: OpEnvelope) -> Result<(), VcsError> {
        let mut edit: Edit<Op> = edit_from_op_envelope(&envelope)?;
        edit.actor = Some(envelope.actor.0.clone());
        if self.envelope.vcs.edits.iter().any(|existing| existing.id == edit.id) {
            return Ok(());
        }
        self.edit_sequence = self.edit_sequence.max(edit.sequence_number);
        let edit_id = edit.id.clone();
        self.envelope.vcs.edits.push(edit);
        self.applied_edit_ids.push(edit_id);
        self.bump();
        Ok(())
    }

    fn merge_remote_snapshot(&mut self, envelope_json: &str) -> Result<(), VcsError> {
        let remote: DocumentVcsEnvelope<P, Op> =
            serde_json::from_str(envelope_json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
        if self.envelope.vcs.edits.is_empty() {
            let applied: Vec<String> = remote.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
            self.edit_sequence = remote
                .vcs
                .edits
                .iter()
                .map(|edit| edit.sequence_number)
                .max()
                .unwrap_or(0);
            let backbone_ref = self.envelope.backbone.clone();
            self.envelope = remote;
            self.envelope.backbone = backbone_ref;
            self.applied_edit_ids = applied;
            self.redo_edit_ids.clear();
            self.bump();
            return Ok(());
        }
        let existing_edit_ids: HashSet<String> = self.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
        for edit in remote.vcs.edits {
            if existing_edit_ids.contains(&edit.id) {
                continue;
            }
            self.edit_sequence = self.edit_sequence.max(edit.sequence_number);
            self.applied_edit_ids.push(edit.id.clone());
            self.envelope.vcs.edits.push(edit);
        }
        merge_by_id(&mut self.envelope.vcs.changes, remote.vcs.changes, |change| &change.id);
        merge_by_id(&mut self.envelope.vcs.checkpoints, remote.vcs.checkpoints, |checkpoint| &checkpoint.id);
        merge_by_id(&mut self.envelope.vcs.alternatives, remote.vcs.alternatives, |alternative| &alternative.id);
        self.bump();
        Ok(())
    }

    fn previous_edit_dependency(&self) -> Vec<OperationId> {
        let len = self.applied_edit_ids.len();
        if len >= 2 {
            vec![OperationId(self.applied_edit_ids[len - 2].clone())]
        } else {
            Vec::new()
        }
    }

    /// @emoji 📥 Pumps every queued inbound message from the attached backbone into the timeline.
    fn pump(&mut self) -> Result<bool, VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(false);
        };
        let received = backbone.receive();
        self.backbone = Some(backbone);
        let messages = received?;
        if messages.is_empty() {
            return Ok(false);
        }
        let mut acked_op_ids: Vec<String> = Vec::new();
        for message in messages {
            match message {
                BackboneMessage::Snapshot { envelope_json } => self.merge_remote_snapshot(&envelope_json)?,
                BackboneMessage::Ops { envelopes } => {
                    let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.id.0.clone()).collect();
                    for envelope in envelopes {
                        self.ingest_remote(envelope)?;
                    }
                    acked_op_ids.extend(op_ids);
                }
                // A store never consumes acks (they flow store→actor); drain and ignore any that echo back.
                BackboneMessage::Ack { .. } => {}
            }
        }
        if !acked_op_ids.is_empty() {
            if let Some(mut backbone) = self.backbone.take() {
                let result = backbone.send(BackboneMessage::Ack { op_ids: acked_op_ids });
                self.backbone = Some(backbone);
                result?;
            }
        }
        Ok(true)
    }

    /// @emoji 📤 Sends the just-applied change outward: a single {@link OpEnvelope} for `Apply`,
    /// or a full snapshot for every structural command (undo/redo/checkpoint/alternative/amend).
    fn flush_outbound(&mut self, is_apply: bool) -> Result<(), VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(());
        };
        let result = if is_apply {
            match self.envelope.vcs.edits.last() {
                Some(edit) => {
                    let deps = self.previous_edit_dependency();
                    match op_envelope_from_edit(&self.envelope, edit, deps) {
                        Ok(op_envelope) => {
                            // Registers this locally-authored edit as already-applied in our own
                            // DAG, so a later remote envelope that depends on it doesn't stall as pending.
                            let _ = self.dag.insert(op_envelope.clone());
                            backbone.send(BackboneMessage::Ops { envelopes: vec![op_envelope] })
                        }
                        Err(error) => Err(error),
                    }
                }
                None => Ok(()),
            }
        } else {
            self.envelope_json()
                .and_then(|json| backbone.send(BackboneMessage::Snapshot { envelope_json: json }))
        };
        self.backbone = Some(backbone);
        result
    }

    /// @emoji 🖋️ Whether `edit_id` was authored by the local actor. Unauthored (legacy) edits count
    /// as local; every other actor is foreign and must not be undone by this store.
    fn edit_is_local(&self, edit_id: &str) -> bool {
        self.envelope
            .vcs
            .edits
            .iter()
            .find(|edit| edit.id == edit_id)
            .map(|edit| edit.actor.is_none() || edit.actor.as_deref() == self.local_actor_id.as_deref())
            .unwrap_or(false)
    }

    fn bump(&mut self) {
        self.generation += 1;
    }
}

fn merge_by_id<T: Clone>(local: &mut Vec<T>, remote: Vec<T>, id_of: impl Fn(&T) -> &String) {
    let mut existing: HashSet<String> = local.iter().map(|item| id_of(item).clone()).collect();
    for item in remote {
        if existing.insert(id_of(&item).clone()) {
            local.push(item);
        }
    }
}

/// @emoji 📦 Serializes an `Edit` into the causal wire envelope exchanged over a backbone channel.
pub fn op_envelope_from_edit<P, Op>(
    envelope: &DocumentVcsEnvelope<P, Op>,
    edit: &Edit<Op>,
    deps: Vec<OperationId>,
) -> Result<OpEnvelope, VcsError>
where
    Op: Serialize,
{
    let payload = serde_json::to_value(edit).map_err(|e| VcsError::Serialize(e.to_string()))?;
    let payload_hash = semio_framework_hash::hash_bytes(&serde_json::to_vec(edit).unwrap_or_default());
    let author_id = edit
        .operation_meta
        .last()
        .map(|meta| meta.author_id.clone())
        .unwrap_or_else(|| "local".into());
    let undo_policy = edit
        .operation_meta
        .last()
        .map(|meta| meta.undo_policy)
        .unwrap_or(UndoPolicy::ExactBaseOnly);
    Ok(OpEnvelope {
        id: OperationId(edit.id.clone()),
        actor: ActorId(author_id),
        document: DocumentId(envelope.id.clone()),
        schema_version: SchemaVersion(envelope.schema.clone()),
        deps: deps.clone(),
        payload_hash: PayloadHash(payload_hash),
        diff: DocumentDiff {
            schema_id: SchemaId(envelope.schema.clone()),
            payload,
        },
        inverse: InverseOperation {
            target_operation: OperationId(edit.id.clone()),
            inverse_diff: DocumentDiff {
                schema_id: SchemaId(envelope.schema.clone()),
                payload: serde_json::json!({ "backwards": edit.backwards }),
            },
            base_version: DocumentVersion(edit.sequence_number as u64),
            dependencies: deps,
            undo_policy,
        },
    })
}

/// @emoji 📦 Recovers an `Edit` from the causal wire envelope produced by `op_envelope_from_edit`.
pub fn edit_from_op_envelope<Op>(envelope: &OpEnvelope) -> Result<Edit<Op>, VcsError>
where
    Op: DeserializeOwned,
{
    serde_json::from_value(envelope.diff.payload.clone()).map_err(|e| VcsError::Deserialize(e.to_string()))
}
//#endregion 🔖DocumentVcsStore

//#region 🔖Backbone
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioConflict {
    pub kind: String,
    pub uri: String,
    pub message: String,
}

/// @emoji 📨 Wire message exchanged over an attached backbone channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum BackboneMessage {
    Snapshot { envelope_json: String },
    Ops { envelopes: Vec<OpEnvelope> },
    /// @emoji ✅ Acknowledges inbound ops the store has ingested (store→actor). Lets a future actor
    /// implement at-least-once redelivery with id-based dedupe — safe across store crashes/reloads.
    Ack { op_ids: Vec<String> },
}

/// @emoji 🧵 Non-blocking, IO-free in-memory queue contract between a `DocumentVcsStore` and its
/// sync actor. `send`/`receive` MUST return immediately: implementations only enqueue/dequeue
/// `BackboneMessage`s — never HTTP, never filesystem, never a blocking wait. All IO (persistence,
/// hub sync, file watching, presence) lives behind this queue in `framework/sync`'s actor layer,
/// which owns the other end; the store's `pump()`/`flush_outbound()` run synchronously on the
/// caller's thread and must never be blocked by transport work.
pub trait Backbone: Send + Sync {
    fn descriptor(&self) -> DocumentBackboneRef;
    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError>;
    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError>;
}

pub trait BackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

static HOST_BACKBONE_PORT: Mutex<Option<Arc<dyn BackbonePort>>> = Mutex::new(None);

/// @emoji 🔌 Injects the browser or dev-server backbone port for wasm file/folder IO.
pub fn set_host_backbone_port(port: Arc<dyn BackbonePort>) {
    if let Ok(mut guard) = HOST_BACKBONE_PORT.lock() {
        *guard = Some(port);
    }
}

fn host_backbone_port() -> Option<Arc<dyn BackbonePort>> {
    HOST_BACKBONE_PORT.lock().ok().and_then(|guard| guard.clone())
}

#[derive(Default)]
pub struct MemoryBackbonePort {
    files: Mutex<HashMap<String, String>>,
}

impl MemoryBackbonePort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BackbonePort for MemoryBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .get(uri)
            .cloned()
            .ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.files
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .insert(uri.to_string(), payload.to_string());
        Ok(())
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
fn local_storage_backbone_key(uri: &str) -> String {
    format!("semio:vcs:{uri}")
}

/// @emoji 💾 Browser `localStorage` backbone port with in-memory fallback for native tests.
pub struct LocalStorageBackbonePort {
    fallback: MemoryBackbonePort,
}

impl LocalStorageBackbonePort {
    pub fn new() -> Self {
        Self {
            fallback: MemoryBackbonePort::new(),
        }
    }
}

impl Default for LocalStorageBackbonePort {
    fn default() -> Self {
        Self::new()
    }
}

impl BackbonePort for LocalStorageBackbonePort {
    fn read(&self, uri: &str) -> Result<String, VcsError> {
        if let Some(port) = host_backbone_port() {
            if let Ok(value) = port.read(uri) {
                return Ok(value);
            }
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item(&local_storage_backbone_key(uri)) {
                        return Ok(value);
                    }
                }
            }
        }
        self.fallback.read(uri)
    }

    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError> {
        self.fallback.write(uri, payload)?;
        if let Some(port) = host_backbone_port() {
            let _ = port.write(uri, payload);
        }
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(&local_storage_backbone_key(uri), payload);
                }
            }
        }
        Ok(())
    }
}

/// @emoji 🕸️ Injectable duplex transport across the wasm sandbox boundary (plugin ↔ host process).
pub trait BackboneChannelPort: Send + Sync {
    fn send(&self, uri: &str, message_json: &str) -> Result<(), VcsError>;
    fn poll(&self, uri: &str) -> Result<Vec<String>, VcsError>;
}

static HOST_BACKBONE_CHANNEL: Mutex<Option<Arc<dyn BackboneChannelPort>>> = Mutex::new(None);

/// @emoji 🔌 Injects the plugin host's duplex backbone channel for wasm-sandboxed document stores.
pub fn set_host_backbone_channel(channel: Arc<dyn BackboneChannelPort>) {
    if let Ok(mut guard) = HOST_BACKBONE_CHANNEL.lock() {
        *guard = Some(channel);
    }
}

fn host_backbone_channel() -> Option<Arc<dyn BackboneChannelPort>> {
    HOST_BACKBONE_CHANNEL.lock().ok().and_then(|guard| guard.clone())
}

/// @emoji 🧵 Backbone that forwards messages across the wasm sandbox boundary to the host process,
/// which resolves the real `file://`/`folder://`/`remote://` backbone on its own (native) side.
pub struct PortBackbone {
    uri: String,
}

impl PortBackbone {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.to_string() }
    }
}

impl Backbone for PortBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        let channel = host_backbone_channel()
            .ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        let json = serde_json::to_string(&message).map_err(|e| VcsError::Serialize(e.to_string()))?;
        channel.send(&self.uri, &json)
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let channel = host_backbone_channel()
            .ok_or_else(|| VcsError::Backbone("backbone channel requires host port".into()))?;
        channel
            .poll(&self.uri)?
            .into_iter()
            .map(|json| serde_json::from_str(&json).map_err(|e| VcsError::Deserialize(e.to_string())))
            .collect()
    }
}

/// @emoji 🔗 Two crossed in-memory channel ends: whatever `a` sends, `b` receives, and vice versa.
pub struct MemoryBackbone {
    uri: String,
    inbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbox: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl MemoryBackbone {
    pub fn pair(uri_a: &str, uri_b: &str) -> (Self, Self) {
        let a_to_b = Arc::new(Mutex::new(VecDeque::new()));
        let b_to_a = Arc::new(Mutex::new(VecDeque::new()));
        (
            Self {
                uri: uri_a.to_string(),
                inbox: b_to_a.clone(),
                outbox: a_to_b.clone(),
            },
            Self {
                uri: uri_b.to_string(),
                inbox: a_to_b,
                outbox: b_to_a,
            },
        )
    }
}

impl Backbone for MemoryBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbox
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbox = self.inbox.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbox.drain(..).collect())
    }
}

/// @emoji 🔗 The store-side end of a pair of crossed in-memory queues. Implements the non-blocking
/// {@link Backbone} contract; the matching {@link ChannelBackboneRemote} is held by an external sync
/// actor (built in `framework/sync`, a later workstream) that pushes inbound messages and drains the
/// store's outbound ones. This crate only provides the queue plumbing — never the actor itself.
pub struct ChannelBackbone {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

/// @emoji 🎛️ The actor-side end paired with a {@link ChannelBackbone}: `push` delivers a message to
/// the store's inbound queue, `drain` collects everything the store has sent outbound. Not a
/// `Backbone` — this is the handle an IO-owning actor endpoint holds across the store boundary.
pub struct ChannelBackboneRemote {
    uri: String,
    inbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
    outbound: Arc<Mutex<VecDeque<BackboneMessage>>>,
}

impl ChannelBackbone {
    /// @emoji 🔗 Creates a crossed pair sharing a URI: the store attaches the `ChannelBackbone`; the
    /// actor keeps the `ChannelBackboneRemote`.
    pub fn pair(uri: &str) -> (ChannelBackbone, ChannelBackboneRemote) {
        let inbound = Arc::new(Mutex::new(VecDeque::new()));
        let outbound = Arc::new(Mutex::new(VecDeque::new()));
        (
            ChannelBackbone {
                uri: uri.to_string(),
                inbound: inbound.clone(),
                outbound: outbound.clone(),
            },
            ChannelBackboneRemote {
                uri: uri.to_string(),
                inbound,
                outbound,
            },
        )
    }
}

impl Backbone for ChannelBackbone {
    fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    fn send(&mut self, message: BackboneMessage) -> Result<(), VcsError> {
        self.outbound
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut inbound = self.inbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(inbound.drain(..).collect())
    }
}

impl ChannelBackboneRemote {
    pub fn descriptor(&self) -> DocumentBackboneRef {
        document_backbone_ref(&self.uri)
    }

    /// @emoji 📥 Delivers a message to the store's inbound queue (actor→store).
    pub fn push(&self, message: BackboneMessage) -> Result<(), VcsError> {
        self.inbound
            .lock()
            .map_err(|_| VcsError::Backbone("lock poisoned".into()))?
            .push_back(message);
        Ok(())
    }

    /// @emoji 📤 Collects everything the store has sent outbound (store→actor), draining the queue.
    pub fn drain(&self) -> Result<Vec<BackboneMessage>, VcsError> {
        let mut outbound = self.outbound.lock().map_err(|_| VcsError::Backbone("lock poisoned".into()))?;
        Ok(outbound.drain(..).collect())
    }
}

/// @emoji 🗃️ Pure single-blob file persistence (`file://x.json`) — read/write a whole envelope JSON.
/// No `Backbone` impl: the `framework/sync` actor layer drives this from its own thread; this crate
/// only owns the file format. `file://` is an export/import format; `folder://` is the canonical
/// local store.
#[cfg(not(target_arch = "wasm32"))]
pub struct FileJsonStorage {
    path: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileJsonStorage {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }

    /// @emoji 📖 Reads the stored envelope JSON, or `None` if the file does not exist yet.
    pub fn read(&self) -> Result<Option<String>, VcsError> {
        match std::fs::read_to_string(&self.path) {
            Ok(json) => Ok(Some(json)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(VcsError::Backbone(err.to_string())),
        }
    }

    /// @emoji ✍️ Writes the whole envelope JSON, creating parent directories as needed.
    pub fn write(&self, envelope_json: &str) -> Result<(), VcsError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| VcsError::Backbone(e.to_string()))?;
        }
        std::fs::write(&self.path, envelope_json).map_err(|e| VcsError::Backbone(e.to_string()))
    }
}

/// @emoji 🗄️ Pure multi-document sqlite persistence (`folder://`), the canonical local store. Rows
/// are keyed by document id: `document(id, schema, json, updated_at)` — a single folder holds every
/// open document's envelope. No `Backbone` impl: the `framework/sync` actor layer drives this from
/// its own thread; this crate only owns the sqlite schema.
#[cfg(not(target_arch = "wasm32"))]
pub struct FolderSqliteStorage {
    folder: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderSqliteStorage {
    pub fn new(folder: std::path::PathBuf) -> Self {
        Self { folder }
    }

    fn db_path(&self) -> std::path::PathBuf {
        self.folder.join(".semio").join("documents.db")
    }

    fn connection(&self) -> Result<rusqlite::Connection, VcsError> {
        let semio_dir = self.folder.join(".semio");
        std::fs::create_dir_all(&semio_dir).map_err(|e| VcsError::Backbone(e.to_string()))?;
        let conn = rusqlite::Connection::open(self.db_path()).map_err(|e| VcsError::Backbone(e.to_string()))?;
        Self::ensure_schema(&conn)?;
        Ok(conn)
    }

    fn ensure_schema(conn: &rusqlite::Connection) -> Result<(), VcsError> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS document (\
                 id TEXT PRIMARY KEY,\
                 schema TEXT,\
                 json TEXT NOT NULL,\
                 updated_at INTEGER NOT NULL\
             );",
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))
    }

    /// @emoji 📖 Reads the stored envelope JSON for `document_id`, or `None` if absent.
    pub fn read(&self, document_id: &str) -> Result<Option<String>, VcsError> {
        use rusqlite::OptionalExtension;
        let conn = self.connection()?;
        conn.query_row("SELECT json FROM document WHERE id = ?1", [document_id], |row| row.get(0))
            .optional()
            .map_err(|e| VcsError::Backbone(e.to_string()))
    }

    /// @emoji ✍️ Upserts `document_id`'s envelope JSON (with its schema id and an `updated_at` stamp).
    pub fn write(&self, document_id: &str, schema: &str, envelope_json: &str) -> Result<(), VcsError> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO document (id, schema, json, updated_at) VALUES (?1, ?2, ?3, ?4) \
             ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, json = excluded.json, updated_at = excluded.updated_at",
            rusqlite::params![document_id, schema, envelope_json, now_ms() as i64],
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(())
    }

    /// @emoji 📇 Lists every stored document id (newest write first), for a folder-wide index.
    pub fn document_ids(&self) -> Result<Vec<String>, VcsError> {
        let conn = self.connection()?;
        let mut statement = conn
            .prepare("SELECT id FROM document ORDER BY updated_at DESC")
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        let ids = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| VcsError::Backbone(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(ids)
    }
}

/// @emoji 🔌 Resolves a backbone URI to a concrete channel implementation. Only available inside the
/// wasm sandbox, where every scheme forwards to the host process over the injected
/// {@link BackboneChannelPort} (a pure in-memory queue). Native IO-performing backbones moved out of
/// this crate entirely — the `framework/sync` actor layer owns them.
#[cfg(target_arch = "wasm32")]
pub fn resolve_backbone(uri: &str) -> Result<Box<dyn Backbone>, VcsError> {
    Ok(Box::new(PortBackbone::new(uri)))
}
//#endregion 🔖Backbone

//#region 🔖TestSupport
/// @emoji 🧪 Round-trip assertions shared by every technology crate's `Operation` test suite.
pub mod test_support {
    use super::*;

    /// @emoji 🔁 Asserts that applying `op` then applying its reversed `backwards(pre)` restores `pre`.
    pub fn assert_operation_round_trip<P, Op>(pre: &P, op: Op)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Op: Operation<P>,
    {
        let post = apply_operation(pre, &op);
        let mut backwards = op.backwards(pre);
        backwards.reverse();
        let restored = backwards
            .iter()
            .fold(post, |projection, back_op| apply_operation(&projection, back_op));
        assert_eq!(&restored, pre, "operation backwards did not restore pre-state");
    }

    /// @emoji 🗄️ Asserts a full store round trip: Apply→Undo restores `initial`, Redo restores the
    /// post-apply projection, and replay-materialization agrees with the live store projection.
    pub fn assert_store_roundtrip<P, Op>(initial: P, op: Op)
    where
        P: Clone + Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
        Op: Clone + Serialize + DeserializeOwned + Operation<P>,
    {
        let envelope = create_document_vcs_envelope("test/v1", "test", initial.clone(), None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![op],
                description: None,
            })
            .expect("apply");
        let post = store.projection().expect("post projection");
        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(
            store.projection().expect("undo projection"),
            initial,
            "undo did not restore initial projection"
        );
        store.dispatch(DocumentVcsCommand::Redo).expect("redo");
        assert_eq!(
            store.projection().expect("redo projection"),
            post,
            "redo did not restore post projection"
        );
        let replayed = materialize_document_projection(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(replayed, post, "materialization from replay diverged from store projection");
    }
}
//#endregion 🔖TestSupport

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoProjection {
        n: i32,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoDiff {
        n: Option<i32>,
    }

    impl OperationDiff<DemoProjection> for DemoDiff {
        fn apply(&self, projection: &DemoProjection) -> DemoProjection {
            DemoProjection {
                n: self.n.unwrap_or(projection.n),
            }
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "op")]
    enum DemoOp {
        SetN { n: i32 },
    }

    impl Operation<DemoProjection> for DemoOp {
        type Diff = DemoDiff;

        fn diff(&self, _projection: &DemoProjection) -> DemoDiff {
            match self {
                DemoOp::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }

        fn backwards(&self, projection: &DemoProjection) -> Vec<Self> {
            vec![DemoOp::SetN { n: projection.n }]
        }
    }

    /// @emoji 🛰️ Builds a foreign {@link OpEnvelope} (as if authored by `actor` on another peer) by
    /// applying `op` in a throwaway peer store and stamping the envelope's actor id.
    fn foreign_op_envelope(actor: &str, op: DemoOp) -> OpEnvelope {
        let mut peer = DocumentVcsStore::new(create_document_vcs_envelope::<DemoProjection, DemoOp>(
            "demo/v1",
            "demo",
            DemoProjection { n: 0 },
            None,
        ));
        peer.dispatch(DocumentVcsCommand::Apply {
            operations: vec![op],
            description: None,
        })
        .expect("peer apply");
        let edit = peer.envelope().vcs.edits.last().expect("peer edit").clone();
        let mut envelope = op_envelope_from_edit(peer.envelope(), &edit, Vec::new()).expect("op envelope");
        envelope.actor = ActorId(actor.to_string());
        envelope
    }

    #[test]
    fn materialize_replays_forward_ops() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").n, 1);
        assert_eq!(store.envelope().vcs.edits.len(), 1);
    }

    #[test]
    fn undo_redo_round_trip() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").n, 0);
        store.dispatch(DocumentVcsCommand::Redo).expect("redo");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn apply_computes_backwards_from_pre_state() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let edit = &store.envelope().vcs.edits[0];
        assert_eq!(edit.backwards, vec![DemoOp::SetN { n: 0 }]);
    }

    #[test]
    fn commit_checkpoint_wraps_edits_into_change() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("init".into()),
                authors: vec![Author {
                    id: "a1".into(),
                    name: "Alice".into(),
                    avatar: None,
                }],
            })
            .expect("commit");
        assert_eq!(store.envelope().vcs.changes.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints[0].message, Some("init".into()));
    }

    #[test]
    fn checkout_checkpoint_restores_applied_edits() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit");
        let checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 9 }],
                description: None,
            })
            .expect("apply2");
        assert_eq!(store.projection().expect("projection").n, 9);
        store
            .dispatch(DocumentVcsCommand::CheckoutCheckpoint {
                checkpoint_id,
            })
            .expect("checkout");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn alternatives_switch_restores_checkpoint_chain() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CreateAlternative {
                name: "branch-a".into(),
            })
            .expect("create alternative");
        let alt_id = store.envelope().vcs.alternatives[0].id.clone();
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 2 }],
                description: None,
            })
            .expect("apply on branch");
        store
            .dispatch(DocumentVcsCommand::SwitchAlternative {
                alternative_id: alt_id,
            })
            .expect("switch");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn checkout_old_checkpoint_then_commit_creates_a_fork() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit c1");
        let c1 = store.envelope().vcs.checkpoints[0].id.clone();
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("c2".into()),
                authors: Vec::new(),
            })
            .expect("commit c2");
        store
            .dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: c1.clone() })
            .expect("checkout c1");
        assert_eq!(store.current_checkpoint_id(), Some(c1.as_str()));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 9 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("fork".into()),
                authors: Vec::new(),
            })
            .expect("commit fork");
        let children: Vec<&Checkpoint> = store
            .envelope()
            .vcs
            .checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(c1.as_str()))
            .collect();
        assert_eq!(children.len(), 2, "checking out an old checkpoint before committing must fork, not extend the trunk");
    }

    #[test]
    fn create_alternative_appends_commits_to_its_own_checkpoint_chain() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("root".into()),
                authors: Vec::new(),
            })
            .expect("commit root");
        store
            .dispatch(DocumentVcsCommand::CreateAlternative { name: "feature-a".into() })
            .expect("create alternative");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("branch commit".into()),
                authors: Vec::new(),
            })
            .expect("commit on branch");
        assert_eq!(store.envelope().vcs.alternatives[0].checkpoint_ids.len(), 2);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2);
    }

    #[test]
    fn history_columns_orders_newest_first_and_labels_trunk_root() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit c1");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("c2".into()),
                authors: Vec::new(),
            })
            .expect("commit c2");
        let columns = store.history_columns();
        assert_eq!(columns.len(), 2);
        assert_eq!(columns[0].description, Some("c2".into()), "newest checkpoint must be first");
        assert_eq!(columns[0].lane, 0);
        assert_eq!(columns[0].labels, vec!["main".to_string()], "newest unlabeled row falls back to main");
        assert!(columns[1].labels.is_empty(), "only the newest row gets the main fallback");
        let json = serde_json::to_string(&columns[0]).expect("serialize");
        assert!(json.contains("checkpointId"), "wire format must be camelCase: {json}");
    }

    #[test]
    fn history_columns_assigns_distinct_lanes_and_pulls_main_only_descendants_to_trunk() {
        let envelope = create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("root".into()),
                authors: Vec::new(),
            })
            .expect("commit root");
        let root = store.envelope().vcs.checkpoints[0].id.clone();

        store
            .dispatch(DocumentVcsCommand::CreateAlternative { name: "feature-a".into() })
            .expect("create feature-a");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("a1".into()),
                authors: Vec::new(),
            })
            .expect("commit a1");

        store
            .dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: root.clone() })
            .expect("checkout root");
        store
            .dispatch(DocumentVcsCommand::CreateAlternative { name: "feature-b".into() })
            .expect("create feature-b");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 3 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("b1".into()),
                authors: Vec::new(),
            })
            .expect("commit b1");

        store
            .dispatch(DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id: root.clone() })
            .expect("checkout root again");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 4 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("main resumed".into()),
                authors: Vec::new(),
            })
            .expect("commit main resumed");

        let columns = store.history_columns();
        assert_eq!(columns.len(), 4, "root + a1 + b1 + main-resumed");
        let by_message: HashMap<String, &HistoryColumn> = columns
            .iter()
            .filter_map(|column| column.description.clone().map(|description| (description, column)))
            .collect();
        assert_eq!(by_message["root"].lane, 0, "root has no parent, lane 0");
        assert_eq!(by_message["main resumed"].lane, 0, "commit with no alternative stays on the trunk");
        let a_lane = by_message["a1"].lane;
        let b_lane = by_message["b1"].lane;
        assert_ne!(a_lane, 0, "a1 belongs to an alternative, not the trunk");
        assert_ne!(b_lane, 0, "b1 belongs to an alternative, not the trunk");
        assert_ne!(a_lane, b_lane, "distinct alternatives must get distinct swimlanes");

        let root_children: Vec<&HistoryColumn> = columns
            .iter()
            .filter(|column| column.parent_checkpoint_id.as_deref() == Some(root.as_str()))
            .collect();
        assert_eq!(root_children.len(), 3, "root forked three ways: a1, b1, main-resumed");
    }

    #[test]
    fn no_backbone_by_default() {
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        assert!(envelope.backbone.is_none(), "a fresh document has no attached backbone");
        let store = DocumentVcsStore::new(envelope);
        assert!(store.backbone_ref().is_none());
    }

    #[test]
    fn memory_backbone_pair_propagates_edits_bidirectionally() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope_a: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let envelope_b: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store_a = DocumentVcsStore::new(envelope_a);
        let mut store_b = DocumentVcsStore::new(envelope_b);
        store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        store_a
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("apply on a");
        store_b.tick().expect("tick b");
        assert_eq!(store_b.projection().expect("projection b").n, 1, "b receives a's edit");

        store_b
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 2 }],
                description: None,
            })
            .expect("apply on b");
        store_a.tick().expect("tick a");
        assert_eq!(store_a.projection().expect("projection a").n, 2, "a receives b's edit");
    }

    #[test]
    fn detach_backbone_stops_synchronizing_but_keeps_the_wip_graph() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store_a = DocumentVcsStore::new(envelope.clone());
        let mut store_b = DocumentVcsStore::new(envelope);
        store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");
        store_a.detach_backbone();
        assert!(store_a.backbone_ref().is_none());

        store_a
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 9 }],
                description: None,
            })
            .expect("apply after detach still works on the in-memory graph");
        assert_eq!(store_a.projection().expect("projection a").n, 9);
        store_b.tick().expect("tick b");
        assert_eq!(store_b.projection().expect("projection b").n, 0, "detached edits never reach the peer");
    }

    #[test]
    fn file_json_storage_round_trips_a_blob() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FileJsonStorage::new(dir.path().join("demo.json"));
        assert_eq!(storage.read().expect("read empty"), None, "absent file reads as None");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 1 }, None);
        storage
            .write(&serde_json::to_string(&envelope).expect("json"))
            .expect("write");
        let loaded: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&storage.read().expect("read").expect("some")).expect("parse");
        assert_eq!(loaded.id, "demo");
    }

    #[test]
    fn folder_sqlite_storage_round_trips_by_document_id() {
        let dir = tempfile::tempdir().expect("tempdir");
        let storage = FolderSqliteStorage::new(dir.path().to_path_buf());
        assert_eq!(storage.read("doc-a").expect("read empty"), None, "absent document reads as None");
        let env_a: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "doc-a", DemoProjection { n: 3 }, None);
        let env_b: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "doc-b", DemoProjection { n: 7 }, None);
        storage
            .write("doc-a", "demo/v1", &serde_json::to_string(&env_a).expect("json a"))
            .expect("write a");
        storage
            .write("doc-b", "demo/v1", &serde_json::to_string(&env_b).expect("json b"))
            .expect("write b");
        let loaded_a: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&storage.read("doc-a").expect("read a").expect("some a")).expect("parse a");
        let loaded_b: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&storage.read("doc-b").expect("read b").expect("some b")).expect("parse b");
        assert_eq!(loaded_a.vcs.initial_projection.n, 3, "documents are keyed independently");
        assert_eq!(loaded_b.vcs.initial_projection.n, 7);

        let env_a2: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "doc-a", DemoProjection { n: 5 }, None);
        storage
            .write("doc-a", "demo/v1", &serde_json::to_string(&env_a2).expect("json a2"))
            .expect("upsert a");
        let reloaded_a: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&storage.read("doc-a").expect("reread a").expect("some a2")).expect("parse a2");
        assert_eq!(reloaded_a.vcs.initial_projection.n, 5, "writing the same id upserts in place");

        let mut ids = storage.document_ids().expect("document ids");
        ids.sort();
        assert_eq!(ids, vec!["doc-a".to_string(), "doc-b".to_string()], "folder indexes every document");
    }

    #[test]
    fn attach_reconciles_a_pushed_snapshot() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let seeded: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut seed_store = DocumentVcsStore::new(seeded);
        seed_store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        remote
            .push(BackboneMessage::Snapshot {
                envelope_json: seed_store.envelope_json().expect("seed json"),
            })
            .expect("push snapshot");

        let fresh: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(fresh);
        store.attach_backbone(Box::new(channel)).expect("attach reconciles the pushed snapshot");
        assert_eq!(store.projection().expect("projection").n, 5, "adopted the pushed snapshot's edit");
    }

    #[test]
    fn channel_backbone_round_trips_between_store_and_actor() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store.attach_backbone(Box::new(channel)).expect("attach");
        let attach_flush = remote.drain().expect("drain attach");
        assert!(
            attach_flush.iter().any(|message| matches!(message, BackboneMessage::Snapshot { .. })),
            "attach flushes a snapshot to the actor end: {attach_flush:?}"
        );

        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 4 }],
                description: None,
            })
            .expect("apply");
        let outbound = remote.drain().expect("drain apply");
        assert!(
            outbound.iter().any(|message| matches!(message, BackboneMessage::Ops { .. })),
            "a local apply is sent outbound as ops: {outbound:?}"
        );

        remote
            .push(BackboneMessage::Ops {
                envelopes: vec![foreign_op_envelope("peer", DemoOp::SetN { n: 8 })],
            })
            .expect("push inbound ops");
        store.tick().expect("tick");
        assert_eq!(store.projection().expect("projection").n, 8, "store ingests the actor's inbound ops");
    }

    #[test]
    fn pump_acks_ingested_ops() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store.attach_backbone(Box::new(channel)).expect("attach");
        let _ = remote.drain().expect("drain attach snapshot");

        let inbound = foreign_op_envelope("peer", DemoOp::SetN { n: 7 });
        let op_id = inbound.id.0.clone();
        remote
            .push(BackboneMessage::Ops { envelopes: vec![inbound] })
            .expect("push inbound ops");
        store.tick().expect("tick");
        assert_eq!(store.projection().expect("projection").n, 7, "ingested the inbound op");

        let outbound = remote.drain().expect("drain ack");
        assert!(
            outbound
                .iter()
                .any(|message| matches!(message, BackboneMessage::Ack { op_ids } if op_ids == &vec![op_id.clone()])),
            "successful ops ingest emits an Ack for the ingested op ids: {outbound:?}"
        );
    }

    #[test]
    fn exact_base_only_undo_refuses_a_foreign_tail() {
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("local apply");
        store
            .ingest_remote(foreign_op_envelope("peer", DemoOp::SetN { n: 2 }))
            .expect("ingest foreign");
        assert_eq!(store.projection().expect("projection").n, 2, "foreign edit sits at the tail");

        let error = store
            .dispatch(DocumentVcsCommand::UndoWithPolicy {
                policy: UndoPolicy::ExactBaseOnly,
                semantic_command: None,
            })
            .expect_err("undo must refuse a foreign tail");
        assert!(matches!(error, VcsError::ForeignEdit(_)), "got {error:?}");
        assert_eq!(store.projection().expect("projection").n, 2, "the timeline is untouched after refusal");
    }

    #[test]
    fn transform_against_concurrent_undo_skips_over_a_foreign_tail() {
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 1 }],
                description: None,
            })
            .expect("local apply");
        let local_edit_id = store.applied_edit_ids()[0].clone();
        let foreign = foreign_op_envelope("peer", DemoOp::SetN { n: 2 });
        let foreign_id = foreign.id.0.clone();
        store.ingest_remote(foreign).expect("ingest foreign");
        assert_eq!(store.applied_edit_ids().len(), 2, "local + foreign are both applied");

        store
            .dispatch(DocumentVcsCommand::UndoWithPolicy {
                policy: UndoPolicy::TransformAgainstConcurrent,
                semantic_command: None,
            })
            .expect("transform undo removes the local edit from mid-timeline");
        assert_eq!(
            store.applied_edit_ids(),
            &[foreign_id.clone()],
            "only the local edit is removed; the concurrent foreign edit stays applied"
        );
        assert_eq!(store.redo_edit_ids(), &[local_edit_id.clone()], "the local edit is on the redo stack");
        assert_eq!(store.projection().expect("projection").n, 2, "projection re-materializes from the foreign edit alone");

        store.dispatch(DocumentVcsCommand::Redo).expect("redo brings the local edit back");
        assert_eq!(store.applied_edit_ids().len(), 2);
        assert_eq!(store.projection().expect("projection").n, 1, "redo re-applies the local edit at the tail");
    }

    #[test]
    fn collection_diff_from_op_projects_each_variant() {
        let items: Vec<DemoItem> = vec![
            DemoItem { id: "a".into(), value: 1 },
            DemoItem { id: "b".into(), value: 2 },
        ];
        let added = collection_diff_from_op::<String, DemoItem, DemoItemPatch>(
            &items,
            &CollectionOp::Add {
                index: 0,
                item: DemoItem { id: "c".into(), value: 3 },
            },
        );
        assert_eq!(added.added.len(), 1);
        assert!(added.removed.is_empty() && added.modified.is_empty());

        let removed = collection_diff_from_op::<String, DemoItem, DemoItemPatch>(&items, &CollectionOp::Remove { id: "a".into() });
        assert_eq!(removed.removed, vec!["a".to_string()]);

        let patched = collection_diff_from_op(
            &items,
            &CollectionOp::Patch {
                id: "b".into(),
                patch: DemoItemPatch { value: Some(9) },
            },
        );
        assert_eq!(patched.modified.len(), 1);
        assert_eq!(patched.modified[0].id, "b");

        let moved = collection_diff_from_op::<String, DemoItem, DemoItemPatch>(&items, &CollectionOp::Move { id: "a".into(), to_index: 1 });
        assert_eq!(moved.removed, vec!["a".to_string()], "move is encoded as remove + re-add by identity");
        assert_eq!(moved.added.len(), 1);
        assert_eq!(moved.added[0].id, "a");
    }

    #[test]
    fn edit_operations_exposes_the_latest_edit() {
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        assert!(store.edit_operations().is_none(), "no edits yet");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let (forwards, backwards, meta) = store.edit_operations().expect("edit operations");
        assert_eq!(forwards, &[DemoOp::SetN { n: 5 }]);
        assert_eq!(backwards, &[DemoOp::SetN { n: 0 }], "backwards restores the pre-state");
        assert_eq!(meta.len(), 1);
    }

    #[test]
    fn amend_last_absorbs_into_matching_coalesce_key() {
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::AmendLast {
                operations: vec![DemoOp::SetN { n: 1 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("first amend");
        store
            .dispatch(DocumentVcsCommand::AmendLast {
                operations: vec![DemoOp::SetN { n: 2 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("second amend");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced into a single edit");
        assert_eq!(store.projection().expect("projection").n, 2);
        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(
            store.projection().expect("projection after undo").n,
            0,
            "undo restores pre-gesture state in one step"
        );
    }

    #[test]
    fn amend_last_starts_new_edit_when_coalesce_key_differs() {
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::AmendLast {
                operations: vec![DemoOp::SetN { n: 1 }],
                coalesce_key: Some("drag-a".into()),
            })
            .expect("first drag");
        store
            .dispatch(DocumentVcsCommand::AmendLast {
                operations: vec![DemoOp::SetN { n: 2 }],
                coalesce_key: Some("drag-b".into()),
            })
            .expect("second drag");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "distinct gestures are separate edits");
    }

    #[test]
    fn amend_last_does_not_absorb_into_committed_edit() {
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::AmendLast {
                operations: vec![DemoOp::SetN { n: 1 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: None,
                authors: Vec::new(),
            })
            .expect("commit");
        store
            .dispatch(DocumentVcsCommand::AmendLast {
                operations: vec![DemoOp::SetN { n: 2 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend after commit");
        assert_eq!(
            store.envelope().vcs.edits.len(),
            2,
            "committed edits are never amended, even with a matching coalesce key"
        );
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoItem {
        id: String,
        value: i32,
    }

    impl Identified<String> for DemoItem {
        fn id(&self) -> &String {
            &self.id
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoItemPatch {
        value: Option<i32>,
    }

    impl Patchable<DemoItemPatch> for DemoItem {
        fn apply_patch(&mut self, patch: &DemoItemPatch) -> DemoItemPatch {
            let inverse = DemoItemPatch { value: Some(self.value) };
            if let Some(value) = patch.value {
                self.value = value;
            }
            inverse
        }
    }

    #[test]
    fn collection_op_add_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem {
            id: "a".into(),
            value: 1,
        }];
        let op = CollectionOp::Add {
            index: 1,
            item: DemoItem {
                id: "b".into(),
                value: 2,
            },
        };
        let mut applied = items.clone();
        apply_collection_op(&mut applied, &op);
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[1].id, "b");
        let inverse = invert_collection_op(&items, &op);
        apply_collection_op(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_move_and_invert() {
        let items: Vec<DemoItem> = vec![
            DemoItem { id: "a".into(), value: 1 },
            DemoItem { id: "b".into(), value: 2 },
            DemoItem { id: "c".into(), value: 3 },
        ];
        let op = CollectionOp::Move {
            id: "a".into(),
            to_index: 2,
        };
        let mut applied = items.clone();
        apply_collection_op(&mut applied, &op);
        assert_eq!(applied.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "a"]);
        let inverse = invert_collection_op(&items, &op);
        apply_collection_op(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_patch_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let op = CollectionOp::Patch {
            id: "a".into(),
            patch: DemoItemPatch { value: Some(9) },
        };
        let mut applied = items.clone();
        apply_collection_op(&mut applied, &op);
        assert_eq!(applied[0].value, 9);
        let inverse = invert_collection_op(&items, &op);
        apply_collection_op(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_remove_and_invert() {
        let items: Vec<DemoItem> = vec![
            DemoItem { id: "a".into(), value: 1 },
            DemoItem { id: "b".into(), value: 2 },
        ];
        let op = CollectionOp::Remove { id: "a".into() };
        let mut applied = items.clone();
        apply_collection_op(&mut applied, &op);
        assert_eq!(applied.len(), 1);
        let inverse = invert_collection_op(&items, &op);
        apply_collection_op(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn test_support_round_trip_helpers_pass_for_demo_op() {
        test_support::assert_operation_round_trip(&DemoProjection { n: 4 }, DemoOp::SetN { n: 9 });
        test_support::assert_store_roundtrip(DemoProjection { n: 4 }, DemoOp::SetN { n: 9 });
    }
}
//#endregion 🧪Tests
