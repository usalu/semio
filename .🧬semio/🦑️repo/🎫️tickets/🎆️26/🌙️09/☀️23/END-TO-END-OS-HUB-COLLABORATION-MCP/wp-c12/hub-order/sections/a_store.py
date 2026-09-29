# 🧭️ Section A: the store folds a sequencer's operations in the sequencer's order and rebases its unconfirmed ones on top.

edit(STORE, """    Member {
        slot: String,
        child_id: String,
        #[dsl(base64)]
        envelopes: Vec<u8>,
    },
}
""", """    Member {
        slot: String,
        child_id: String,
        #[dsl(base64)]
        envelopes: Vec<u8>,
    },
    /// ✅️ The sequencer (the hub) decided these operations of THIS replica, in its order: it committed them to its log,
    /// or refused them and the operations that follow roll them back (transport owner → store). Every operation the
    /// sequencer ordered before them already arrived `Sequenced`, so they leave the backbone's unconfirmed ledger where
    /// they stand, and an operation it orders after them lands after them.
    Committed { op_ids: Vec<String> },
    /// 🧭️ Other authors' operations in the sequencer's order (transport owner → store): the sequencer committed them
    /// before every operation of this replica it has not decided yet, so they land right before the first of those,
    /// which replay on top, and every entry is stamped along that order ([`sequenced_stamp`]) — so the history fold,
    /// which orders by stamp, folds the sequencer's order. `Mutations` from a peer without a sequencer merge by their HLC.
    Sequenced {
        #[dsl(base64)]
        envelopes: Vec<u8>,
    },
}
""", "store committed and sequenced messages")

edit(STORE, """                BackboneMessage::Mutations { envelopes } => {
                    if !envelopes.is_empty() {
                        *self.bytes = Some(std::mem::take(envelopes));
                        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                }
                BackboneMessage::Ack { op_ids } => {
                    if let Some(op_id) = op_ids.pop() {""", """                BackboneMessage::Mutations { envelopes } | BackboneMessage::Sequenced { envelopes } => {
                    if !envelopes.is_empty() {
                        *self.bytes = Some(std::mem::take(envelopes));
                        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                }
                BackboneMessage::Ack { op_ids } | BackboneMessage::Committed { op_ids } => {
                    if let Some(op_id) = op_ids.pop() {""", "store committed and sequenced message retirement")

edit(STORE, """pub struct PortBackbone {
    uri: String,
    channel: Option<Arc<BackboneChannelPorts>>,
}

impl PortBackbone {
    pub async fn new(uri: &str) -> Self {
        Self { uri: uri.to_string(), channel: None }
    }

    pub async fn with_channel(uri: &str, channel: Arc<BackboneChannelPorts>) -> Self {
        Self { uri: uri.to_string(), channel: Some(channel) }
    }
}""", """pub struct PortBackbone {
    uri: String,
    channel: Option<Arc<BackboneChannelPorts>>,
    /// 🧾️ This store's sent operations no sequencer decided yet, oldest first (see [`Backbones::unconfirmed_operations`]).
    unconfirmed: Vec<UnconfirmedOperation>,
    /// ⏭️ The stamp of the newest entry the sequencer decided (see [`sequenced_stamp`]).
    decided: Option<HybridLogicalTimestamp>,
}

impl PortBackbone {
    pub async fn new(uri: &str) -> Self {
        Self { uri: uri.to_string(), channel: None, unconfirmed: Vec::new(), decided: None }
    }

    pub async fn with_channel(uri: &str, channel: Arc<BackboneChannelPorts>) -> Self {
        Self { uri: uri.to_string(), channel: Some(channel), unconfirmed: Vec::new(), decided: None }
    }
}""", "store port backbone ledger")

