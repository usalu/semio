#!/usr/bin/env python3
"""🗂 H14 14c (c)(d)(e): kernel-db `db_artifact` — the index backlog appends through its owned runs (no listing per append,
level folds), index runs are written after a commit's receipt was sent (the runner maintains the index after replying;
`submit` flushes first only when the backlog outgrew its bound and refuses transiently before any WAL write while index
storage fails), and the never-consumed `outbox`/`commit_log` are removed while `applied_receipts` keeps a bounded window.
Idempotent, region-guarded; `--dry-run` reports only. usage: python3 h14-index-off-ack.py [--dry-run]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"
ENGINE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs"
DRY = "--dry-run" in sys.argv
files = {PATH: open(PATH, encoding="utf-8").read(), ENGINE: open(ENGINE, encoding="utf-8").read()}
problems, states = [], []


def edit(old, new, label, count=1, path=PATH):
    text = files[path]
    if new and new in text and old not in text:
        states.append("done")
        return
    if not new and old not in text:
        states.append("done")
        return
    found = text.count(old)
    if found != count:
        problems.append(f"{label}: expected {count}, found {found}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


edit("""/// @emoji 📤️ One committed operation's opaque effect bytes, queued for downstream
/// replication/notification (`db_sync`/`db_engine`'s concern to actually drain and ship — this
/// crate only accumulates and hands them out via `ArtifactEngine::drain_outbox`).
#[derive(Clone, Debug)]
pub struct OutboxEntry {
    pub mutation_id: protocol::MutationId,
    pub bytes: Vec<u8>,
}

/// @emoji 📣️ One commit's live-query-relevant summary — `ArtifactEngine::commit_log` accumulates
/// these so a poll-based subscriber can diff its last-seen index against the log to discover what
/// changed, without this crate needing an actual push/subscribe transport of its own. `db_query`'s
/// `LiveQuery` (see `🔖️Query`, new this revision) is the push-shaped sibling of this same signal.
#[derive(Clone, Debug)]
pub struct CommitNotification {
    pub frontier: Frontier,
    pub operation_ids: Vec<protocol::MutationId>,
    pub touched: db_state::TouchedSet,
}

""", "", "remove outbox + commit-log types")

edit("""/// @emoji 🎭️ The document authority's real, synchronous pipeline: one open document's WAL,
/// materialized state, causal dependency bookkeeping, previews, and outbox — everything""", """/// @emoji 🎭️ The document authority's real, synchronous pipeline: one open document's WAL,
/// materialized state, causal dependency bookkeeping, previews and index backlog — everything""", "engine doc")

edit("""    applied_receipts: HashMap<String, CommandReceipt>,
    durable_group_receipts""", """    applied_receipts: HashMap<String, CommandReceipt>,
    applied_receipt_order: VecDeque<String>,
    durable_group_receipts""", "receipt order field")

edit("""    head_edit_id: Option<protocol::MutationId>,
    outbox: Vec<OutboxEntry>,
    commit_log: Vec<CommitNotification>,
    previews: db_preview::PreviewStore,""", """    head_edit_id: Option<protocol::MutationId>,
    previews: db_preview::PreviewStore,""", "remove outbox + commit-log fields")

edit("""            applied_receipts: HashMap::new(),
            durable_group_receipts: HashMap::new(),""", """            applied_receipts: HashMap::new(),
            applied_receipt_order: VecDeque::new(),
            durable_group_receipts: HashMap::new(),""", "receipt order init")

edit("""            head_edit_id: None,
            outbox: Vec::new(),
            commit_log: Vec::new(),
            previews:""", """            head_edit_id: None,
            previews:""", "remove outbox + commit-log init")

