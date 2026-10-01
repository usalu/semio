"""🎞️ W1-G session 2: design §15 transaction-scoped amend in the store — one atomic, count-asserted rewrite.

Moves the open tool transaction onto the envelope (persisted forms and the shared log leave its edit out), lifts it over
remote ingests, routes the batched publication path, and replaces the predecessor's lazy tail re-stamp. Every anchor must
match exactly the expected number of times or nothing is written.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
UNIT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"
HOST = ROOT / "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs"

TOOL_TRANSACTIONS = '''    //#region 🔖️ToolTransactions
    /// 🪡️ `AppendTransaction`: the first append opens one edit stamped with the ref — no description, no coalesce key — and
    /// every later one folds its operations onto the projection and appends them to that edit, every operation stamped with
    /// the ref. The open edit is always the applied tail ([`Self::ingest_remote`] lifts it over remote edits), and neither
    /// the shared log nor a persisted form holds it before its commit, so an abort leaves no trace anywhere.
    async fn append_transaction(&mut self, mutations: Vec<Mutation>, transaction: protocol::TransactionRef) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        let Some(open) = self.envelope.open_transaction.clone() else {
            return self.open_transaction_edit(mutations, transaction).await;
        };
        let pre_snapshot = Arc::clone(&*self.current);
        let (forwards, inverse, mutation_meta, post, messages) = self.replay_mutations(&pre_snapshot, mutations, Some(&transaction)).await?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        edit.forwards.extend(forwards);
        edit.inverse.extend(inverse);
        edit.mutation_meta.extend(mutation_meta);
        self.record_edit_messages(&open.edit_id, messages)?;
        self.replace_current_retained(Arc::new(post))?;
        self.reproject().await?;
        self.bump()
    }

    /// 🪁️ The first append of a tool transaction: one fresh edit at the applied tail holding its operations, open and
    /// unannounced.
    async fn open_transaction_edit(&mut self, mutations: Vec<Mutation>, transaction: protocol::TransactionRef) -> Result<(), VcsError> {
        let started_at = now_iso();
        let pre_snapshot = Arc::clone(&*self.current);
        let (forwards, inverse, mutation_meta, post, messages) = self.replay_mutations(&pre_snapshot, mutations, Some(&transaction)).await?;
        let actor = edit_actor_from_meta(&mutation_meta).await;
        self.replace_local_actor_retained(actor.clone())?;
        self.edit_sequence += 1;
        let forwards_fingerprint = crate::os_pack::json::to_json_string(&forwards).into_bytes();
        let reservation = self.reserve_edit_history_slot()?;
        let edit_id = mint_edit_id(self.clock.actor, self.edit_sequence, &forwards_fingerprint).await;
        let edit = Edit { id: edit_id.clone(), actor, forwards, inverse, mutation_meta, description: None, verb: self.authoring_verb.clone(), coalesce_key: None, sequence_number: self.edit_sequence, started_at, finished_at: None };
        self.insert_reserved_edit_history(reservation, edit)?;
        self.record_edit_messages(&edit_id, messages)?;
        self.replace_tail_undo_cache_retained(Some((edit_id.clone(), pre_snapshot)))?;
        self.applied_edit_ids.push(edit_id.clone());
        self.replace_current_retained(Arc::new(post))?;
        self.envelope.open_transaction = Some(OpenToolTransaction { transaction, edit_id });
        self.reproject().await?;
        self.bump()
    }

    /// 🎗️ `CommitTransaction`: the open edit closes and every operation it holds is announced in one batch — exactly the edit
    /// one `Apply` of the same operations under the same ref would have recorded.
    async fn commit_transaction(&mut self, transaction_id: &str) -> Result<(), VcsError> {
        let open = self.open_transaction_named(transaction_id)?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        edit.finished_at = Some(now_iso());
        stamp_primary_operation_identity(edit);
        let edit = self.envelope.vcs.edits.iter().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        let operations = self.operation_envelopes(edit)?;
        self.envelope.open_transaction = None;
        self.announce_operations(operations)?;
        self.bump()
    }

    /// 🪃️ `AbortTransaction`: the open edit leaves the ledger, its messages and its operations retire, the edit sequence it
    /// took is returned, and the projection folds back to the history without it — the projection and revision of before
    /// its first append.
    async fn abort_transaction(&mut self, transaction_id: &str) -> Result<(), VcsError> {
        let open = self.open_transaction_named(transaction_id)?;
        let removed = self.envelope.vcs.edits.extract_if(|edit| edit.id == open.edit_id).map_err(|fault| VcsError::ValidationFailed(format!("the open transaction edit cannot leave its ledger: {fault:?}")))?;
        if removed.iter().any(|edit| edit.sequence_number == self.edit_sequence) {
            self.edit_sequence -= 1;
        }
        retire_scratch_edits::<P, Mutation>(removed);
        self.replace_edit_messages(&open.edit_id, Vec::new())?;
        self.envelope.open_transaction = None;
        self.reproject().await?;
        self.bump()
    }

    /// 🧿️ The open tool transaction when `transaction_id` names it, else `UnknownTransaction`.
    fn open_transaction_named(&self, transaction_id: &str) -> Result<OpenToolTransaction, VcsError> {
        self.envelope.open_transaction.clone().filter(|open| open.transaction.id == transaction_id).ok_or_else(|| VcsError::UnknownTransaction(transaction_id.to_string()))
    }

    /// 🪂️ Takes the open tool transaction's edit off the applied tail before a remote merge: the ledger, the message ledger
    /// and the projection return to the history without it.
    async fn lift_open_transaction(&mut self) -> Result<Option<(OpenToolTransaction, Edit<Mutation>)>, VcsError> {
        let Some(open) = self.envelope.open_transaction.take() else {
            return Ok(None);
        };
        let mut removed = self.envelope.vcs.edits.extract_if(|edit| edit.id == open.edit_id).map_err(|fault| VcsError::ValidationFailed(format!("the open transaction edit cannot leave its ledger: {fault:?}")))?;
        let edit = removed.pop().ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        retire_scratch_edits::<P, Mutation>(removed);
        self.replace_edit_messages(&open.edit_id, Vec::new())?;
        self.reproject().await?;
        Ok(Some((open, edit)))
    }

    /// 🛞️ Folds a lifted tool transaction's edit back as the applied tail of the merged history: its operations take fresh
    /// clock stamps after everything this replica has seen and its next edit sequence, and the Report replay recomputes
    /// their inverses and messages against the merged head (keep-and-record: nothing is quarantined or refused). When the
    /// ledger can no longer hold it the transaction is aborted — zero trace — and the refusal is answered.
    async fn restore_open_transaction(&mut self, open: OpenToolTransaction, mut edit: Edit<Mutation>) -> Result<(), VcsError> {
        for meta in edit.mutation_meta.iter_mut() {
            self.clock.tick(now_ms());
            meta.timestamp = self.clock;
        }
        self.edit_sequence += 1;
        edit.sequence_number = self.edit_sequence;
        let reservation = match self.reserve_edit_history_slot() {
            Ok(reservation) => reservation,
            Err(error) => {
                self.edit_sequence -= 1;
                retire_scratch_edits::<P, Mutation>([edit]);
                return Err(error);
            }
        };
        self.insert_reserved_edit_history(reservation, edit)?;
        self.envelope.open_transaction = Some(open);
        self.reproject().await?;
        self.bump()
    }
    //#endregion 🔖️ToolTransactions
'''

INGEST_WRAPPER = '''    /// 🛟️ The sole public remote write gate ([`Self::ingest_remote_merge`]). While a tool transaction is open its
    /// unannounced edit is lifted off the applied tail for the merge and folded back onto the merged head after it, so a
    /// remote edit never reorders, quarantines or degrades it and it stays the tail its commit announces.
    pub async fn ingest_remote(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<crate::os_spr::MergeReport, VcsError> {
        let Some((open, edit)) = self.lift_open_transaction().await? else {
            return self.ingest_remote_merge(envelope).await;
        };
        let report = self.ingest_remote_merge(envelope).await;
        self.restore_open_transaction(open, edit).await?;
        report
    }

    /// 🕸️ Feeds a remote {@link MutationEnvelope} through the causal DAG'''

APPLY_GUARD_OLD = '''    async fn apply_command(&mut self, mutations: Vec<Mutation>, description: Option<String>, lane: HistoryLane, transaction: Option<protocol::TransactionRef>) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
'''
APPLY_GUARD_NEW = APPLY_GUARD_OLD + '''        if let Some(open) = self.envelope.open_transaction.as_ref() {
            let transaction_id = open.transaction.id.clone();
            retire_operations::<P, Mutation>(mutations);
            return Err(VcsError::TransactionOpen { transaction_id });
        }
'''

REPLACEMENTS = [
    (
        "envelope field",
        "    pub history_shape: crate::os_spr::HistoryShape,\n}\n\n/// 📸️ One immutable envelope observation",
        "    pub history_shape: crate::os_spr::HistoryShape,\n"
        "    /// 🎐️ The tool transaction whose edit is open in `vcs.edits` ([`ArtifactCommand::AppendTransaction`]). The store folds\n"
        "    /// that edit as its applied tail, but neither the shared log ([`ArtifactStore::event_log`]) nor a persisted form\n"
        "    /// ([`print_document_spr`], [`print_document_text`]) holds it before its commit, so an abort or a lost session leaves\n"
        "    /// no trace.\n"
        "    pub open_transaction: Option<OpenToolTransaction>,\n}\n\n/// 📸️ One immutable envelope observation",
        1,
    ),
    (
        "committed-edit predicate",
        "impl<P, Mutation> ArtifactEnvelopeOwners<P, Mutation> {\n    /// 📸️ Captures read roots before any serializer can cross a shared group decision.\n",
        "impl<P, Mutation> ArtifactEnvelopeOwners<P, Mutation> {\n"
        "    /// 🖇️ Whether `edit_id` is committed history: every edit but the open tool transaction's, which the shared log and\n"
        "    /// every persisted form leave out until its commit.\n"
        "    pub fn holds_committed_edit(&self, edit_id: &str) -> bool {\n"
        "        self.open_transaction.as_ref().is_none_or(|open| open.edit_id != edit_id)\n"
        "    }\n\n"
        "    /// 📸️ Captures read roots before any serializer can cross a shared group decision.\n",
        1,
    ),
    ("envelope destructures", "transitions, history_shape: _ }", "transitions, history_shape: _, open_transaction: _ }", 4),
    (
        "store field",
        "    authoring_verb: Option<String>,\n    /// 🎞️ The tool transaction whose open edit this store's `AppendTransaction`s grow, until its commit or abort.\n    open_transaction: Option<OpenToolTransaction>,\n",
        "    authoring_verb: Option<String>,\n",
        1,
    ),
    ("accessor", "        self.open_transaction.as_ref()\n    }", "        self.envelope.open_transaction.as_ref()\n    }", 1),
    ("dispatch gate", "        if let Some(open) = self.open_transaction.as_ref() {", "        if let Some(open) = self.envelope.open_transaction.as_ref() {", 1),
    (
        "event log",
        "for edit in self.envelope.vcs.edits.iter().filter(|edit| self.open_transaction.as_ref().is_none_or(|open| open.edit_id != edit.id)) {",
        "for edit in self.envelope.vcs.edits.iter().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) {",
        1,
    ),
    (
        "member tail payload",
        "        let Some(edit) = self.envelope.vcs.edits.last() else { return Ok(Vec::new()) };\n        let document_id = ArtifactId(self.envelope.id.clone());",
        "        let Some(edit) = self.envelope.vcs.edits.last().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) else { return Ok(Vec::new()) };\n        let document_id = ArtifactId(self.envelope.id.clone());",
        1,
    ),
    (
        "ops log edits",
        "    for edit in &envelope.vcs.edits {\n        ops.push_str(&print_edit_lines(edit).await?);\n    }",
        "    for edit in envelope.vcs.edits.iter().filter(|edit| envelope.holds_committed_edit(&edit.id)) {\n        ops.push_str(&print_edit_lines(edit).await?);\n    }",
        1,
    ),
    (
        "ops log messages",
        "    for entry in &envelope.edit_messages {\n        if !envelope.vcs.edits.iter().any(|edit| edit.id == entry.edit_id) {",
        "    for entry in envelope.edit_messages.iter().filter(|entry| envelope.holds_committed_edit(&entry.edit_id)) {\n        if !envelope.vcs.edits.iter().any(|edit| edit.id == entry.edit_id) {",
        1,
    ),
    (
        "spr messages",
        "    for entry in &envelope.edit_messages {\n        if message_ledger.insert(entry.edit_id.as_str(), entry.messages.as_slice()).is_some() {",
        "    for entry in envelope.edit_messages.iter().filter(|entry| envelope.holds_committed_edit(&entry.edit_id)) {\n        if message_ledger.insert(entry.edit_id.as_str(), entry.messages.as_slice()).is_some() {",
        1,
    ),
    (
        "spr edits",
        "    for edit in &envelope.vcs.edits {\n        edits.push(history_edit_from_edit::<Mutation>(",
        "    for edit in envelope.vcs.edits.iter().filter(|edit| envelope.holds_committed_edit(&edit.id)) {\n        edits.push(history_edit_from_edit::<Mutation>(",
        1,
    ),
    (
        "ingest merge rename",
        "    pub async fn ingest_remote(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<crate::os_spr::MergeReport, VcsError> {\n        self.ensure_durable_group_idle()?;",
        "    async fn ingest_remote_merge(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<crate::os_spr::MergeReport, VcsError> {\n        self.ensure_durable_group_idle()?;",
        1,
    ),
    ("ingest wrapper", "    /// 🕸️ Feeds a remote {@link MutationEnvelope} through the causal DAG", INGEST_WRAPPER, 1),
    ("apply guard", APPLY_GUARD_OLD, APPLY_GUARD_NEW, 1),
    (
        "batch fields",
        "    outbound: bool,\n    announce_from: usize,\n}",
        "    outbound: bool,\n    announce_from: usize,\n    announce_items: usize,\n    transaction_open: bool,\n}",
        1,
    ),
    (
        "batch setter",
        "    pub fn transaction(&self) -> Option<&protocol::TransactionRef> {\n        self.transaction.as_ref()\n    }\n",
        "    pub fn transaction(&self) -> Option<&protocol::TransactionRef> {\n        self.transaction.as_ref()\n    }\n\n"
        "    /// 🫙️ Keeps this gesture's tool transaction open after its operations land: they join the transaction's open edit (the\n"
        "    /// first streamed gesture opens it) and nothing is announced until a closing gesture or `CommitTransaction` commits\n"
        "    /// it. A closing gesture (the default) that carries the open transaction's ref appends to its edit and commits it.\n"
        "    pub fn set_transaction_open(&mut self, open: bool) {\n"
        "        self.transaction_open = open;\n"
        "    }\n",
        1,
    ),
    ("batch init", "            outbound,\n            announce_from: 0,\n        })", "            outbound,\n            announce_from: 0,\n            announce_items: 0,\n            transaction_open: false,\n        })", 1),
    (
        "batch admission",
        '        if self.durable_group_root.is_some() {\n            return Err(reject("batched publication cannot interleave with an unresolved durable group decision".into(), source));\n        }\n        if self.generation != expected_generation',
        '        if self.durable_group_root.is_some() {\n            return Err(reject("batched publication cannot interleave with an unresolved durable group decision".into(), source));\n        }\n'
        "        if let Some(open) = self.envelope.open_transaction.as_ref().filter(|open| transaction.as_ref().is_none_or(|transaction| transaction.id != open.transaction.id)) {\n"
        '            return Err(reject(format!("batched publication cannot interleave with the open tool transaction {}", open.transaction.id), source));\n'
        "        }\n"
        "        if self.generation != expected_generation",
        1,
    ),
    (
        "batch amend target",
        "    /// 🎥️ Tail uncommitted document edit that a coalesced batch may absorb into, matching `amend_command`.\n    fn batch_amend_target(&self, key: Option<&str>) -> Option<String> {\n",
        "    /// 🎥️ The edit a batch appends to instead of minting a ledger slot: the open edit of the tool transaction it carries,\n"
        "    /// else the tail uncommitted document edit its coalesce key matches, as `amend_command` does.\n"
        "    fn batch_amend_target(&self, key: Option<&str>, transaction: Option<&protocol::TransactionRef>) -> Option<String> {\n"
        "        if let Some(transaction) = transaction {\n"
        "            return self.envelope.open_transaction.as_ref().filter(|open| open.transaction.id == transaction.id && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.clone());\n"
        "        }\n",
        1,
    ),
    ("batch amend calls", "self.batch_amend_target(publication.coalesce_key.as_deref())", "self.batch_amend_target(publication.coalesce_key.as_deref(), publication.transaction.as_ref())", 2),
    (
        "batch amend publish",
        "                    if let Some(existing) = self.envelope.vcs.edits.iter_mut().find(|candidate| candidate.id == edit_id) {\n"
        "                        publication.announce_from = existing.forwards.len();\n"
        "                        existing.forwards.extend(edit.forwards);\n"
        "                        existing.inverse.extend(edit.inverse);\n"
        "                        existing.mutation_meta.extend(edit.mutation_meta);\n"
        "                        existing.finished_at = edit.finished_at;\n"
        "                    } else {",
        "                    let closes = publication.transaction.is_some() && !publication.transaction_open;\n"
        "                    if let Some(existing) = self.envelope.vcs.edits.iter_mut().find(|candidate| candidate.id == edit_id) {\n"
        "                        publication.announce_from = if closes { 0 } else { existing.forwards.len() };\n"
        "                        existing.forwards.extend(edit.forwards);\n"
        "                        existing.inverse.extend(edit.inverse);\n"
        "                        existing.mutation_meta.extend(edit.mutation_meta);\n"
        "                        existing.finished_at = if closes { Some(now_iso()) } else { edit.finished_at };\n"
        "                        publication.announce_items = existing.forwards.len() - publication.announce_from;\n"
        "                    } else {",
        1,
    ),
    (
        "batch amend close",
        "                    self.replace_pending_report_retained(PendingCommandReport::default())?;\n"
        "                    self.replace_local_actor_retained(local_actor)?;\n"
        "                    self.replace_redo_edit_ids_retained(Vec::new())?;\n",
        "                    if closes {\n"
        "                        self.envelope.open_transaction = None;\n"
        "                    } else if publication.transaction_open {\n"
        "                        publication.outbound = false;\n"
        "                    }\n"
        "                    self.replace_pending_report_retained(PendingCommandReport::default())?;\n"
        "                    self.replace_local_actor_retained(local_actor)?;\n"
        "                    self.replace_redo_edit_ids_retained(Vec::new())?;\n",
        1,
    ),
    (
        "batch new edit announce",
        "                publication.announce_from = 0;\n                let post = post.ok_or_else(",
        "                publication.announce_from = 0;\n                publication.announce_items = publication.admitted_items;\n                let post = post.ok_or_else(",
        1,
    ),
    (
        "batch new edit opens",
        "                self.insert_reserved_edit_history(reservation, *edit)?;\n",
        "                let opened = publication.transaction.clone().filter(|_| publication.transaction_open).map(|transaction| OpenToolTransaction { transaction, edit_id: edit.id.clone() });\n"
        "                self.insert_reserved_edit_history(reservation, *edit)?;\n"
        "                if opened.is_some() {\n"
        "                    publication.outbound = false;\n"
        "                    self.envelope.open_transaction = opened;\n"
        "                }\n",
        1,
    ),
    (
        "finalize guard",
        "        self.ensure_durable_group_idle()?;\n        if finished.generation != self.generation || finished.revision != self.content_revision {",
        "        self.ensure_durable_group_idle()?;\n        if let Some(open) = self.envelope.open_transaction.as_ref() {\n            return Err(VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() });\n        }\n        if finished.generation != self.generation || finished.revision != self.content_revision {",
        1,
    ),
    ("batch flush", "self.flush_apply_outbound(publication.announce_from, publication.admitted_items).await?;", "self.flush_apply_outbound(publication.announce_from, publication.announce_items).await?;", 1),
]


def apply(text: str, label: str, old: str, new: str, expected: int) -> str:
    count = text.count(old)
    if count != expected:
        sys.exit(f"[{label}] anchor matched {count}x, expected {expected}")
    return text.replace(old, new)


def main() -> None:
    store = STORE.read_text()
    for label, old, new, expected in REPLACEMENTS:
        store = apply(store, label, old, new, expected)
    store, inits = re.subn(r"(\n\s*authoring_verb: None,)\n\s*open_transaction: None,", r"\1", store)
    if inits != 2:
        sys.exit(f"[store ctor] matched {inits}x, expected 2")
    store, literals = re.subn(r"(\n(\s*)history_shape: crate::os_spr::HistoryShape::Document,)", r"\1\n\2open_transaction: None,", store)
    if literals != 4:
        sys.exit(f"[envelope literals] matched {literals}x, expected 4")
    start = store.index("    //#region 🔖️ToolTransactions\n")
    end = store.index("    //#endregion 🔖️ToolTransactions\n") + len("    //#endregion 🔖️ToolTransactions\n")
    store = store[:start] + TOOL_TRANSACTIONS + store[end:]
    unit = UNIT.read_text()
    unit, unit_literals = re.subn(r"(\n(\s*)history_shape: crate::os_spr::HistoryShape::Document,)", r"\1\n\2open_transaction: None,", unit)
    if unit_literals != 1:
        sys.exit(f"[unit literal] matched {unit_literals}x, expected 1")
    host = HOST.read_text()
    host, host_literals = re.subn(r"(\n(\s*)history_shape: protocol::HistoryShape::Document,)", r"\1\n\2open_transaction: None,", host)
    if host_literals != 2:
        sys.exit(f"[host literals] matched {host_literals}x, expected 2")
    if "--write" in sys.argv:
        STORE.write_text(store)
        UNIT.write_text(unit)
        HOST.write_text(host)
        print("written")
    else:
        print("dry run ok")


main()