edit(STORE, """pub struct ChannelBackbone {
    uri: String,
    inbound: Option<Arc<Mutex<VecDeque<BackboneMessage>>>>,
    outbound: Option<Arc<Mutex<VecDeque<BackboneMessage>>>>,
    outbound_wake: Arc<std::sync::OnceLock<Arc<dyn Fn() + Send + Sync>>>,
}""", """pub struct ChannelBackbone {
    uri: String,
    inbound: Option<Arc<Mutex<VecDeque<BackboneMessage>>>>,
    outbound: Option<Arc<Mutex<VecDeque<BackboneMessage>>>>,
    outbound_wake: Arc<std::sync::OnceLock<Arc<dyn Fn() + Send + Sync>>>,
    /// 🧾️ This store's sent operations no sequencer decided yet, oldest first (see [`Backbones::unconfirmed_operations`]).
    unconfirmed: Vec<UnconfirmedOperation>,
    /// ⏭️ The stamp of the newest entry the sequencer decided (see [`sequenced_stamp`]).
    decided: Option<HybridLogicalTimestamp>,
}""", "store channel backbone ledger")
edit(STORE, "        (ChannelBackbone { uri: uri.to_string(), inbound: Some(inbound.clone()), outbound: Some(outbound.clone()), outbound_wake: outbound_wake.clone() }, ChannelBackboneRemote {",
     "        (ChannelBackbone { uri: uri.to_string(), inbound: Some(inbound.clone()), outbound: Some(outbound.clone()), outbound_wake: outbound_wake.clone(), unconfirmed: Vec::new(), decided: None }, ChannelBackboneRemote {", "store channel backbone ledger constructor")

edit(STORE, """                if let Some(channel) = backbone.channel.take() {
                    drop(channel);
                    return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
            }""", """                if let Some(channel) = backbone.channel.take() {
                    drop(channel);
                    return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
                if let Some(operation) = backbone.unconfirmed.pop() {
                    *self.bytes = Some(operation.op_id.into_bytes());
                    return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
            }""", "store port ledger retirement")
edit(STORE, """                match Self::take_unique_queue(&mut backbone.outbound) {
                    Ok(Some(queue)) => {
                        *self.queue = Some(queue);
                        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    Err(()) => return Ok(SnapshotRetirementStep::Blocked),
                    Ok(None) => {}
                }
            }
        }
        drop(self.backbone.take());""", """                match Self::take_unique_queue(&mut backbone.outbound) {
                    Ok(Some(queue)) => {
                        *self.queue = Some(queue);
                        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    Err(()) => return Ok(SnapshotRetirementStep::Blocked),
                    Ok(None) => {}
                }
                if let Some(operation) = backbone.unconfirmed.pop() {
                    *self.bytes = Some(operation.op_id.into_bytes());
                    return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
            }
        }
        drop(self.backbone.take());""", "store channel ledger retirement")