edit("""/// @emoji 🗂️ Index entries of commits that are durable in the WAL but not yet written as index runs.
/// A commit appends to it; whenever a kind holds a full run (`db_index::RUN_ENTRIES_MAX`), that run
/// is written — at most `INDEX_RUNS_PER_KIND_PER_COMMIT` runs per kind per commit — so the four
/// index kinds cost one run write per 64 entries instead of a write, a listing and a merge per entry.
/// The WAL stays the source of truth: nothing here is durable, and opening a document refills the
/// backlog from the WAL suffix past what each kind's newest run already records.
#[derive(Default)]
struct ArtifactIndexBacklog {
    commands: VecDeque<(u64, db_index::RecordLocation)>,
    inverses: VecDeque<(u64, db_index::RecordLocation)>,
    actor_seqs: VecDeque<(ActorId, u64, u64)>,
    frontiers: VecDeque<Frontier>,
}
""", """/// @emoji 🗂️ Index entries of commits that are durable in the WAL but not yet written as index runs.
/// A commit appends to it; after the commit's receipt was sent the runner writes every full run
/// (`db_index::RUN_ENTRIES_MAX` entries) through the kinds' owned runs — no listing, level folds
/// bounded per append — so the four index kinds cost one run write per 64 entries and no Ack waits
/// for index I/O. The WAL stays the source of truth: nothing here is durable, and opening a document
/// refills the backlog from the WAL suffix past what each kind's newest run already records.
#[derive(Default)]
struct ArtifactIndexBacklog {
    commands: VecDeque<(u64, db_index::RecordLocation)>,
    inverses: VecDeque<(u64, db_index::RecordLocation)>,
    actor_seqs: VecDeque<(ActorId, u64, u64)>,
    frontiers: VecDeque<Frontier>,
    runs: Option<ArtifactIndexRuns>,
}

/// @emoji 🗂️ The four kinds' owned runs, listed once on the backlog's first write and kept by its appends; dropped after
/// a failed write, so the next write lists again what storage really holds.
struct ArtifactIndexRuns {
    commands: db_index::OwnedRuns,
    inverses: db_index::OwnedRuns,
    actor_seqs: db_index::OwnedRuns,
    frontiers: db_index::OwnedRuns,
}
""", "backlog struct")

edit("""/// @emoji 🧺️ The most full runs one commit writes per index kind: enough to keep pace with the
/// largest batch (256 envelopes = 4 runs) and drain a backlog left by a reopen.
const INDEX_RUNS_PER_KIND_PER_COMMIT: usize = 5;

/// @emoji 🛑️ The most entries one index kind may keep waiting for a run write before a commit
/// surfaces the write failure instead of queueing further.
const INDEX_BACKLOG_ENTRIES_MAX: usize = 64 * db_index::RUN_ENTRIES_MAX;
""", """/// @emoji 🛑️ The most entries one index kind may keep waiting for a run write: a `submit` that finds more writes
/// them first, and refuses transiently — before any WAL write — while index storage keeps failing.
const INDEX_BACKLOG_ENTRIES_MAX: usize = 64 * db_index::RUN_ENTRIES_MAX;

/// @emoji 🧾️ How many batch receipts the engine keeps for a whole-batch resend to be answered with its original
/// receipt; an older resend is answered by the per-envelope dedupe with the current frontier.
const APPLIED_RECEIPTS_MAX: usize = 1_024;
""", "backlog constants")

