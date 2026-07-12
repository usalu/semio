//! 🗄️ Generic document VCS engine — Operation/Edit/Change/Checkpoint/Alternative, materialize-by-replay, backbone.

use semio_framework_core::{HybridLogicalTimestamp, MergeStrategyKind, UndoPolicy};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use thiserror::Error;

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// @emoji 🆔 Allocates stable ids for document VCS entities.
pub fn create_document_vcs_id(prefix: &str) -> String {
    let n = ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{n}")
}

//#region 🔖Schemas
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackboneKind {
    Temporary,
    File,
    Folder,
    Remote,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentBackboneRef {
    pub kind: BackboneKind,
    pub uri: String,
}

/// @emoji 🧭 Maps a backbone URI scheme to its canonical kind.
pub fn backbone_kind_from_uri(uri: &str) -> BackboneKind {
    match uri.split("://").next().unwrap_or("") {
        "temp" => BackboneKind::Temporary,
        "file" => BackboneKind::File,
        "folder" => BackboneKind::Folder,
        "remote" => BackboneKind::Remote,
        _ => BackboneKind::Temporary,
    }
}

/// @emoji 🔗 Builds a typed backbone reference from a URI.
pub fn document_backbone_ref(uri: &str) -> DocumentBackboneRef {
    DocumentBackboneRef {
        kind: backbone_kind_from_uri(uri),
        uri: uri.to_string(),
    }
}

/// @emoji 🧠 Default in-memory backbone for a document id.
pub fn default_temporary_backbone_ref(document_id: &str) -> DocumentBackboneRef {
    DocumentBackboneRef {
        kind: BackboneKind::Temporary,
        uri: format!("temp://{document_id}"),
    }
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
    let backbone = backbone.or_else(|| Some(default_temporary_backbone_ref(id)));
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

fn latest_checkpoint<P, Op>(envelope: &DocumentVcsEnvelope<P, Op>) -> Option<&Checkpoint>
where
    Op: Clone,
    P: Clone,
{
    envelope.vcs.checkpoints.last()
}
//#endregion 🔖Materialize

//#region 🔖DocumentVcsStore
pub struct DocumentVcsStore<P, Op>
where
    P: Clone + Serialize + DeserializeOwned,
    Op: Clone + Serialize + DeserializeOwned + Operation<P>,
{
    envelope: DocumentVcsEnvelope<P, Op>,
    backbone: Option<Box<dyn Backbone>>,
    applied_edit_ids: Vec<String>,
    redo_edit_ids: Vec<String>,
    edit_sequence: i32,
    generation: u64,
}

impl<P, Op> DocumentVcsStore<P, Op>
where
    P: Clone + Serialize + DeserializeOwned,
    Op: Clone + Serialize + DeserializeOwned + Operation<P>,
{
    pub fn new(envelope: DocumentVcsEnvelope<P, Op>) -> Self {
        let backbone = envelope
            .backbone
            .as_ref()
            .and_then(|entry| resolve_backbone(&entry.uri).ok());
        Self {
            envelope,
            backbone,
            applied_edit_ids: Vec::new(),
            redo_edit_ids: Vec::new(),
            edit_sequence: 0,
            generation: 0,
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
        self.backbone = envelope
            .backbone
            .as_ref()
            .and_then(|entry| resolve_backbone(&entry.uri).ok());
        self.edit_sequence = envelope
            .vcs
            .edits
            .iter()
            .map(|edit| edit.sequence_number)
            .max()
            .unwrap_or(0);
        self.envelope = envelope;
        self.applied_edit_ids = applied_edit_ids;
        self.redo_edit_ids = redo_edit_ids;
        self.bump();
    }

    pub fn projection(&self) -> Result<P, VcsError> {
        materialize_document_projection(&self.envelope, &self.applied_edit_ids)
    }

    pub fn dispatch(&mut self, command: DocumentVcsCommand<Op>) -> Result<(), VcsError> {
        self.dispatch_inner(command)?;
        self.maybe_sync_backbone()
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
            } => {
                let last = self.applied_edit_ids.last().cloned().ok_or(VcsError::NothingToUndo)?;
                let edit = self
                    .envelope
                    .vcs
                    .edits
                    .iter()
                    .find(|edit| edit.id == last)
                    .ok_or_else(|| VcsError::UnknownEdit(last.clone()))?;
                match policy {
                    UndoPolicy::ExactBaseOnly => {
                        self.applied_edit_ids.pop().ok_or(VcsError::NothingToUndo)?;
                        self.redo_edit_ids.push(last);
                    }
                    UndoPolicy::TransformAgainstConcurrent => {
                        self.applied_edit_ids.pop().ok_or(VcsError::NothingToUndo)?;
                        self.redo_edit_ids.push(last);
                    }
                    UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                        if semantic_command.is_none() {
                            return Err(VcsError::Backbone(
                                "semantic undo requires compensating command".into(),
                            ));
                        }
                        self.applied_edit_ids.pop().ok_or(VcsError::NothingToUndo)?;
                        self.redo_edit_ids.push(last);
                    }
                }
                let _ = edit;
                self.bump();
                Ok(())
            }
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
                let parent = latest_checkpoint(&self.envelope);
                let mut change_ids = parent.map(|cp| cp.change_ids.clone()).unwrap_or_default();
                change_ids.push(change.id.clone());
                let checkpoint = Checkpoint {
                    id: create_document_vcs_id("checkpoint"),
                    change_ids,
                    parent_id: parent.map(|cp| cp.id.clone()),
                    authors,
                    message,
                    timestamp: now_iso(),
                };
                self.envelope.vcs.changes.push(change);
                self.envelope.vcs.checkpoints.push(checkpoint);
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
                    .envelope
                    .vcs
                    .checkpoints
                    .last()
                    .map(|cp| cp.id.clone())
                    .ok_or(VcsError::NoCheckpoint)?;
                let alt_id = create_document_vcs_id("alternative");
                self.envelope.vcs.alternatives.push(Alternative {
                    id: alt_id.clone(),
                    name,
                    checkpoint_ids: vec![checkpoint_id],
                });
                self.envelope.active_alternative_id = Some(alt_id);
                let checkpoint = self.envelope.vcs.checkpoints.last().ok_or(VcsError::NoCheckpoint)?;
                self.applied_edit_ids = edit_ids_for_changes(&self.envelope, &checkpoint.change_ids);
                self.redo_edit_ids.clear();
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
                let checkpoint = self
                    .envelope
                    .vcs
                    .checkpoints
                    .iter()
                    .find(|cp| cp.id == checkpoint_id)
                    .ok_or(VcsError::NoCheckpoint)?;
                self.applied_edit_ids = edit_ids_for_changes(&self.envelope, &checkpoint.change_ids);
                self.redo_edit_ids.clear();
                self.envelope.active_alternative_id = Some(alternative_id);
                self.bump();
                Ok(())
            }
            DocumentVcsCommand::CheckoutCheckpoint { checkpoint_id } => {
                let checkpoint = self
                    .envelope
                    .vcs
                    .checkpoints
                    .iter()
                    .find(|cp| cp.id == checkpoint_id)
                    .ok_or_else(|| VcsError::UnknownChange(checkpoint_id.clone()))?;
                self.applied_edit_ids = edit_ids_for_changes(&self.envelope, &checkpoint.change_ids);
                self.redo_edit_ids.clear();
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
                self.edit_sequence += 1;
                let edit = Edit {
                    id: create_document_vcs_id("edit"),
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
                    self.edit_sequence += 1;
                    let edit = Edit {
                        id: create_document_vcs_id("edit"),
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

    pub fn sync_backbone(&self) -> Result<(), VcsError> {
        let backbone = self
            .backbone
            .as_ref()
            .ok_or_else(|| VcsError::Backbone("no backbone attached".into()))?;
        let json = self.envelope_json()?;
        backbone.sync(&json)
    }

    pub fn load_backbone(&mut self) -> Result<(), VcsError> {
        let backbone = self
            .backbone
            .as_ref()
            .ok_or_else(|| VcsError::Backbone("no backbone attached".into()))?;
        let json = backbone.load()?;
        let loaded: DocumentVcsEnvelope<P, Op> =
            serde_json::from_str(&json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
        self.set_envelope(loaded, Vec::new());
        Ok(())
    }

    pub fn attach_backbone(&mut self, uri: &str) -> Result<(), VcsError> {
        self.envelope.backbone = Some(document_backbone_ref(uri));
        self.backbone = Some(resolve_backbone(uri)?);
        self.bump();
        Ok(())
    }

    pub fn detach_backbone(&mut self) {
        let id = self.envelope.id.clone();
        let uri = format!("temp://{id}");
        self.envelope.backbone = Some(document_backbone_ref(&uri));
        self.backbone = resolve_backbone(&uri).ok();
        self.bump();
    }

    pub fn backbone_ref(&self) -> Option<&DocumentBackboneRef> {
        self.envelope.backbone.as_ref()
    }

    fn maybe_sync_backbone(&self) -> Result<(), VcsError> {
        if self.backbone.is_some() {
            self.sync_backbone()?;
        }
        Ok(())
    }

    fn bump(&mut self) {
        self.generation += 1;
    }
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

/// @emoji 🗄️ Opaque envelope persistence — callers only pass a URI.
pub trait Backbone: Send + Sync {
    fn load(&self) -> Result<String, VcsError>;
    fn sync(&self, envelope_json: &str) -> Result<(), VcsError>;
}

pub trait BackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<String, VcsError>;
    fn write(&self, uri: &str, payload: &str) -> Result<(), VcsError>;
}

static TEMPORARY_STORE: LazyLock<MemoryBackbonePort> = LazyLock::new(MemoryBackbonePort::new);
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

pub struct TemporaryBackbone {
    uri: String,
}

impl TemporaryBackbone {
    pub fn new(uri: &str) -> Result<Self, VcsError> {
        if !uri.starts_with("temp://") {
            return Err(VcsError::Backbone(format!("expected temp:// uri, got {uri}")));
        }
        Ok(Self { uri: uri.to_string() })
    }
}

impl Backbone for TemporaryBackbone {
    fn load(&self) -> Result<String, VcsError> {
        TEMPORARY_STORE.read(&self.uri)
    }

    fn sync(&self, envelope_json: &str) -> Result<(), VcsError> {
        TEMPORARY_STORE.write(&self.uri, envelope_json)
    }
}

pub struct FileJsonBackbone {
    uri: String,
}

impl FileJsonBackbone {
    pub fn new(uri: &str) -> Result<Self, VcsError> {
        if !uri.starts_with("file://") {
            return Err(VcsError::Backbone(format!("expected file:// uri, got {uri}")));
        }
        Ok(Self { uri: uri.to_string() })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn file_path(&self) -> Result<std::path::PathBuf, VcsError> {
        let path = self
            .uri
            .strip_prefix("file://")
            .ok_or_else(|| VcsError::Backbone(format!("invalid file uri: {}", self.uri)))?;
        Ok(std::path::PathBuf::from(path))
    }
}

impl Backbone for FileJsonBackbone {
    fn load(&self) -> Result<String, VcsError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(port) = host_backbone_port() {
                return port.read(&self.uri);
            }
            let path = self.file_path()?;
            std::fs::read_to_string(&path).map_err(|e| VcsError::Backbone(e.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            host_backbone_port()
                .ok_or_else(|| VcsError::Backbone("file backbone requires host port".into()))?
                .read(&self.uri)
        }
    }

    fn sync(&self, envelope_json: &str) -> Result<(), VcsError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(port) = host_backbone_port() {
                return port.write(&self.uri, envelope_json);
            }
            let path = self.file_path()?;
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| VcsError::Backbone(e.to_string()))?;
            }
            std::fs::write(&path, envelope_json).map_err(|e| VcsError::Backbone(e.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            host_backbone_port()
                .ok_or_else(|| VcsError::Backbone("file backbone requires host port".into()))?
                .write(&self.uri, envelope_json)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct FolderSqliteBackbone {
    folder: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FolderSqliteBackbone {
    pub fn new(uri: &str) -> Result<Self, VcsError> {
        let folder = uri
            .strip_prefix("folder://")
            .ok_or_else(|| VcsError::Backbone(format!("expected folder:// uri, got {uri}")))?;
        Ok(Self {
            folder: std::path::PathBuf::from(folder),
        })
    }

    fn db_path(&self) -> std::path::PathBuf {
        self.folder.join(".semio").join("document.db")
    }

    fn connection(&self) -> Result<rusqlite::Connection, VcsError> {
        let semio_dir = self.folder.join(".semio");
        std::fs::create_dir_all(&semio_dir).map_err(|e| VcsError::Backbone(e.to_string()))?;
        rusqlite::Connection::open(self.db_path()).map_err(|e| VcsError::Backbone(e.to_string()))
    }

    fn ensure_schema(conn: &rusqlite::Connection) -> Result<(), VcsError> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS document (id INTEGER PRIMARY KEY CHECK (id = 1), json TEXT NOT NULL);",
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Backbone for FolderSqliteBackbone {
    fn load(&self) -> Result<String, VcsError> {
        if let Some(port) = host_backbone_port() {
            return port.read(&format!("folder://{}", self.folder.display()));
        }
        let conn = self.connection()?;
        Self::ensure_schema(&conn)?;
        let json: String = conn
            .query_row("SELECT json FROM document WHERE id = 1", [], |row| row.get(0))
            .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(json)
    }

    fn sync(&self, envelope_json: &str) -> Result<(), VcsError> {
        if let Some(port) = host_backbone_port() {
            return port.write(&format!("folder://{}", self.folder.display()), envelope_json);
        }
        let conn = self.connection()?;
        Self::ensure_schema(&conn)?;
        conn.execute(
            "INSERT INTO document (id, json) VALUES (1, ?1) ON CONFLICT(id) DO UPDATE SET json = excluded.json",
            [envelope_json],
        )
        .map_err(|e| VcsError::Backbone(e.to_string()))?;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
pub struct FolderSqliteBackbone {
    uri: String,
}

#[cfg(target_arch = "wasm32")]
impl FolderSqliteBackbone {
    pub fn new(uri: &str) -> Result<Self, VcsError> {
        if !uri.starts_with("folder://") {
            return Err(VcsError::Backbone(format!("expected folder:// uri, got {uri}")));
        }
        Ok(Self { uri: uri.to_string() })
    }
}

#[cfg(target_arch = "wasm32")]
impl Backbone for FolderSqliteBackbone {
    fn load(&self) -> Result<String, VcsError> {
        host_backbone_port()
            .ok_or_else(|| VcsError::Backbone("folder backbone requires host port".into()))?
            .read(&self.uri)
    }

    fn sync(&self, envelope_json: &str) -> Result<(), VcsError> {
        host_backbone_port()
            .ok_or_else(|| VcsError::Backbone("folder backbone requires host port".into()))?
            .write(&self.uri, envelope_json)
    }
}

pub struct RemoteBackbone {
    uri: String,
    last_conflict: Mutex<Option<StudioConflict>>,
}

impl RemoteBackbone {
    pub fn new(uri: &str) -> Result<Self, VcsError> {
        if !uri.starts_with("remote://") {
            return Err(VcsError::Backbone(format!("expected remote:// uri, got {uri}")));
        }
        Ok(Self {
            uri: uri.to_string(),
            last_conflict: Mutex::new(None),
        })
    }

    pub fn last_conflict(&self) -> Option<StudioConflict> {
        self.last_conflict.lock().ok().and_then(|g| g.clone())
    }

    fn endpoint(&self) -> Result<(String, String), VcsError> {
        let rest = self
            .uri
            .strip_prefix("remote://")
            .ok_or_else(|| VcsError::Backbone(format!("invalid remote uri: {}", self.uri)))?;
        let (host_port, document_id) = rest
            .rsplit_once('/')
            .ok_or_else(|| VcsError::Backbone(format!("remote uri missing document id: {}", self.uri)))?;
        if document_id.is_empty() {
            return Err(VcsError::Backbone(format!(
                "remote uri missing document id: {}",
                self.uri
            )));
        }
        Ok((format!("http://{host_port}"), document_id.to_string()))
    }

    fn record_conflict(&self, message: impl Into<String>) {
        let conflict = StudioConflict {
            kind: "studio-conflict".into(),
            uri: self.uri.clone(),
            message: message.into(),
        };
        if let Ok(mut guard) = self.last_conflict.lock() {
            *guard = Some(conflict);
        }
    }
}

impl Backbone for RemoteBackbone {
    fn load(&self) -> Result<String, VcsError> {
        #[cfg(target_arch = "wasm32")]
        {
            return host_backbone_port()
                .ok_or_else(|| VcsError::Backbone("remote backbone requires host port".into()))?
                .read(&self.uri);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (base, document_id) = self.endpoint()?;
            let url = format!("{base}/documents/{document_id}/envelope");
            let response = ureq::get(&url)
                .call()
                .map_err(|err| {
                    self.record_conflict(err.to_string());
                    VcsError::Backbone(err.to_string())
                })?;
            if response.status() != 200 {
                self.record_conflict(format!("remote load failed with status {}", response.status()));
                return Err(VcsError::Backbone(format!(
                    "remote load failed with status {}",
                    response.status()
                )));
            }
            response
                .into_string()
                .map_err(|err| VcsError::Backbone(err.to_string()))
        }
    }

    fn sync(&self, envelope_json: &str) -> Result<(), VcsError> {
        #[cfg(target_arch = "wasm32")]
        {
            return host_backbone_port()
                .ok_or_else(|| VcsError::Backbone("remote backbone requires host port".into()))?
                .write(&self.uri, envelope_json);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (base, document_id) = self.endpoint()?;
            let url = format!("{base}/documents/{document_id}/envelope");
            let response = ureq::put(&url)
                .set("Content-Type", "application/json")
                .send_string(envelope_json)
                .map_err(|err| {
                    self.record_conflict(err.to_string());
                    VcsError::Backbone(err.to_string())
                })?;
            if response.status() != 200 {
                self.record_conflict(format!("remote sync failed with status {}", response.status()));
                return Err(VcsError::Backbone(format!(
                    "remote sync failed with status {}",
                    response.status()
                )));
            }
            Ok(())
        }
    }
}

/// @emoji 🔌 Resolves a backbone URI to a concrete storage implementation.
pub fn resolve_backbone(uri: &str) -> Result<Box<dyn Backbone>, VcsError> {
    let scheme = uri.split("://").next().unwrap_or("");
    match scheme {
        "temp" => Ok(Box::new(TemporaryBackbone::new(uri)?)),
        "file" => Ok(Box::new(FileJsonBackbone::new(uri)?)),
        "folder" => Ok(Box::new(FolderSqliteBackbone::new(uri)?)),
        "remote" => Ok(Box::new(RemoteBackbone::new(uri)?)),
        _ => Err(VcsError::Backbone(format!("unsupported backbone uri: {uri}"))),
    }
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
    fn temporary_backbone_round_trip() {
        let uri = "temp://demo".to_string();
        let backbone = TemporaryBackbone::new(&uri).expect("backbone");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 1 }, None);
        let json = serde_json::to_string(&envelope).expect("json");
        backbone.sync(&json).expect("sync");
        let loaded: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&backbone.load().expect("load")).expect("parse");
        assert_eq!(loaded.id, "demo");
    }

    #[test]
    fn file_json_backbone_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("demo.json");
        let uri = format!("file://{}", path.display());
        let backbone = FileJsonBackbone::new(&uri).expect("backbone");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 1 }, None);
        let json = serde_json::to_string(&envelope).expect("json");
        backbone.sync(&json).expect("sync");
        let loaded: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&backbone.load().expect("load")).expect("parse");
        assert_eq!(loaded.id, "demo");
    }

    #[test]
    fn folder_sqlite_backbone_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let uri = format!("folder://{}", dir.path().display());
        let backbone = FolderSqliteBackbone::new(&uri).expect("backbone");
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 3 }, None);
        let json = serde_json::to_string(&envelope).expect("json");
        backbone.sync(&json).expect("sync");
        let loaded: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&backbone.load().expect("load")).expect("parse");
        assert_eq!(loaded.vcs.initial_projection.n, 3);
    }

    #[test]
    fn store_attaches_and_auto_syncs_temporary_backbone() {
        let envelope: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            create_document_vcs_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentVcsStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![DemoOp::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        let loaded = TemporaryBackbone::new("temp://demo")
            .expect("backbone")
            .load()
            .expect("load");
        let parsed: DocumentVcsEnvelope<DemoProjection, DemoOp> =
            serde_json::from_str(&loaded).expect("parse");
        assert_eq!(parsed.vcs.edits.len(), 1);
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