edit(STORE, """impl Backbone for Backbones {
    async fn descriptor(&self) -> ArtifactBackboneRef {""", """/// 🧾️ How many of its own undecided operations a replica remembers: above the transport's own pending limit (the worker
/// refuses a batch beyond 2 000), so a sequenced replica never forgets one its sequencer can still decide; a backbone
/// whose owner never sequences (a local document's) keeps only the newest.
pub const BACKBONE_UNCONFIRMED_OPERATION_CAPACITY: usize = 2_048;

/// 🧾️ An entry (edit operation or history transition) this replica sent that no sequencer decided yet: its wire identity, the
/// stamp it was sent with — every other replica derives its stamp from that one — and its provisional stamp here, moved past
/// every entry the sequencer placed before it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnconfirmedOperation {
    pub op_id: String,
    pub sent: HybridLogicalTimestamp,
    pub stamp: HybridLogicalTimestamp,
}

/// ⏭️ The stamp of an entry the sequencer placed right after the entry stamped `decided`: its own stamp when that is later,
/// else the next logical tick after `decided`, keeping the entry's actor. Every replica applies it along the sequencer's log,
/// so the history fold — which orders entries by stamp — folds the sequencer's order, and replicas of one log hold equal
/// stamps.
pub fn sequenced_stamp(decided: Option<HybridLogicalTimestamp>, stamp: HybridLogicalTimestamp) -> HybridLogicalTimestamp {
    match decided {
        Some(decided) if stamp <= decided => HybridLogicalTimestamp { actor: stamp.actor, physical_ms: decided.physical_ms, logical: decided.logical.saturating_add(1) },
        _ => stamp,
    }
}

/// 🧭️ Where a remote operation lands among a replica's applied operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RemoteOrder {
    /// 🕰️ By its HLC: peers without a sequencer (`BackboneMessage::Mutations`, [`ArtifactStore::ingest_remote`]).
    Causal,
    /// 🧭️ Where the sequencer placed it: before this replica's first operation the sequencer has not decided yet
    /// (`BackboneMessage::Sequenced`), stamped along the sequencer's order.
    Sequenced,
}

impl Backbones {
    /// 🧭️ The store's sent entries no sequencer (the hub) decided yet, oldest first: a `Sequenced` batch lands before
    /// the first of them and they replay on top. Empty on an in-process backbone, whose peers never sequence.
    pub fn unconfirmed_operations(&self) -> &[UnconfirmedOperation] {
        match self {
            Self::Port(backbone) => &backbone.unconfirmed,
            Self::Channel(backbone) => &backbone.unconfirmed,
            Self::Memory(_) => &[],
        }
    }

    /// ⏭️ The stamp of the newest entry the sequencer decided; `None` before the first (and on an in-process backbone).
    pub fn decided_stamp(&self) -> Option<HybridLogicalTimestamp> {
        match self {
            Self::Port(backbone) => backbone.decided,
            Self::Channel(backbone) => backbone.decided,
            Self::Memory(_) => None,
        }
    }

    /// 🧾️ Records entries this store just sent through a transport owner as undecided, with the stamps they were sent with.
    fn record_unconfirmed(&mut self, sent: impl IntoIterator<Item = (String, HybridLogicalTimestamp)>) {
        let (Self::Port(PortBackbone { unconfirmed, .. }) | Self::Channel(ChannelBackbone { unconfirmed, .. })) = self else { return };
        unconfirmed.extend(sent.into_iter().map(|(op_id, stamp)| UnconfirmedOperation { op_id, sent: stamp, stamp }));
        let excess = unconfirmed.len().saturating_sub(BACKBONE_UNCONFIRMED_OPERATION_CAPACITY);
        unconfirmed.drain(..excess);
    }

    /// ⏭️ The sequencer decided the entry stamped `stamp` next: the decided frontier moves to it and every undecided entry
    /// moves past it, answering the entries restamped (see [`Self::restamp_undecided`]).
    fn decide(&mut self, stamp: HybridLogicalTimestamp) -> Vec<(String, HybridLogicalTimestamp)> {
        let (Self::Port(PortBackbone { decided, .. }) | Self::Channel(ChannelBackbone { decided, .. })) = self else { return Vec::new() };
        *decided = Some(stamp);
        self.restamp_undecided()
    }

    /// ✅️ The sequencer decided `op_ids` of this replica, in its order: each leaves the unconfirmed ledger stamped the way
    /// every other replica stamps it — from the stamp it was sent with — answering every entry restamped.
    fn decide_own(&mut self, op_ids: &[String]) -> Vec<(String, HybridLogicalTimestamp)> {
        let mut restamped = Vec::new();
        for op_id in op_ids {
            let (Self::Port(PortBackbone { unconfirmed, decided, .. }) | Self::Channel(ChannelBackbone { unconfirmed, decided, .. })) = self else { return restamped };
            let Some(position) = unconfirmed.iter().position(|operation| operation.op_id == *op_id) else { continue };
            let operation = unconfirmed.remove(position);
            let stamp = sequenced_stamp(*decided, operation.sent);
            if stamp != operation.stamp {
                restamped.push((operation.op_id, stamp));
            }
            restamped.extend(self.decide(stamp));
        }
        restamped
    }

    /// ⏭️ Moves every undecided entry past the decided frontier, keeping their order: the history fold orders by stamp, so
    /// an undecided entry never folds before a decided one. The ledger's stamps rise strictly (the store's clock merges every
    /// stamp it sets), so the walk stops at the first entry already past it.
    fn restamp_undecided(&mut self) -> Vec<(String, HybridLogicalTimestamp)> {
        let (Self::Port(PortBackbone { unconfirmed, decided, .. }) | Self::Channel(ChannelBackbone { unconfirmed, decided, .. })) = self else { return Vec::new() };
        let mut previous = *decided;
        let mut restamped = Vec::new();
        for operation in unconfirmed.iter_mut() {
            let stamp = sequenced_stamp(previous, operation.stamp);
            if stamp == operation.stamp {
                break;
            }
            operation.stamp = stamp;
            restamped.push((operation.op_id.clone(), stamp));
            previous = Some(stamp);
        }
        restamped
    }
}

impl Backbone for Backbones {
    async fn descriptor(&self) -> ArtifactBackboneRef {""", "store unconfirmed ledger and remote order")

