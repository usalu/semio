"""🏪️ Stages sealed Hub edits and their exact durable WAL outbox descriptor."""

import hashlib
import json
import runpy
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
BASE = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db")
PROTOCOL = Path("🧰️framework/🔨️modules/📡️replication")
CAUSAL = PROTOCOL / "🚪️io/💾️binary/🔗️causal/🦀️.rs"
FACADE = PROTOCOL / "📦️packages/🦀️rust/🦀️.rs"
HUB_CODEC = CAUSAL.with_name("🏪️atomic-hub.rs")
CONTRACT = BASE / "🗿️artifact/🧬️contract/🏷️ids-command-and-messages/🦀️.rs"
SUBMIT = BASE / "🗿️artifact/⚙️engine/🚦️engine-submit-pipeline/🦀️.rs"
LIFECYCLE = SUBMIT.parent.with_name("🌱️engine-lifecycle") / "🦀️.rs"
CONFIG = SUBMIT.parent.with_name("⚙️config-index-backlog-and-open-rejection") / "🦀️.rs"
LEDGER = SUBMIT.parent.with_name("📈️ledger-tail") / "🦀️.rs"
SYNC = BASE / "🔄️sync/🪞️replica-sync-protocol/🦀️.rs"
SYNC_LIB = SYNC.parent.parent / "🦀️.rs"
OUT = TICKET / "🗑️generated/raw-hub-atomic-overlay.json"


