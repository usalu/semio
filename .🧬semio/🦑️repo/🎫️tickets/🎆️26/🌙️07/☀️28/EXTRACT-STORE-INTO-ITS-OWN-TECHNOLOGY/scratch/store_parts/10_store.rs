//#region 🔖️DocumentStore
pub struct DocumentStore<P, Operation>
where
    P: Clone + Serialize + DeserializeOwned,
    Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P>,
{
    envelope: DocumentEnvelope<P, Operation>,
    backbone: Option<Box<dyn Backbone>>,
    dag: semio_framework_core::OpDag,
    applied_edit_ids: Vec<String>,
    redo_edit_ids: Vec<String>,
    edit_sequence: i32,
    generation: u64,
    /// @emoji 🧭️ The checkpoint new commits parent onto; advances on commit/checkout/switch. Not
    /// part of the wire envelope — callers that reconstruct the store per call (e.g. a WASM plugin)
    /// must save/restore it themselves via {@link current_checkpoint_id}/{@link set_current_checkpoint_id}.
    current_checkpoint_id: Option<String>,
    /// @emoji 🖋️ Identity of the local actor driving this store. Set from each local `Apply`/
    /// `AmendLast`'s operation author; compared against `Edit.actor` so undo never touches foreign
    /// edits. Not part of the wire envelope — callers that reconstruct the store per call must
    /// save/restore it via {@link local_actor_id}/{@link set_local_actor_id}.
    local_actor_id: Option<String>,
    /// @emoji 🤝️ Conflicts reported by the last {@link Operation::reconcile} pass, refreshed after
    /// remote ingestion (see {@link ingest_envelope}). Empty for every document kind that keeps the
    /// default no-operation `reconcile`. Not part of the wire envelope — it is derived, not source of truth.
    conflicts: Vec<StudioConflict>,
    /// @emoji ⚡️ The live, incrementally-maintained RAW fold of `initial_projection` over every
    /// `forwards` operation in `applied_edit_ids` order — i.e. exactly what a full
    /// {@link materialize_document_projection} replay computes BEFORE its single final
    /// {@link Operation::reconcile} call. Kept in lock-step by every mutating command below instead of
    /// replaying on every read, so `projection()`/`Apply`/`AmendLast` are O(new work) instead of
    /// O(total history). Cold-path commands (checkout/switch/set_state, which reassign
    /// `applied_edit_ids` wholesale rather than appending) fall back to a full raw-fold recompute —
    /// see `fold_current`. Differential ground truth: `test_support::assert_live_equals_replay`.
    current: P,
    /// @emoji 🪢️ `(edit_id, projection right before that edit's forwards were first applied)` for
    /// whichever edit is CURRENTLY the tail of `applied_edit_ids` — refreshed by `Apply`/`AmendLast`
    /// (fresh-edit branch)/`Redo`, left untouched by further amends to the same edit (so it always
    /// points at the state before the edit as a whole, not before its latest increment). Powers an
    /// O(1) `Undo` of exactly this edit; any other undo (not the cached tail, or `None`) falls back
    /// to `fold_current` — always correct, just not always O(1).
    tail_undo_cache: Option<(String, P)>,
}

/// @emoji 🖋️ Derives an edit's authoring actor from its per-operation metadata (the author of its
/// first operation), so a local edit records who produced it for later `UndoPolicy` classification.
fn edit_actor_from_meta(operation_meta: &[OperationMeta]) -> Option<String> {
    operation_meta.first().and_then(|meta| meta.author_id.clone()).map(|actor_id| actor_id.0)
}