edit(STORE, """        let Some(mut backbone) = self.backbone.take() else {
            return Ok(());
        };
        let envelopes = crate::os_spr::encode_envelopes(&std::mem::take(&mut self.pending_report.outbound));
        let result = backbone.send(BackboneMessage::Mutations { envelopes }).await;""", """        let Some(mut backbone) = self.backbone.take() else {
            return Ok(());
        };
        let outbound = std::mem::take(&mut self.pending_report.outbound);
        backbone.record_unconfirmed(outbound.iter().map(|envelope| (envelope.mutation_id.0.clone(), envelope.timestamp)));
        let envelopes = crate::os_spr::encode_envelopes(&outbound);
        let result = backbone.send(BackboneMessage::Mutations { envelopes }).await;""", "store records unconfirmed outbound")

edit(STORE, """                BackboneMessage::Mutations { envelopes } => {
                    let envelopes = crate::os_spr::decode_envelopes(&envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
                    for envelope in envelopes {
                        reports.push(self.ingest_remote(envelope).await?);
                    }
                    acked_op_ids.extend(op_ids);
                }
                member @ BackboneMessage::Member { .. } => self.member_inbox.push_back(member),
                BackboneMessage::Ack { .. } => {}
            }""", """                BackboneMessage::Mutations { envelopes } => acked_op_ids.extend(self.ingest_remote_message(&envelopes, RemoteOrder::Causal, &mut reports).await?),
                BackboneMessage::Sequenced { envelopes } => acked_op_ids.extend(self.ingest_remote_message(&envelopes, RemoteOrder::Sequenced, &mut reports).await?),
                member @ BackboneMessage::Member { .. } => self.member_inbox.push_back(member),
                BackboneMessage::Ack { .. } => {}
                BackboneMessage::Committed { op_ids } => {
                    let restamped = self.backbone.as_mut().map(|backbone| backbone.decide_own(&op_ids)).unwrap_or_default();
                    self.restamp_entries(&restamped);
                }
            }""", "store pump folds sequenced batches and confirms committed operations")

edit(STORE, """    /// 🧬️ A peer's `Genesis` must name this document's own initial snapshot: replicas of one
    /// document share their genesis, so a mismatch is a different document, never something to merge.
    fn verify_genesis(""", """    /// 📥️ Folds one inbound batch of remote operations where `placement` puts them, answering their ids for the `Ack`.
    async fn ingest_remote_message(&mut self, envelopes: &[u8], placement: RemoteOrder, reports: &mut Vec<crate::os_spr::MergeReport>) -> Result<Vec<String>, VcsError> {
        let envelopes = crate::os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
        for envelope in envelopes {
            reports.push(self.ingest_remote_in(envelope, placement).await?);
        }
        Ok(op_ids)
    }

    /// 🧬️ A peer's `Genesis` must name this document's own initial snapshot: replicas of one
    /// document share their genesis, so a mismatch is a different document, never something to merge.
    fn verify_genesis(""", "store remote message ingest")