edit("""    /// @emoji 🚚️ Writes this backlog's runs: only full ones (bounded per kind) after a commit, or
    /// everything including a partial tail (`drain`) before WAL history may be deleted. Each written
    /// run leaves the backlog at once, so a failed write resumes exactly where it stopped.
    async fn flush(&mut self, storage: &impl db_storage::IndexStorage, document: &ArtifactId, drain: bool) -> Result<(), DbError> {
        let full = db_index::RUN_ENTRIES_MAX;
        let runs = if drain { usize::MAX } else { INDEX_RUNS_PER_KIND_PER_COMMIT };
        let commands = db_index::CommandIndex::new(storage, document.clone()).await;
        for _ in 0..runs {
            let take = if self.commands.len() >= full { full } else if drain { self.commands.len() } else { 0 };
            if take == 0 {
                break;
            }
            let run: Vec<_> = self.commands.iter().take(take).copied().collect();
            commands.record_run(&run).await?;
            self.commands.drain(..take);
        }
        let inverses = db_index::InverseIndex::new(storage, document.clone()).await;
        for _ in 0..runs {
            let take = if self.inverses.len() >= full { full } else if drain { self.inverses.len() } else { 0 };
            if take == 0 {
                break;
            }
            let run: Vec<_> = self.inverses.iter().take(take).copied().collect();
            inverses.record_run(&run).await?;
            self.inverses.drain(..take);
        }
        let actor_seqs = db_index::ActorSeqIndex::new(storage, document.clone()).await;
        for _ in 0..runs {
            let take = if self.actor_seqs.len() >= full { full } else if drain { self.actor_seqs.len() } else { 0 };
            if take == 0 {
                break;
            }
            let run: Vec<_> = self.actor_seqs.iter().take(take).cloned().collect();
            actor_seqs.record_run(&run).await?;
            self.actor_seqs.drain(..take);
        }
        let frontiers = db_index::FrontierIndex::new(storage, document.clone()).await;
        for _ in 0..runs {
            let take = if self.frontiers.len() >= full { full } else if drain { self.frontiers.len() } else { 0 };
            if take == 0 {
                break;
            }
            let run: Vec<_> = self.frontiers.iter().take(take).cloned().collect();
            frontiers.record_run(&run).await?;
            self.frontiers.drain(..take);
        }
        Ok(())
    }""", """    /// @emoji 🚚️ Writes this backlog's runs: every full one, or everything including a partial tail
    /// (`drain`) before WAL history may be deleted. Each written run leaves the backlog at once, so a
    /// failed write resumes exactly where it stopped, and forgets the owned runs, which the next write
    /// lists again.
    async fn flush(&mut self, storage: &impl db_storage::IndexStorage, document: &ArtifactId, drain: bool) -> Result<(), DbError> {
        let written = self.flush_owned(storage, document, drain).await;
        if written.is_err() {
            self.runs = None;
        }
        written
    }

    async fn flush_owned(&mut self, storage: &impl db_storage::IndexStorage, document: &ArtifactId, drain: bool) -> Result<(), DbError> {
        let full = db_index::RUN_ENTRIES_MAX;
        let take = |len: usize| if len >= full { full } else if drain { len } else { 0 };
        if [self.commands.len(), self.inverses.len(), self.actor_seqs.len(), self.frontiers.len()].into_iter().all(|len| take(len) == 0) {
            return Ok(());
        }
        let commands = db_index::CommandIndex::new(storage, document.clone()).await;
        let inverses = db_index::InverseIndex::new(storage, document.clone()).await;
        let actor_seqs = db_index::ActorSeqIndex::new(storage, document.clone()).await;
        let frontiers = db_index::FrontierIndex::new(storage, document.clone()).await;
        let runs = match self.runs.as_mut() {
            Some(runs) => runs,
            None => self.runs.insert(ArtifactIndexRuns { commands: commands.owned_runs().await?, inverses: inverses.owned_runs().await?, actor_seqs: actor_seqs.owned_runs().await?, frontiers: frontiers.owned_runs().await? }),
        };
        while take(self.commands.len()) > 0 {
            let count = take(self.commands.len());
            let run: Vec<_> = self.commands.iter().take(count).copied().collect();
            commands.record_owned_run(&mut runs.commands, &run).await?;
            self.commands.drain(..count);
        }
        while take(self.inverses.len()) > 0 {
            let count = take(self.inverses.len());
            let run: Vec<_> = self.inverses.iter().take(count).copied().collect();
            inverses.record_owned_run(&mut runs.inverses, &run).await?;
            self.inverses.drain(..count);
        }
        while take(self.actor_seqs.len()) > 0 {
            let count = take(self.actor_seqs.len());
            let run: Vec<_> = self.actor_seqs.iter().take(count).cloned().collect();
            actor_seqs.record_owned_run(&mut runs.actor_seqs, &run).await?;
            self.actor_seqs.drain(..count);
        }
        while take(self.frontiers.len()) > 0 {
            let count = take(self.frontiers.len());
            let run: Vec<_> = self.frontiers.iter().take(count).cloned().collect();
            frontiers.record_owned_run(&mut runs.frontiers, &run).await?;
            self.frontiers.drain(..count);
        }
        Ok(())
    }""", "backlog flush")