impl<P, Operation> DocumentStore<P, Operation>
where
    P: Clone + Serialize + DeserializeOwned,
    Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P>,
{
    /// @emoji 🚫️ A store is always constructed with no backbone attached — the envelope's
    /// `backbone` field is a descriptor of the last attachment, never an instruction to
    /// reconnect. Callers attach explicitly via {@link attach_backbone}/{@link attach_backbone_uri}.
    pub fn new(envelope: DocumentEnvelope<P, Operation>) -> Self {
        let current_checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let current = envelope.vcs.initial_projection.clone();
        Self {
            envelope,
            backbone: None,
            dag: semio_framework_core::OpDag::new(),
            applied_edit_ids: Vec::new(),
            redo_edit_ids: Vec::new(),
            edit_sequence: 0,
            generation: 0,
            current_checkpoint_id,
            local_actor_id: None,
            conflicts: Vec::new(),
            current,
            tail_undo_cache: None,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn envelope(&self) -> &DocumentEnvelope<P, Operation> {
        &self.envelope
    }

    pub fn applied_edit_ids(&self) -> &[String] {
        &self.applied_edit_ids
    }

    /// @emoji ↪️ Pending redo stack (edit ids undone since the last fresh `Apply`).
    pub fn redo_edit_ids(&self) -> &[String] {
        &self.redo_edit_ids
    }

    /// @emoji 🧭️ The checkpoint new commits currently parent onto (defaults to the latest checkpoint
    /// on construction/`set_state`; advances on commit/checkout/switch).
    pub fn current_checkpoint_id(&self) -> Option<&str> {
        self.current_checkpoint_id.as_deref()
    }

    /// @emoji 🧭️ Restores the checkout position after reconstructing the store from a serialized
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

    /// @emoji 🔧️ The most recently created/amended edit's `(forwards, backwards, per-operation meta)`.
    /// Used right after `dispatch(Apply{..})`/`AmendLast` to build a `KernelOperation`/`InvocationResult`
    /// with a true inverse from the just-recorded `Edit.backwards`.
    pub fn edit_operations(&self) -> Option<(&[Operation], &[Operation], &[OperationMeta])> {
        self.envelope.vcs.edits.last().map(|edit| {
            (
                edit.forwards.as_slice(),
                edit.backwards.as_slice(),
                edit.operation_meta.as_slice(),
            )
        })
    }

    /// @emoji 📜️ Ancestor-graph rows for this store's checkpoint history. See {@link build_history_columns}.
    pub fn history_columns(&self) -> Vec<HistoryColumn> {
        build_history_columns(&self.envelope)
    }

    pub fn set_envelope(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>) {
        self.set_state(envelope, applied_edit_ids, Vec::new());
    }

    /// @emoji 💾️ Restores full store state including the redo stack, so `Redo` survives
    /// round-tripping through a serialized envelope (e.g. one `dispatch` call per request).
    pub fn set_state(
        &mut self,
        envelope: DocumentEnvelope<P, Operation>,
        applied_edit_ids: Vec<String>,
        redo_edit_ids: Vec<String>,
    ) {
        self.backbone = None;
        self.edit_sequence = envelope
            .vcs
            .edits
            .iter()
            .map(|edit| edit.sequence_number)
            .max()
            .unwrap_or(0);
        self.current_checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        self.envelope = envelope;
        // 🌱️ These ids are adopted directly, not through `dag.insert`, so the dag never learns they're
        // satisfied — seed it or a later remote envelope whose `deps` reference one would sit `Pending`
        // forever (see `OpDag::seed_applied`). Covers every `set_state` caller: `set_envelope`
        // (store reconstruction from a persisted/cloned document), checkpoint checkout, etc.
        self.dag.seed_applied(applied_edit_ids.iter().cloned());
        self.applied_edit_ids = applied_edit_ids;
        self.redo_edit_ids = redo_edit_ids;
        self.conflicts = Vec::new();
        self.tail_undo_cache = None;
        self.current = self.fold_current().expect("set_state: fold_current should not fail for a consistent envelope");
        self.bump();
    }

    /// @emoji 🧭️ Restores applied edits + checkout position for `checkpoint_id`, clearing redo.
    /// Shared by `createAlternative`/`switchAlternative`/`checkoutCheckpoint`. Mirrors premigration
    /// `checkoutCheckpointInternal`. Cold path: reassigns `applied_edit_ids` wholesale (not a tail
    /// append), so `current` is recomputed by a full raw-fold rather than an incremental update.
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
        self.tail_undo_cache = None;
        self.current = self.fold_current().expect("checkout: fold_current should not fail for a consistent envelope");
    }

    /// @emoji ⚡️ The live projection: `Operation::reconcile` applied to the incrementally-maintained
    /// `current` fold. Always `Ok` in practice (kept as `Result` for API stability); O(1) instead of a
    /// full replay. See the `current` field doc for the maintenance invariant.
    pub fn projection(&self) -> Result<P, VcsError> {
        Ok(reconcile_with_last(self.last_applied_operation(), self.current.clone()).0)
    }

    /// @emoji 🤝️ `current` reconciled, plus whatever conflicts {@link Operation::reconcile} reports.
    /// O(1) instead of a full replay — see {@link projection}.
    pub fn projection_with_conflicts(&self) -> Result<(P, Vec<StudioConflict>), VcsError> {
        Ok(reconcile_with_last(self.last_applied_operation(), self.current.clone()))
    }

    /// @emoji 🎞️ The last-applied edit's last forward operation — the instance `reconcile_with_last`
    /// runs `Operation::reconcile` against (see that fn's doc comment for why any single instance is
    /// equivalent to the old per-TYPE associated-fn call for every technology in this repo today).
    fn last_applied_operation(&self) -> Option<&Operation> {
        self.applied_edit_ids.last().and_then(|edit_id| self.envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id)).and_then(|edit| edit.forwards.last())
    }

    /// @emoji 🔂️ Full raw fold of `initial_projection` over every `forwards` op in `applied_edit_ids`
    /// order, WITHOUT the final `Operation::reconcile` pass — the from-scratch computation `current`
    /// is an incrementally-maintained cache of. Used to recompute `current` on the cold paths that
    /// reassign `applied_edit_ids` wholesale instead of appending/popping its tail.
    fn fold_current(&self) -> Result<P, VcsError> {
        let mut projection = self.envelope.vcs.initial_projection.clone();
        for edit_id in &self.applied_edit_ids {
            let edit = self
                .envelope
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

    /// @emoji 🤝️ Conflicts from the last reconciliation pass (see {@link conflicts} field doc).
    pub fn conflicts(&self) -> &[StudioConflict] {
        &self.conflicts
    }

    pub fn dispatch(&mut self, command: DocumentCommand<Operation>) -> Result<(), VcsError> {
        self.pump()?;
        let is_apply = matches!(command, DocumentCommand::Apply { .. });
        self.dispatch_inner(command)?;
        self.flush_outbound(is_apply)
    }

    fn dispatch_inner(&mut self, command: DocumentCommand<Operation>) -> Result<(), VcsError> {
        match command {
            DocumentCommand::Undo => self.dispatch(DocumentCommand::UndoWithPolicy {
                policy: UndoPolicy::ExactBaseOnly,
                semantic_command: None,
            }),
            DocumentCommand::UndoWithPolicy {
                policy,
                semantic_command,
            } => match policy {
                UndoPolicy::ExactBaseOnly => {
                    let last = self.applied_edit_ids.last().cloned().ok_or(VcsError::NothingToUndo)?;
                    if !self.edit_is_local(&last) {
                        return Err(VcsError::ForeignEdit(last));
                    }
                    self.applied_edit_ids.pop();
                    self.redo_edit_ids.push(last.clone());
                    // ⚡️ O(1) fast path when undoing exactly the cached tail edit; any other shape
                    // (cache miss, or a prior mid-history undo already invalidated it) falls back to a
                    // full raw-fold recompute — always correct, see `fold_current`.
                    match self.tail_undo_cache.take() {
                        Some((cached_id, cached_pre)) if cached_id == last => {
                            self.current = cached_pre;
                        }
                        _ => {
                            self.current = self.fold_current()?;
                        }
                    }
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
                    // 🔂️ Removing a MID-history edit has no cheap incremental inverse; cold-path replay.
                    self.tail_undo_cache = None;
                    self.current = self.fold_current()?;
                    self.bump();
                    Ok(())
                }
                UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                    let command_json = semantic_command.ok_or_else(|| {
                        VcsError::Backbone("semantic undo requires compensating command".into())
                    })?;
                    let command: DocumentCommand<Operation> =
                        serde_json::from_str(&command_json).map_err(|e| VcsError::Deserialize(e.to_string()))?;
                    self.dispatch_inner(command)
                }
            },
            DocumentCommand::Redo => {
                let next = self.redo_edit_ids.pop().ok_or(VcsError::NothingToRedo)?;
                self.applied_edit_ids.push(next.clone());
                // ⚡️ Fold the redone edit's forwards onto `current` in their own natural order — cheap
                // and correct regardless of the edit's internal op grouping (unlike undo, this never
                // needs `Edit.backwards`). Re-seeds `tail_undo_cache` so a following Undo is O(1) again.
                if let Some(edit) = self.envelope.vcs.edits.iter().find(|entry| entry.id == next) {
                    let pre = self.current.clone();
                    let mut folded = pre.clone();
                    for operation in &edit.forwards {
                        folded = apply_operation(&folded, operation);
                    }
                    self.current = folded;
                    self.tail_undo_cache = Some((next, pre));
                }
                self.bump();
                Ok(())
            }
            DocumentCommand::CommitCheckpoint { message, authors } => {
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
                // 🎞️ CW3: the new change is pushed BEFORE computing the checkpoint id (was after),
                // so `content_addressed_checkpoint_id` can hash its actual content, not a placeholder.
                self.envelope.vcs.changes.push(change);
                let timestamp = now_iso();
                let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &self.envelope.vcs.changes, message.as_deref(), &authors, &timestamp);
                let checkpoint = Checkpoint {
                    id,
                    change_ids,
                    parent_id,
                    authors,
                    message,
                    timestamp,
                };
                let checkpoint_id = checkpoint.id.clone();
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
            DocumentCommand::CreateAlternative { name } => {
                if self.envelope.vcs.checkpoints.is_empty() {
                    self.dispatch(DocumentCommand::CommitCheckpoint {
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
            DocumentCommand::SwitchAlternative { alternative_id } => {
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
            DocumentCommand::CheckoutCheckpoint { checkpoint_id } => {
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
            DocumentCommand::Apply {
                operations,
                description,
            } => {
                if operations.is_empty() {
                    return Err(VcsError::EmptyApply);
                }
                let started_at = now_iso();
                // ⚡️ `current` is always up to date (maintained by every mutating command below), so
                // this is an O(1) clone instead of a full replay — see the `current` field doc.
                let pre_projection = self.current.clone();
                let (forwards, backwards, operation_meta, post) =
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
                self.tail_undo_cache = Some((edit.id.clone(), pre_projection));
                self.applied_edit_ids.push(edit.id.clone());
                self.envelope.vcs.edits.push(edit);
                self.current = post;
                self.redo_edit_ids.clear();
                self.bump();
                Ok(())
            }
            DocumentCommand::AmendLast {
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
                    // ⚡️ `current` already reflects this edit's existing forwards (it was folded in
                    // when the edit was created or last amended), so it's always the correct base for
                    // the NEW operations — O(1) instead of the old cache-validity dance.
                    let pre_projection = self.current.clone();
                    let (new_forwards, new_backwards, new_operation_meta, post) =
                        Self::replay_operations(&pre_projection, operations);
                    if let Some(edit) = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                        edit.forwards.extend(new_forwards);
                        edit.backwards.extend(new_backwards);
                        edit.operation_meta.extend(new_operation_meta);
                        edit.finished_at = Some(now_iso());
                    }
                    self.current = post;
                    self.redo_edit_ids.clear();
                    self.bump();
                    Ok(())
                } else {
                    let started_at = now_iso();
                    let pre_projection = self.current.clone();
                    let (forwards, backwards, operation_meta, post) =
                        Self::replay_operations(&pre_projection, operations);
                    let actor = edit_actor_from_meta(&operation_meta);
                    self.local_actor_id = actor.clone();
                    self.edit_sequence += 1;
                    let edit_id = create_document_vcs_id("edit");
                    let edit = Edit {
                        id: edit_id.clone(),
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
                    self.tail_undo_cache = Some((edit_id, pre_projection));
                    self.applied_edit_ids.push(edit.id.clone());
                    self.envelope.vcs.edits.push(edit);
                    self.current = post;
                    self.redo_edit_ids.clear();
                    self.bump();
                    Ok(())
                }
            }
        }
    }

    /// @emoji 🔂️ Replays `operations` over `pre_projection`, returning forwards, reversed-backwards,
    /// per-operation metadata, and the resulting projection. Shared by `Apply` and `AmendLast`.
    fn replay_operations(pre_projection: &P, operations: Vec<Operation>) -> (Vec<Operation>, Vec<Operation>, Vec<OperationMeta>, P) {
        let mut projection = pre_projection.clone();
        let mut forwards = Vec::with_capacity(operations.len());
        let mut backwards = Vec::new();
        let mut operation_meta = Vec::with_capacity(operations.len());
        for operation in operations {
            let mut back = operation.backwards(&projection);
            back.reverse();
            backwards.extend(back);
            operation_meta.push(OperationMeta {
                operation_id: Some(operation
                    .operation_id()
                    .unwrap_or_else(|| OperationId(create_document_vcs_id("operation")))),
                dependencies: operation.dependencies(),
                base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp: operation
                    .timestamp()
                    .unwrap_or_else(|| protocol::HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: operation.undo_policy(),
                // 🎞️ CW3: direct blake3 (same primitive `pack_core::ContentHash` uses) replaces the
                // old `semio_framework_hash::hash_bytes` String hash — `protocol_core::PayloadHash` is
                // now `[u8; 32]`, not a hex string. NOT `pack::content_hash`, which reads a pack
                // FILE's footer rather than hashing arbitrary bytes.
                payload_hash: Some(protocol::PayloadHash(*blake3::hash(&serde_json::to_vec(&operation).unwrap_or_default()).as_bytes())),
            });
            projection = apply_operation(&projection, &operation);
            forwards.push(operation);
        }
        (forwards, backwards, operation_meta, projection)
    }

    pub fn dispatch_json(&mut self, command_json: &str) -> Result<(), VcsError> {
        let command: DocumentCommand<Operation> =
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

    /// @emoji 🔗️ Attaches a backbone channel, reconciling any already-persisted state before
    /// seeding it with this store's current snapshot.
    pub fn attach_backbone(&mut self, backbone: Box<dyn Backbone>) -> Result<(), VcsError> {
        self.envelope.backbone = Some(backbone.descriptor());
        self.backbone = Some(backbone);
        self.pump()?;
        self.flush_outbound(false)?;
        self.bump();
        Ok(())
    }

    /// @emoji 🔗️ Resolves a backbone URI and attaches it. Only available inside the wasm sandbox,
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

    /// @emoji 📡️ Drains inbound backbone messages into the edit timeline. Safe to call anytime;
    /// `dispatch` already calls this before every command.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.pump()
    }

    /// @emoji 🕸️ Feeds a remote {@link OperationEnvelope} through the causal DAG, applying it (and any
    /// now-unblocked dependents) into the edit timeline. Closes the sync gap between
    /// `framework/sync`'s `OpDag` and the vcs edit history.
    pub fn ingest_remote(&mut self, envelope: OperationEnvelope) -> Result<(), VcsError> {
        self.dag
            .insert(envelope)
            .map_err(|error| VcsError::Backbone(error.to_string()))?;
        for envelope in self.dag.drain_applied_envelopes() {
            self.ingest_envelope(envelope)?;
        }
        Ok(())
    }

    fn ingest_envelope(&mut self, envelope: OperationEnvelope) -> Result<(), VcsError> {
        let mut edit: Edit<Operation> = edit_from_operation_envelope(&envelope)?;
        edit.actor = Some(envelope.actor.0.clone());
        if self.envelope.vcs.edits.iter().any(|existing| existing.id == edit.id) {
            return Ok(());
        }
        self.edit_sequence = self.edit_sequence.max(edit.sequence_number);
        let edit_id = edit.id.clone();
        // ⚡️ Fold just the new edit's forwards onto the existing `current` (which already reflects
        // every prior applied edit) — algebraically identical to a full raw-fold replay, in O(new ops).
        for operation in &edit.forwards {
            self.current = apply_operation(&self.current, operation);
        }
        self.envelope.vcs.edits.push(edit);
        self.applied_edit_ids.push(edit_id);
        self.tail_undo_cache = None;
        // 🤝️ Tail reconciliation hook: remote ingestion is the one path where this store's projection
        // can diverge from what a local `Apply` alone would produce, so refresh conflicts here.
        let (_, conflicts) = reconcile_with_last(self.last_applied_operation(), self.current.clone());
        self.conflicts = conflicts;
        self.bump();
        Ok(())
    }

    fn merge_remote_snapshot(&mut self, envelope_json: &str) -> Result<(), VcsError> {
        let remote: DocumentEnvelope<P, Operation> =
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
            // 🌱️ A snapshot adopts these edits directly (not through `dag.insert`), so the dag never
            // learns they're satisfied — seed it here or a later envelope whose `deps` point back at
            // one of these ids would sit `Pending` forever (see `OpDag::seed_applied`).
            self.dag.seed_applied(applied.iter().cloned());
            self.applied_edit_ids = applied;
            self.redo_edit_ids.clear();
            self.tail_undo_cache = None;
            // 🔂️ Wholesale replacement, not a tail append — cold-path full raw-fold recompute.
            self.current = self.fold_current()?;
            self.bump();
            return Ok(());
        }
        let existing_edit_ids: HashSet<String> = self.envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
        let mut newly_merged_ids: Vec<String> = Vec::new();
        for edit in remote.vcs.edits {
            if existing_edit_ids.contains(&edit.id) {
                continue;
            }
            self.edit_sequence = self.edit_sequence.max(edit.sequence_number);
            self.applied_edit_ids.push(edit.id.clone());
            newly_merged_ids.push(edit.id.clone());
            // ⚡️ Each newly-merged edit is appended at the tail, so folding its forwards onto `current`
            // in iteration order is exactly a prefix-extension of the existing raw fold.
            for operation in &edit.forwards {
                self.current = apply_operation(&self.current, operation);
            }
            self.envelope.vcs.edits.push(edit);
        }
        self.dag.seed_applied(newly_merged_ids);
        merge_by_id(&mut self.envelope.vcs.changes, remote.vcs.changes, |change| &change.id);
        merge_by_id(&mut self.envelope.vcs.checkpoints, remote.vcs.checkpoints, |checkpoint| &checkpoint.id);
        merge_by_id(&mut self.envelope.vcs.alternatives, remote.vcs.alternatives, |alternative| &alternative.id);
        self.tail_undo_cache = None;
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

    /// @emoji 📥️ Pumps every queued inbound message from the attached backbone into the timeline.
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
                BackboneMessage::Operations { envelopes } => {
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

    /// @emoji 📤️ Sends the just-applied change outward: a single {@link OperationEnvelope} for `Apply`,
    /// or a full snapshot for every structural command (undo/redo/checkpoint/alternative/amend).
    fn flush_outbound(&mut self, is_apply: bool) -> Result<(), VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(());
        };
        let result = if is_apply {
            match self.envelope.vcs.edits.last() {
                Some(edit) => {
                    let deps = self.previous_edit_dependency();
                    match operation_envelope_from_edit(&self.envelope, edit, deps) {
                        Ok(op_envelope) => {
                            // Registers this locally-authored edit as already-applied in our own
                            // DAG, so a later remote envelope that depends on it doesn't stall as pending.
                            let _ = self.dag.insert(op_envelope.clone());
                            backbone.send(BackboneMessage::Operations { envelopes: vec![op_envelope] })
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

/// @emoji 📦️ Serializes an `Edit` into the causal wire envelope exchanged over a backbone channel.
pub fn operation_envelope_from_edit<P, Operation>(
    envelope: &DocumentEnvelope<P, Operation>,
    edit: &Edit<Operation>,
    deps: Vec<OperationId>,
) -> Result<OperationEnvelope, VcsError>
where
    Operation: Serialize,
{
    let payload = serde_json::to_value(edit).map_err(|e| VcsError::Serialize(e.to_string()))?;
    let payload_hash = hash_bytes(&serde_json::to_vec(edit).unwrap_or_default());
    let author_id = edit
        .operation_meta
        .last()
        .and_then(|meta| meta.author_id.clone())
        .map(|actor_id| actor_id.0)
        .unwrap_or_else(|| "local".into());
    // 🎞️ CW3: `OperationMeta.undo_policy` is now `protocol::UndoPolicy` (the moved struct's field
    // type), while this envelope's `InverseOperation.undo_policy` stays `semio_framework_core`'s own
    // (kept local this wave — see the kernel cut-over note on `framework/core`'s `UndoPolicy`); both
    // enums share identical variants, so this is a plain, lossless re-tag, not a real conversion.
    let undo_policy = match edit.operation_meta.last().map(|meta| meta.undo_policy) {
        Some(protocol::UndoPolicy::ExactBaseOnly) | None => UndoPolicy::ExactBaseOnly,
        Some(protocol::UndoPolicy::TransformAgainstConcurrent) => UndoPolicy::TransformAgainstConcurrent,
        Some(protocol::UndoPolicy::SemanticUndo) => UndoPolicy::SemanticUndo,
        Some(protocol::UndoPolicy::CompensatingAction) => UndoPolicy::CompensatingAction,
    };
    Ok(OperationEnvelope {
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

/// @emoji 📦️ Recovers an `Edit` from the causal wire envelope produced by `operation_envelope_from_edit`.
pub fn edit_from_operation_envelope<Operation>(envelope: &OperationEnvelope) -> Result<Edit<Operation>, VcsError>
where
    Operation: DeserializeOwned,
{
    serde_json::from_value(envelope.diff.payload.clone()).map_err(|e| VcsError::Deserialize(e.to_string()))
}
//#endregion 🔖️DocumentStore