edit(STORE, """    pub async fn ingest_remote(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<crate::os_spr::MergeReport, VcsError> {
        self.ensure_durable_group_idle()?;
        let no_op_report""", """    pub async fn ingest_remote(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<crate::os_spr::MergeReport, VcsError> {
        self.ingest_remote_in(envelope, RemoteOrder::Causal).await
    }

    /// 🧭️ [`Self::ingest_remote`] with the ready batch landing where `placement` puts it: `Causal` merges it by its HLC
    /// (steps 3–4), `Sequenced` stamps it along the sequencer's order ([`sequenced_stamp`]), moves this replica's undecided
    /// entries past it and places it in arrival order right before the first of them — splitting that operation's edit
    /// when its prefix is decided — and replays those on top (the rebase).
    async fn ingest_remote_in(&mut self, envelope: crate::os_spr::MutationEnvelope, placement: RemoteOrder) -> Result<crate::os_spr::MergeReport, VcsError> {
        self.ensure_durable_group_idle()?;
        let no_op_report""", "store ingest takes its placement")

DRAIN_OLD = """        let mut batch: Vec<Edit<Mutation>> = Vec::new();
        let mut ready_transitions: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
        loop {
            let ready_envelope = match candidate_dag.take_next_applied() {
                crate::os_spr::MutationDagAppliedStep::Envelope(envelope) => envelope,
                crate::os_spr::MutationDagAppliedStep::SeededIdentity => continue,
                crate::os_spr::MutationDagAppliedStep::Complete => break,
            };
            if crate::os_spr::is_history_transition(&ready_envelope) {
                ready_transitions.push(ready_envelope);
                continue;
            }
"""
DRAIN_NEW = """        let mut batch: Vec<Edit<Mutation>> = Vec::new();
        let mut ready_transitions: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
        let mut decided = self.backbone.as_ref().and_then(|backbone| backbone.decided_stamp());
        loop {
            let mut ready_envelope = match candidate_dag.take_next_applied() {
                crate::os_spr::MutationDagAppliedStep::Envelope(envelope) => envelope,
                crate::os_spr::MutationDagAppliedStep::SeededIdentity => continue,
                crate::os_spr::MutationDagAppliedStep::Complete => break,
            };
            if placement == RemoteOrder::Sequenced {
                ready_envelope.timestamp = sequenced_stamp(decided, ready_envelope.timestamp);
            }
            if crate::os_spr::is_history_transition(&ready_envelope) {
                decided = Some(ready_envelope.timestamp);
                ready_transitions.push(ready_envelope);
                continue;
            }
"""
edit(STORE, DRAIN_OLD, DRAIN_NEW, "store ingest stamps a sequenced batch along the sequencer's order")

edit(STORE, """            self.clock.merge(&ready_envelope.timestamp);
            batch.push(edit);
        }
        if batch.is_empty() {
            if ready_transitions.is_empty() {""", """            self.clock.merge(&ready_envelope.timestamp);
            decided = Some(ready_envelope.timestamp);
            batch.push(edit);
        }
        if let Some(stamp) = decided.filter(|_| placement == RemoteOrder::Sequenced) {
            let restamped = self.backbone.as_mut().map(|backbone| backbone.decide(stamp)).unwrap_or_default();
            self.restamp_entries(&restamped);
        }
        if batch.is_empty() {
            if ready_transitions.is_empty() {""", "store ingest moves undecided entries past a sequenced batch")