edit("""        // dedupe (whole-batch, keyed by the batch's designated command_id)
        if let Some(cached) = self.applied_receipts.get(&command_id.0) {
            return Ok(cached.clone());
        }
""", """        // dedupe (whole-batch, keyed by the batch's designated command_id)
        if let Some(cached) = self.applied_receipts.get(&command_id.0) {
            return Ok(cached.clone());
        }

        // index backlog bound: write what earlier commits queued before this one adds more; refuse
        // transiently while index storage fails, before the WAL holds anything of this batch
        if self.index_backlog.pending() > INDEX_BACKLOG_ENTRIES_MAX {
            if let Err(error) = self.maintain_index().await {
                return Err(DbError::Unavailable(format!("index backlog of {} entries cannot be written: {error}", self.index_backlog.pending())));
            }
        }
""", "submit backlog admission")

edit("""            let receipt = CommandReceipt { command_id, frontier: self.frontier.clone(), durability: options.durability, conflicts: Vec::new(), state_hash: Some(self.state.content_hash().await?), messages: Vec::new() };
            self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone());
            return Ok(receipt);""", """            let receipt = CommandReceipt { command_id, frontier: self.frontier.clone(), durability: options.durability, conflicts: Vec::new(), state_hash: Some(self.state.content_hash().await?), messages: Vec::new() };
            self.remember_receipt(&receipt);
            return Ok(receipt);""", "no-op receipt")

edit("""        for touch in batch_touches {
            remember_recent_touch(&mut self.recent_touches, touch);
        }
        for ((envelope, _), bytes) in newly_applied.iter().zip(&commands) {
            self.outbox.push(OutboxEntry { mutation_id: envelope.mutation_id.clone(), bytes: bytes.clone() });
        }
""", """        for touch in batch_touches {
            remember_recent_touch(&mut self.recent_touches, touch);
        }
""", "remove outbox push")

edit("""        // publish: index entries join the backlog; full runs are written, bounded per commit
        let base_seq""", """        // publish: index entries join the backlog; the runner writes its full runs after the receipt
        let base_seq""", "index comment")

edit("""        self.index_backlog.queue_frontier(&queued, &new_frontier);
        let index_facet = self.storage.index().await;
        let flushed = self.index_backlog.flush(&index_facet, &self.document, false).await;
        if flushed.is_err() && self.index_backlog.pending() > INDEX_BACKLOG_ENTRIES_MAX {
            flushed?;
        }

        // project: run every registered projection over each newly-applied envelope
        let projection_classes = (self.config.projections)();
        if !projection_classes.is_empty() {
            let engine = db_projection::ProjectionEngine::new(&index_facet, self.document.clone(), projection_classes).await?;
            for (offset, (envelope, touched)) in newly_applied.iter().enumerate() {
                engine.apply_envelope(base_seq + offset as u64 + 1, envelope, touched).await?;
            }
        }
        drop(index_facet);

        // preview-reconcile
        self.previews.reconcile_with(&db_preview::LandedCommand { frontier: new_frontier.clone(), touched: touched_all.clone() }, &db_preview::DbConflictOracle::default());
        self.commit_log.push(CommitNotification { frontier: new_frontier.clone(), operation_ids: newly_applied.iter().map(|(envelope, _)| envelope.mutation_id.clone()).collect(), touched: touched_all });
        self.head_edit_id""", """        self.index_backlog.queue_frontier(&queued, &new_frontier);

        // project: run every registered projection over each newly-applied envelope
        let projection_classes = (self.config.projections)();
        if !projection_classes.is_empty() {
            let index_facet = self.storage.index().await;
            let engine = db_projection::ProjectionEngine::new(&index_facet, self.document.clone(), projection_classes).await?;
            for (offset, (envelope, touched)) in newly_applied.iter().enumerate() {
                engine.apply_envelope(base_seq + offset as u64 + 1, envelope, touched).await?;
            }
        }

        // preview-reconcile
        self.previews.reconcile_with(&db_preview::LandedCommand { frontier: new_frontier.clone(), touched: touched_all }, &db_preview::DbConflictOracle::default());
        self.head_edit_id""", "submit index + commit log")