def stage():
    script = runpy.run_path(str(TICKET / "raw-spr-group-stage.py"))
    if (ROOT / script["MODULE"]).exists():
        base = {"preimages": {}, "candidate": {}}
    else:
        script["stage"]()
        base = json.loads(script["OUT"].read_text(encoding="utf8"))
    pre, post = base["preimages"], base["candidate"]

    def edit(path, anchor, replacement, *, count=1):
        raw = (ROOT / path).read_bytes()
        pre[str(path)] = hashlib.sha256(raw).hexdigest()
        current = post.get(str(path), raw.decode("utf8"))
        assert current.count(anchor) == count, (path, anchor, current.count(anchor))
        post[str(path)] = current.replace(anchor, replacement)

    edit(CAUSAL, "pub mod atomic_group;",
         'pub mod atomic_group;\n#[path = "🏪️atomic-hub.rs"]\npub mod atomic_hub;')
    edit(script["SPR"], "pub use crate::io::binary::causal::{atomic_group,",
         "pub use crate::io::binary::causal::{atomic_group, atomic_hub,")
    assert not (ROOT / HUB_CODEC).exists()
    pre[str(HUB_CODEC)] = None
    post[str(HUB_CODEC)] = (TICKET / "raw-hub-atomic-group.rs").read_text(encoding="utf8")
    export = "pub use semio_framework_replication_causal::binary::{DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES,"
    edit(FACADE, export, "pub use semio_framework_replication_causal::binary::{atomic_group, atomic_hub, VerifiedAtomicEdit, decode_verified_atomic_backbone, DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES,", count=2)
    edit(CONTRACT, "    pub envelopes: Vec<protocol::MutationEnvelope>,",
         "    pub envelopes: Vec<protocol::MutationEnvelope>,\n    atomic_edit: Option<protocol::atomic_hub::AtomicWalEdit>,")
    edit(CONTRACT, "        Ok(CommandBatch { envelopes })", """        if envelopes.iter().any(|row| row.transaction.as_ref().is_some_and(|tx| protocol::atomic_group::is_atomic_backbone_tool(&tx.tool))) {
            return Err(DbError::InvalidArgument("original media rows require authenticated atomic admission".into()));
        }
        Ok(CommandBatch { envelopes, atomic_edit: None })""")
    edit(CONTRACT, "\n}\n\n/// 🎚️ Per-submit durability override", """

    /// 🔐️ Preserves the authored Store identity when the authenticated Hub maps the document key.
    pub fn from_verified_atomic(group: protocol::VerifiedAtomicEdit, internal_document: &protocol::ArtifactId) -> Result<Self, DbError> {
        let (edit_id, mut envelopes) = group.into_parts();
        let atomic_edit = protocol::atomic_hub::AtomicWalEdit { edit_id, rows: envelopes.len() };
        if !atomic_edit.admits(&envelopes) {
            return Err(DbError::InvalidArgument("sealed original media grouping changed".into()));
        }
        for row in &mut envelopes { row.document_id = internal_document.clone(); }
        Ok(Self { envelopes, atomic_edit: Some(atomic_edit) })
    }

    pub fn atomic_edit(&self) -> Option<&protocol::atomic_hub::AtomicWalEdit> { self.atomic_edit.as_ref() }

    #[doc(hidden)]
    pub fn unadmitted_envelopes_for_law(envelopes: Vec<protocol::MutationEnvelope>) -> Self {
        Self { envelopes, atomic_edit: None }
    }
}

/// 🎚️ Per-submit durability override""")
    for rel in ("⚙️engine/🎫️handle/🧪️tests/🔬️unit/🦀️.rs", "⚙️engine/🧪️tests/🔬️unit/🧸️fixtures-batch-and-concurrent-ensure/🦀️.rs"):
        path = BASE / rel
        for old, new in (("CommandBatch { envelopes }", "CommandBatch::unadmitted_envelopes_for_law(envelopes)"),
                         ("CommandBatch { envelopes: vec![oversize] }", "CommandBatch::unadmitted_envelopes_for_law(vec![oversize])"),
                         ("CommandBatch { envelopes: over }", "CommandBatch::unadmitted_envelopes_for_law(over)")):
            if old in (ROOT / path).read_text(encoding="utf8"):
                edit(path, old, new)
    edit(SUBMIT, "        if let Some(cached) = self.applied_receipts.get(&command_id.0) {", """        if let Some(group) = batch.atomic_edit() {
            if !group.admits(&batch.envelopes) || batch.envelopes.iter().any(|row| row.diff.schema.0 == DB_PATHMAP_SCHEMA || row.inverse.schema.0 == DB_PATHMAP_SCHEMA) ||
                !matches!(options.durability, DurabilityClass::Fsync | DurabilityClass::Quorum(_)) {
                return Err(DbError::InvalidArgument("original media requires one durable grouped commit".into()));
            }
            let committed = batch.envelopes.iter().filter(|row| self.applied.contains_key(&row.mutation_id.0)).count();
            if let Some(prior) = self.committed_atomic_edits.get(&group.edit_id) {
                if committed != batch.envelopes.len() ||
                    !prior.iter().eq(batch.envelopes.iter().map(|row| &row.mutation_id)) ||
                    batch.envelopes.iter().any(|row| self.applied.get(&row.mutation_id.0) != Some(row)) {
                    return Err(DbError::Conflict("original media edit identity was committed with different rows".into()));
                }
            } else if committed != 0 {
                return Err(DbError::Conflict("original media overlaps an established edit".into()));
            }
        } else if batch.envelopes.iter().any(|row| row.transaction.as_ref().is_some_and(|tx| protocol::atomic_group::is_atomic_backbone_tool(&tx.tool))) {
            return Err(DbError::InvalidArgument("original media rows require authenticated atomic admission".into()));
        }
        if let Some(cached) = self.applied_receipts.get(&command_id.0) {""")
    edit(SUBMIT, "        if planned.is_empty() {", """        if batch.atomic_edit().is_some() && !planned.is_empty() && planned.len() != batch.envelopes.len() {
            return Err(DbError::Conflict("original media cannot publish a partial edit".into()));
        }
        if planned.is_empty() {""")
    edit(SUBMIT, "        let mut records = db_wal_log::WalRecordBatch::new();\n        let _ = records.push(db_wal_log::WalRecord::Frontier(Frontier", """        let mut records = db_wal_log::WalRecordBatch::new();
        if let Some(group) = batch.atomic_edit() {
            let body = group.encode().map_err(|message| DbError::InvalidArgument(message.into()))?;
            let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let mut control = db_wal_log::WalCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 4096)?;
            let descriptor = admit_wal_bytes(body, protocol::atomic_hub::MAX_ATOMIC_WAL_DESCRIPTOR_BYTES as u64, &mut control).await?;
            push_wal_record(&mut records, db_wal_log::WalRecord::Outbox(descriptor), &mut control).await?;
        }
        let _ = records.push(db_wal_log::WalRecord::Frontier(Frontier""")
    edit(SUBMIT, """        self.wal.preflight_submit(&commands, &records)?;
        let _ = records.close_step()?;""", """        if let Err(error) = self.wal.preflight_submit(&commands, &records) {
            close_wal_record_batch(&mut records).await?;
            return Err(error);
        }
        let _ = records.close_step()?;""")
    edit(SUBMIT, """        let wal_facet = self.storage.wal().await;
        self.wal.begin_write(&wal_facet).await?;
        drop(wal_facet);""", """        let wal_facet = self.storage.wal().await;
        let writable = self.wal.begin_write(&wal_facet).await;
        drop(wal_facet);
        if let Err(error) = writable {
            close_wal_record_batch(&mut records).await?;
            return Err(error);
        }""")
    edit(SUBMIT, "        for (envelope, plan) in planned {\n            self.commit_one(envelope, plan.entries).await?;\n            applied_actor_seqs.push(*self.actor_seq.get(envelope.actor.0.as_str()).unwrap_or(&0));", """        let grouped = batch.atomic_edit().is_some();
        for (envelope, plan) in planned {
            if grouped {
                if !plan.entries.is_empty() {
                    close_wal_record_batch(&mut records).await?;
                    return Err(DbError::Internal("original media may only stage registered semantic rows".into()));
                }
            } else {
                self.commit_one(envelope, plan.entries).await?;
                applied_actor_seqs.push(*self.actor_seq.get(envelope.actor.0.as_str()).unwrap_or(&0));
            }""")
    edit(SUBMIT, """        for (envelope, _) in &newly_applied {
            self.remember_target(envelope);
        }
        for touch in batch_touches {
            remember_recent_touch(&mut self.recent_touches, touch);
        }

        let new_frontier = Frontier { document: self.document.clone(), head_seq, commit_seq: self.frontier.commit_seq + 1, chain_hash: self.state.content_hash().await?.0, epoch: self.frontier.epoch };
        let mut records = db_wal_log::WalRecordBatch::new();
        let _ = records.push(db_wal_log::WalRecord::Frontier(new_frontier.clone()));""", """        let state_hash = match self.state.content_hash().await {
            Ok(hash) => hash,
            Err(error) => {
                close_wal_record_batch(&mut records).await?;
                return Err(error);
            }
        };
        let new_frontier = Frontier { document: self.document.clone(), head_seq, commit_seq: self.frontier.commit_seq + 1, chain_hash: state_hash.0, epoch: self.frontier.epoch };
        let _ = records.push(db_wal_log::WalRecord::Frontier(new_frontier.clone()));""")
    edit(SUBMIT, """        let appended = self.wal.submit(&wal_facet, &commands, &records, options.durability, now_ms).await?;
        drop(wal_facet);""", """        let appended = self.wal.submit(&wal_facet, &commands, &records, options.durability, now_ms).await;
        drop(wal_facet);
        close_wal_record_batch(&mut records).await?;
        let appended = appended?;
        if grouped {
            for (envelope, _) in &newly_applied {
                self.applied.insert(envelope.mutation_id.0.clone(), (*envelope).clone());
                let ordinal = self.actor_seq.entry(envelope.actor.0.as_str().to_owned()).or_insert(0);
                *ordinal += 1;
                applied_actor_seqs.push(*ordinal);
            }
            let group = batch.atomic_edit().expect("admitted original semantic group");
            self.committed_atomic_edits.insert(group.edit_id.clone(), batch.envelopes.iter().map(|row| row.mutation_id.clone()).collect());
            self.head_edit_id = Some(protocol::MutationId(group.edit_id.clone()));
        }
        for (envelope, _) in &newly_applied {
            self.remember_target(envelope);
        }
        for touch in batch_touches {
            remember_recent_touch(&mut self.recent_touches, touch);
        }""")
    edit(SUBMIT, "        let _ = records.close_step()?;\n        drop(records);",
         "        drop(records);")
    edit(SUBMIT, "        self.head_edit_id = Some(command_id.clone());",
         "        if !grouped { self.head_edit_id = Some(command_id.clone()); }")
    edit(SUBMIT, "        if !projection_classes.is_empty() {",
         "        if !projection_classes.is_empty() && !grouped {")
    edit(SUBMIT, "        let receipt = CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(self.state.content_hash().await?), messages };",
         "        let receipt = CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(state_hash), messages };")
    edit(CONFIG, "    pub(super) applied: HashMap<String, protocol::MutationEnvelope>,",
         "    pub(super) applied: HashMap<String, protocol::MutationEnvelope>,\n    pub(super) committed_atomic_edits: HashMap<String, Vec<protocol::MutationId>>,")
    edit(LIFECYCLE, "            applied: HashMap::new(),", "            applied: HashMap::new(),\n            committed_atomic_edits: HashMap::new(),")
    edit(LIFECYCLE, "                    let mut durable_group = None;",
         "                    let mut durable_group = None;\n                    let mut atomic_group = None;\n                    let mut atomic_rows = Vec::new();")
    edit(LIFECYCLE, "                                replay_head_edit_id = Some(envelope.mutation_id.clone());",
         """                                if envelope.transaction.as_ref().is_some_and(|tx| protocol::atomic_group::is_atomic_backbone_tool(&tx.tool)) {
                                    atomic_rows.push(envelope.mutation_id.clone());
                                }
                                replay_head_edit_id = Some(envelope.mutation_id.clone());""")
    edit(LIFECYCLE, "                            db_wal_log::WalRecord::Event(bytes) => {",
         """                            db_wal_log::WalRecord::Outbox(bytes) => {
                                if let Some(decoded) = db_sync::decode_atomic_wal_descriptor(bytes)? {
                                    if atomic_group.replace(decoded).is_some() {
                                        return Err(DbError::Corrupt("artifact frontier repeats an original edit descriptor".into()));
                                    }
                                }
                            }
                            db_wal_log::WalRecord::Event(bytes) => {""")
    edit(LIFECYCLE, "                    if let Some(record) = durable_group {",
         """                    if let Some(group) = atomic_group {
                        if atomic_rows.len() != batch_ids.len() ||
                            !group.admits(atomic_rows.iter().filter_map(|id| engine.applied.get(&id.0))) ||
                            engine.committed_atomic_edits.contains_key(&group.edit_id) {
                            return Err(DbError::Corrupt("artifact committed original edit lost exact WAL grouping".into()));
                        }
                        replay_head_edit_id = Some(protocol::MutationId(group.edit_id.clone()));
                        engine.committed_atomic_edits.insert(group.edit_id, atomic_rows);
                    } else if !atomic_rows.is_empty() {
                        return Err(DbError::Corrupt("artifact committed original edit has no WAL descriptor".into()));
                    }
                    if let Some(record) = durable_group {""")
    edit(LEDGER, "pub struct ArtifactLedgerCommit {\n    pub point: ArtifactLedgerPoint,",
         "pub struct ArtifactLedgerCommit {\n    pub point: ArtifactLedgerPoint,\n    pub original_edit: Option<protocol::atomic_hub::AtomicWalEdit>,")
    edit(LEDGER, "            let mut decision = None;\n            loop {",
         "            let mut decision = None;\n            let mut original_edit = None;\n            loop {")
    edit(LEDGER, "                    db_wal_log::WalRecord::Event(bytes) => {", """                    db_wal_log::WalRecord::Outbox(bytes) => {
                        if let Some(group) = db_sync::decode_atomic_wal_descriptor(bytes)? {
                            if original_edit.replace(group).is_some() {
                                return Err(DbError::Corrupt("ledger transaction repeats its original edit".into()));
                            }
                        }
                    }
                    db_wal_log::WalRecord::Event(bytes) => {""")
    edit(LEDGER, "            let next = match (decision, frontier, envelopes.last()) {", """            if let Some(group) = &original_edit {
                if !group.admits(&envelopes) || decision.is_some() || frontier.is_none() {
                    return Err(DbError::Corrupt("ledger transaction lost exact original edit grouping".into()));
                }
            } else if envelopes.iter().any(|row| row.transaction.as_ref().is_some_and(|tx| protocol::atomic_group::is_atomic_backbone_tool(&tx.tool))) {
                return Err(DbError::Corrupt("ledger original edit lost its WAL descriptor".into()));
            }
            let next = match (decision, frontier, envelopes.last()) {""")
    edit(LEDGER, "head_edit_id: Some(last.mutation_id.clone())",
         "head_edit_id: Some(original_edit.as_ref().map(|group| protocol::MutationId(group.edit_id.clone())).unwrap_or_else(|| last.mutation_id.clone()))")
    edit(LEDGER, "ArtifactLedgerCommit { point: point.clone(), envelopes }",
         "ArtifactLedgerCommit { point: point.clone(), original_edit, envelopes }")
    edit(SYNC, "/// 📖️ Inverse of " + chr(96) + "encode_command_envelope", """/// 🧾️ Reads only the bounded original group marker alongside this committed WAL transaction.
pub fn decode_atomic_wal_descriptor(bytes: &db_wal_log::WalBytes) -> Result<Option<protocol::atomic_hub::AtomicWalEdit>, DbError> {
    if bytes.len() > protocol::atomic_hub::MAX_ATOMIC_WAL_DESCRIPTOR_BYTES { return Ok(None); }
    with_wal_command(bytes, protocol::atomic_hub::AtomicWalEdit::decode).map_err(|message| DbError::Corrupt(message.into()))
}

/// 📖️ Inverse of """ + chr(96) + "encode_command_envelope")
    edit(SYNC_LIB, "    commands_server_frame, decide_bootstrap, decode_wal_command,",
         "    commands_server_frame, decide_bootstrap, decode_atomic_wal_descriptor, decode_wal_command,")
    edit(SYNC, "    pub commands: Vec<protocol::MutationEnvelope>,",
         "    pub commands: Vec<protocol::MutationEnvelope>,\n    pub atomic_edits: Vec<AtomicReplaySpan>,")
    edit(SYNC, "/// 🔁️ Replays committed transactions",
         """#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtomicReplaySpan {
    pub range: std::ops::Range<usize>,
    pub group: protocol::atomic_hub::AtomicWalEdit,
}

/// 🔁️ Replays committed transactions""")
    edit(SYNC, "    let mut commands = Vec::new();\n    let mut chain", "    let mut commands = Vec::new();\n    let mut atomic_edits = Vec::new();\n    let mut chain")
    edit(SYNC, "            loop {\n                match transaction.next_record_step()?",
         "            let start = commands.len();\n            let mut group = None;\n            loop {\n                match transaction.next_record_step()?")
    edit(SYNC, "                    db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::SnapshotPub",
         """                    db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Outbox(bytes)) if bytes.len() <= protocol::atomic_hub::MAX_ATOMIC_WAL_DESCRIPTOR_BYTES => {
                        let decoded = with_wal_command(&bytes, protocol::atomic_hub::AtomicWalEdit::decode)
                            .map_err(|message| DbError::Corrupt(message.into()))?;
                        if let Some(decoded) = decoded {
                            if group.replace(decoded).is_some() {
                                return Err(DbError::Corrupt("WAL transaction repeats its atomic descriptor".into()));
                            }
                        }
                    }
                    db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::SnapshotPub""")
    edit(SYNC, "            transaction.finish()?;\n            commit_seq", """            transaction.finish()?;
            if let Some(group) = group {
                if !group.admits(&commands[start..]) || atomic_edits.iter().any(|span: &AtomicReplaySpan| span.group.edit_id == group.edit_id) {
                    return Err(DbError::Corrupt("committed WAL atomic edit does not own all of its declared rows".into()));
                }
                atomic_edits.push(AtomicReplaySpan { range: start..commands.len(), group });
            } else if commands[start..].iter().any(|row| row.transaction.as_ref().is_some_and(|tx| protocol::atomic_group::is_atomic_backbone_tool(&tx.tool))) {
                return Err(DbError::Corrupt("committed original media lost its atomic WAL descriptor".into()));
            }
            commit_seq""")
    edit(SYNC, "Ok(ArtifactSyncState { frontier, commands, floor_head_seq })", "Ok(ArtifactSyncState { frontier, commands, atomic_edits, floor_head_seq })")
    runpy.run_path(str(TICKET / "raw-hub-paged-db-admission.py"))["stage"](ROOT, edit)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps({"preimages": pre, "candidate": post}, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(post)} authenticated Hub WAL group postimages")


if __name__ == "__main__":
    stage()