INGEST_OLD = """        let edit_hlc = |edit: &Edit<Mutation>| edit.mutation_meta.first().map_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, |meta| meta.timestamp);
        batch.sort_by_key(|edit| edit_hlc(edit).cmp_key());
        let batch_keys: Vec<(u64, u64, u64)> = batch.iter().map(|edit| edit_hlc(edit).cmp_key()).collect();
        let known_hlc = |edit_id: &str, edits: &ArtifactHistoryLedger<Edit<Mutation>>| edits.iter().find(|edit| edit.id == *edit_id).map(edit_hlc);
        let min_batch_key = batch_keys[0];
        let mut lo = 0usize;
        let mut hi = self.applied_edit_ids.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let candidate_key = known_hlc(&self.applied_edit_ids[mid], &self.envelope.vcs.edits).map(|hlc| hlc.cmp_key());
            let still_before = candidate_key.is_none_or(|key| key < min_batch_key);
            if still_before {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let mut k = lo;
        for edit in &batch {
            for dependency in edit.mutation_meta.first().map(|meta| meta.dependencies.clone()).unwrap_or_default() {
                if let Some(position) = self.applied_edit_ids.iter().position(|id| *id == dependency.0) {
                    k = k.max(position + 1);
                }
            }
        }
        let mut order: Vec<String> = (*self.applied_edit_ids).clone();
        for (edit, &hlc_key) in batch.iter().zip(batch_keys.iter()) {
            let mut insert_at = order.len();
            for offset in k..order.len() {
                let existing_key = known_hlc(&order[offset], &self.envelope.vcs.edits).map(|existing| existing.cmp_key());
                if existing_key.is_some_and(|key| key > hlc_key) {
                    insert_at = offset;
                    break;
                }
            }
            order.insert(insert_at, edit.id.clone());
        }
"""
INGEST_NEW = """        let edit_hlc = |edit: &Edit<Mutation>| edit.mutation_meta.first().map_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, |meta| meta.timestamp);
        let (k, order) = match placement {
            RemoteOrder::Sequenced => {
                let unconfirmed = self.backbone.as_ref().map(|backbone| backbone.unconfirmed_operations().to_vec()).unwrap_or_default();
                let k = match self.sequenced_rebase_point(&unconfirmed) {
                    Some((applied_index, 0)) => applied_index,
                    Some((applied_index, operation)) => {
                        if let Err(error) = self.split_applied_edit(applied_index, operation) {
                            retire_scratch_edits::<P, Mutation>(batch);
                            return self.refuse_with_candidate_dag(candidate_dag, error);
                        }
                        applied_index + 1
                    }
                    None => self.applied_edit_ids.len(),
                };
                let order: Vec<String> = self.applied_edit_ids[..k].iter().cloned().chain(batch.iter().map(|edit| edit.id.clone())).chain(self.applied_edit_ids[k..].iter().cloned()).collect();
                (k, order)
            }
            RemoteOrder::Causal => {
                batch.sort_by_key(|edit| edit_hlc(edit).cmp_key());
                let batch_keys: Vec<(u64, u64, u64)> = batch.iter().map(|edit| edit_hlc(edit).cmp_key()).collect();
                let known_hlc = |edit_id: &str, edits: &ArtifactHistoryLedger<Edit<Mutation>>| edits.iter().find(|edit| edit.id == *edit_id).map(edit_hlc);
                let min_batch_key = batch_keys[0];
                let mut lo = 0usize;
                let mut hi = self.applied_edit_ids.len();
                while lo < hi {
                    let mid = lo + (hi - lo) / 2;
                    let candidate_key = known_hlc(&self.applied_edit_ids[mid], &self.envelope.vcs.edits).map(|hlc| hlc.cmp_key());
                    let still_before = candidate_key.is_none_or(|key| key < min_batch_key);
                    if still_before {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                let mut k = lo;
                for edit in &batch {
                    for dependency in edit.mutation_meta.first().map(|meta| meta.dependencies.clone()).unwrap_or_default() {
                        if let Some(position) = self.applied_edit_ids.iter().position(|id| *id == dependency.0) {
                            k = k.max(position + 1);
                        }
                    }
                }
                let mut order: Vec<String> = (*self.applied_edit_ids).clone();
                for (edit, &hlc_key) in batch.iter().zip(batch_keys.iter()) {
                    let mut insert_at = order.len();
                    for offset in k..order.len() {
                        let existing_key = known_hlc(&order[offset], &self.envelope.vcs.edits).map(|existing| existing.cmp_key());
                        if existing_key.is_some_and(|key| key > hlc_key) {
                            insert_at = offset;
                            break;
                        }
                    }
                    order.insert(insert_at, edit.id.clone());
                }
                (k, order)
            }
        };
"""
edit(STORE, INGEST_OLD, INGEST_NEW, "store ingest folds a sequenced batch in the sequencer's order")