edit("""        let receipt = CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(self.state.content_hash().await?), messages };
        self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone());
        Ok(receipt)
    }
""", """        let receipt = CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(self.state.content_hash().await?), messages };
        self.remember_receipt(&receipt);
        Ok(receipt)
    }

    /// @emoji 🧾️ Keeps `receipt` for a whole-batch resend, forgetting the oldest beyond [`APPLIED_RECEIPTS_MAX`].
    fn remember_receipt(&mut self, receipt: &CommandReceipt) {
        if self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone()).is_none() {
            self.applied_receipt_order.push_back(receipt.command_id.0.clone());
        }
        while self.applied_receipt_order.len() > APPLIED_RECEIPTS_MAX {
            if let Some(oldest) = self.applied_receipt_order.pop_front() {
                self.applied_receipts.remove(&oldest);
            }
        }
    }

    /// @emoji 🗂️ Writes every full index run the commits so far queued — their WAL records are the durable truth
    /// ([`ArtifactIndexBacklog`]). The document runner calls it after a commit's receipt was sent, so no Ack waits
    /// for index I/O; `submit` calls it first only when the backlog outgrew [`INDEX_BACKLOG_ENTRIES_MAX`].
    pub async fn maintain_index(&mut self) -> Result<(), DbError> {
        let index_facet = self.storage.index().await;
        self.index_backlog.flush(&index_facet, &self.document, false).await
    }
""", "receipt + maintain")

edit("""        self.frontier = next_frontier.clone();
        self.head_edit_id = Some(head_edit_id.clone());
        self.durable_group_edit_ids.insert(head_edit_id.0.clone());
        remember_recent_touch(&mut self.recent_touches, durable_group_touch(&head_edit_id.0));
        self.commit_log.push(CommitNotification { frontier: next_frontier, operation_ids: vec![head_edit_id], touched: db_state::TouchedSet::new() });
""", """        self.frontier = next_frontier;
        self.durable_group_edit_ids.insert(head_edit_id.0.clone());
        remember_recent_touch(&mut self.recent_touches, durable_group_touch(&head_edit_id.0));
        self.head_edit_id = Some(head_edit_id);
""", "durable group commit log")

edit("""    pub async fn commit_log(&self) -> &[CommitNotification] {
        &self.commit_log
    }

""", "", "remove commit_log accessor")

edit("""    /// @emoji 📤️ Hands out (and clears) every effect queued since the last drain.
    pub async fn drain_outbox(&mut self) -> Vec<OutboxEntry> {
        std::mem::take(&mut self.outbox)
    }

""", "", "remove drain_outbox")

edit("""    DrainOutbox {
        reply: db_actor::ReplySender<Vec<OutboxEntry>>,
    },
""", "", "remove DrainOutbox message")

edit("""            ArtifactMessage::Submit { batch, options, now_ms, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.submit(batch, options, now_ms).await);
                engine
            })),""", """            ArtifactMessage::Submit { batch, options, now_ms, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.submit(batch, options, now_ms).await);
                let _ = engine.maintain_index().await;
                engine
            })),""", "runner submit")

edit("""            ArtifactMessage::DrainOutbox { reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.drain_outbox().await);
                engine
            })),
""", "", "remove DrainOutbox turn")

edit("""    pub async fn drain_outbox(&self) -> Result<Vec<OutboxEntry>, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::DrainOutbox { reply }).await
    }

""", "", "remove handle drain_outbox")

edit("""//! of this crate per its `Cargo.toml`. `ArtifactHandle::history` replays the WAL through the
//! document authority's retained cursor because its in-memory `commit_log` only contains live
//! submissions from the current process.""", """//! of this crate per its `Cargo.toml`. `ArtifactHandle::history` replays the WAL through the
//! document authority's retained cursor: the WAL is the only record of a document's history.""", "engine module doc", path=ENGINE)

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    for path, text in files.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