SPLIT_ANCHOR = """    /// 📥️ Folds an encoded envelope payload a composing parent routed to THIS member's lane."""
SPLIT_FNS = """    /// 🧭️ Where a `Sequenced` batch lands: `(index in applied_edit_ids, operation)` of this replica's first applied
    /// operation no sequencer decided yet — the batch goes before it — or `None` when every applied operation is decided
    /// (the batch goes last). A sequenced batch never lands after an undecided operation, so those are the applied tail
    /// and the scan walks back from the tail only as far as they reach.
    fn sequenced_rebase_point(&self, unconfirmed: &[UnconfirmedOperation]) -> Option<(usize, usize)> {
        if unconfirmed.is_empty() {
            return None;
        }
        let mut point = None;
        for (applied_index, edit_id) in self.applied_edit_ids.iter().enumerate().rev() {
            let Some(edit) = self.envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id) else { break };
            let Some(operation) = crate::os_spr::mutation_ids_for_edit(edit).iter().position(|id| unconfirmed.iter().any(|entry| entry.op_id == id.0)) else { break };
            point = Some((applied_index, operation));
            if operation > 0 {
                break;
            }
        }
        point
    }

    /// ✂️ Splits applied edit `applied_index` before operation `at`: the decided prefix keeps the edit, the undecided suffix
    /// becomes its own edit right after it, named after its first operation, every operation keeping its wire identity
    /// (explicit mutation ids) — so a batch the sequencer committed between the two lands between them. A typing run a
    /// collaborator interrupted becomes two undo steps, one on each side of the collaborator's operation.
    fn split_applied_edit(&mut self, applied_index: usize, at: usize) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids[applied_index].clone();
        let edit = self.envelope.vcs.edits.iter().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        if edit.inverse.len() != edit.forwards.len() || edit.mutation_meta.len() != edit.forwards.len() {
            return Err(VcsError::ValidationFailed(format!("edit {edit_id} cannot split at its first undecided operation: its inverse and metadata are not one per operation")));
        }
        let ids = crate::os_spr::mutation_ids_for_edit(edit);
        let reservation = self.reserve_edit_history_slot()?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        let forwards = edit.forwards.split_off(at);
        let inverse = edit.inverse.split_off(at);
        let mut mutation_meta = edit.mutation_meta.split_off(at);
        for (meta, id) in mutation_meta.iter_mut().zip(&ids[at..]) {
            meta.mutation_id = Some(id.clone());
        }
        let suffix = Edit {
            id: ids[at].0.clone(),
            actor: edit.actor.clone(),
            forwards,
            inverse,
            mutation_meta,
            description: edit.description.clone(),
            coalesce_key: edit.coalesce_key.clone(),
            sequence_number: edit.sequence_number,
            started_at: edit.started_at.clone(),
            finished_at: edit.finished_at.clone(),
        };
        let suffix_id = suffix.id.clone();
        self.insert_reserved_edit_history(reservation, suffix)?;
        for change in self.envelope.vcs.changes.iter_mut() {
            if let Some(position) = change.edit_ids.iter().position(|id| *id == edit_id) {
                change.edit_ids.insert(position + 1, suffix_id.clone());
            }
        }
        let mut applied: Vec<String> = (*self.applied_edit_ids).clone();
        applied.insert(applied_index + 1, suffix_id);
        self.replace_applied_edit_ids_retained(applied)?;
        self.revision_accumulator.forget_from(Some(applied_index), None);
        self.replace_tail_undo_cache_retained(None)?;
        Ok(())
    }

    /// ⏭️ Stamps the entries (edit operations, history transitions) a sequencer decision moved and merges the clock past
    /// them, so an entry this replica authors next folds after every one; an edit restamped in place leaves the revision
    /// records from its position on.
    fn restamp_entries(&mut self, restamped: &[(String, HybridLogicalTimestamp)]) {
        let Some(newest) = restamped.iter().map(|(_, stamp)| *stamp).max() else { return };
        let stamps: HashMap<&str, HybridLogicalTimestamp> = restamped.iter().map(|(op_id, stamp)| (op_id.as_str(), *stamp)).collect();
        let mut changed: HashSet<String> = HashSet::new();
        for edit in self.envelope.vcs.edits.iter_mut() {
            for meta in &mut edit.mutation_meta {
                if let Some(stamp) = meta.mutation_id.as_ref().and_then(|id| stamps.get(id.0.as_str())) {
                    meta.timestamp = *stamp;
                    changed.insert(edit.id.clone());
                }
            }
        }
        for transition in self.envelope.transitions.iter_mut() {
            if let Some(stamp) = stamps.get(transition.mutation_id.0.as_str()) {
                transition.timestamp = *stamp;
            }
        }
        self.clock.merge(&newest);
        let applied = self.applied_edit_ids.iter().position(|id| changed.contains(id));
        let redo = self.redo_edit_ids.iter().position(|id| changed.contains(id));
        self.revision_accumulator.forget_from(applied, redo);
    }

    /// ⏭️ A transport owner's decided frontier starts at the newest stamp this replica holds: every entry it holds when it
    /// attaches is decided (loaded, or folded from its sequencer) and the sequencer's later entries follow it.
    fn seed_decided_stamp(&mut self) {
        let newest = self.envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().map(|meta| meta.timestamp)).chain(self.envelope.transitions.iter().map(|transition| transition.timestamp)).max();
        if let (Some(stamp), Some(backbone)) = (newest, self.backbone.as_mut()) {
            backbone.decide(stamp);
        }
    }

"""
edit(STORE, SPLIT_ANCHOR, SPLIT_FNS + SPLIT_ANCHOR, "store sequenced rebase point and edit split")

edit(STORE, """    fn revision(&self, checkpoint_id: Option<&str>) -> [u8; 32] {
        let applied = self.applied.last().map_or(self.identity_digest, |record| record.prefix_digest);""", """    /// ✂️ Forgets the applied / redo records from these positions on: the edits there changed in place (a sequenced replica
    /// split or restamped them), so the next reconcile recomputes their digests and every prefix after them.
    fn forget_from(&mut self, applied: Option<usize>, redo: Option<usize>) {
        if let Some(index) = applied {
            self.applied.truncate(index);
            self.applied_tail_chains = None;
        }
        if let Some(index) = redo {
            self.redo.truncate(index);
        }
    }

    fn revision(&self, checkpoint_id: Option<&str>) -> [u8; 32] {
        let applied = self.applied.last().map_or(self.identity_digest, |record| record.prefix_digest);""", "store revision records forget a split edit")

edit(STORE, """    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.seed_known_events()?;""", """    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.seed_decided_stamp();
        self.seed_known_events()?;""", "store attach seeds the decided frontier")
edit(STORE, """    pub async fn attach_hot_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.pump().await?;""", """    pub async fn attach_hot_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.seed_decided_stamp();
        self.pump().await?;""", "store hot attach seeds the decided frontier")
