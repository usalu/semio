"""🎞️ W1-G session 2: design §15 runtime route in the plugin — `Emit` streamed / aborted transactions, the refined
`tool_transaction_shape_fault`, both dispatch routes (unmigrated `dispatch_emit_inner`, migrated batched publication), typed
publication refusals, and the time-travel busy guard. One atomic, count-asserted rewrite per file.
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
TIME_TRAVEL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs"

EMIT_FIELD_OLD = """        /// 🛠️ The committed `ToolTransaction` that authored `artifact_mutations`: stamped on every published op as
        /// `MutationMeta.transaction`, so history lists the gesture as one row and edits its yielded mutations.
        /// `None` outside a tool transaction; refused together with a `coalesce_key` or `child_emits` (one commit is
        /// one plain edit). Build it with [`Emit::commit_transaction`].
        pub transaction: Option<protocol::TransactionRef>,
        pub effects: Vec<Effect>,
"""
EMIT_FIELD_NEW = """        /// 🛠️ The `ToolTransaction` that authored `artifact_mutations`: stamped on every published op as
        /// `MutationMeta.transaction`, so history lists the gesture as one row and edits its yielded mutations.
        /// `None` outside a tool transaction; refused together with a `coalesce_key`, and streamed or aborted only on the
        /// app's own document ([`Self::transaction_phase`]). Build it with [`Emit::commit_transaction`],
        /// [`Emit::stream_transaction`] or [`Emit::abort_transaction`].
        pub transaction: Option<protocol::TransactionRef>,
        /// 🎞️ How `artifact_mutations` join `transaction` (design §15): committed as its one edit, streamed into its open
        /// edit, or its open edit aborted.
        pub transaction_phase: TransactionPhase,
        pub effects: Vec<Effect>,
"""

PHASE_ENUM = """    /// 🎞️ How an emission's artifact mutations join its tool transaction (design §15, transaction-scoped amend). `Commit`
    /// records them as the transaction's one edit — closing the transaction's open edit when earlier ticks streamed into it.
    /// `Stream` appends them to the transaction's open edit (the first tick opens it) and keeps it open: the store folds it
    /// as its applied tail, but nothing is announced or persisted before the commit. `Abort` reverts the open edit with zero
    /// trace.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum TransactionPhase {
        #[default]
        Commit,
        Stream,
        Abort,
    }

    impl<Mutation, ConfigMutation, DraftMutation> Default for Emit<Mutation, ConfigMutation, DraftMutation> {
"""

COMMIT_OLD = """        /// 🛠️ A committed `ToolTransaction` (`ToolMachineRunner`'s `ToolStep::Committed(transaction, mutations)`)
        /// published as ONE document edit whose every op carries `transaction`; with no description the history row
        /// is labelled from the mutations' own `MutationKind` labels. An empty transaction is an empty emission.
        pub fn commit_transaction(transaction: protocol::TransactionRef, artifact_mutations: Vec<Mutation>) -> Self {
            let transaction = (!artifact_mutations.is_empty()).then_some(transaction);
            Self { artifact_mutations, transaction, ..Default::default() }
        }
"""
COMMIT_NEW = """        /// 🛠️ A committed `ToolTransaction` (`ToolMachineRunner`'s `ToolStep::Committed(transaction, mutations)`)
        /// published as ONE document edit whose every op carries `transaction`; with no description the history row
        /// is labelled from the mutations' own `MutationKind` labels. When earlier ticks streamed into the transaction
        /// ([`Self::stream_transaction`]) the mutations join its open edit and the commit closes it; with nothing open and
        /// nothing to record it is an empty emission.
        pub fn commit_transaction(transaction: protocol::TransactionRef, artifact_mutations: Vec<Mutation>) -> Self {
            Self { artifact_mutations, transaction: Some(transaction), ..Default::default() }
        }

        /// 🌊️ One tick of a streamed `ToolTransaction` (design §15): `artifact_mutations` join the transaction's open edit
        /// (the first tick opens it), every op stamped with `transaction`. The document shows them at once, but nothing is
        /// announced or persisted until [`Self::commit_transaction`] with the same ref closes the edit; [`Self::abort_transaction`]
        /// reverts it with zero trace. For tools whose yield cannot wait for the commit (streamed imports).
        pub fn stream_transaction(transaction: protocol::TransactionRef, artifact_mutations: Vec<Mutation>) -> Self {
            Self { artifact_mutations, transaction: Some(transaction), transaction_phase: TransactionPhase::Stream, ..Default::default() }
        }

        /// 🧨️ Reverts the open edit of the streamed `ToolTransaction` `transaction` with zero trace: no edit, no history row,
        /// nothing announced. With nothing open it is an empty emission.
        pub fn abort_transaction(transaction: protocol::TransactionRef) -> Self {
            Self { transaction: Some(transaction), transaction_phase: TransactionPhase::Abort, ..Default::default() }
        }
"""

SHAPE_OLD = """    /// 🛠️ A committed tool transaction is one plain edit per touched member (the parent and every owned child it
    /// carries, all stamped with the same ref — design §12): a transaction riding a coalesced amend is refused instead
    /// of silently losing its stamp.
    fn tool_transaction_shape_fault(verb: &str, transaction: Option<&protocol::TransactionRef>, coalesced: bool) -> Option<Fault> {
        let transaction = transaction.filter(|_| coalesced)?;
        Some(Fault::new(FaultOrigin::Framework, FaultCode::new("toolTransaction.shape"), format!("verb {verb:?} published tool transaction {:?} with a coalesced amend; a committed transaction is one plain edit per member", transaction.id)))
    }
"""
SHAPE_NEW = """    /// 🛠️ A committed tool transaction is one plain edit per touched member (the parent and every owned child it carries,
    /// all stamped with the same ref — design §12); a streamed one is one open edit of the app's own document that its ticks
    /// grow (design §15). Refused instead of silently losing a stamp or a trace: a transaction riding a coalesced amend, a
    /// stream or abort naming no transaction, a stream or abort across owned children, a stream carrying operations with
    /// foreign steps, and an abort carrying operations.
    #[allow(clippy::too_many_arguments)]
    fn tool_transaction_shape_fault(verb: &str, transaction: Option<&protocol::TransactionRef>, phase: TransactionPhase, coalesced: bool, children: bool, carried: bool, foreign: bool) -> Option<Fault> {
        let refuse = |detail: String| Some(Fault::new(FaultOrigin::Framework, FaultCode::new("toolTransaction.shape"), detail));
        let streamed = matches!(phase, TransactionPhase::Stream | TransactionPhase::Abort);
        match transaction {
            Some(transaction) if coalesced => refuse(format!("verb {verb:?} published tool transaction {:?} with a coalesced amend; a transaction is one plain edit per member or one open edit its ticks grow", transaction.id)),
            None if streamed => refuse(format!("verb {verb:?} streamed or aborted no tool transaction")),
            Some(transaction) if streamed && children => refuse(format!("verb {verb:?} streamed or aborted tool transaction {:?} across owned children; a streamed transaction grows the app's own document", transaction.id)),
            Some(transaction) if phase == TransactionPhase::Stream && foreign => refuse(format!("verb {verb:?} streamed operations with foreign steps into tool transaction {:?}; a streamed transaction grows the app's own document", transaction.id)),
            Some(transaction) if phase == TransactionPhase::Abort && carried => refuse(format!("verb {verb:?} aborted tool transaction {:?} with mutations; an abort carries none", transaction.id)),
            _ => None,
        }
    }

    /// 🎞️ The document-store commands an emission's artifact lane dispatches (design §15): a streamed tick appends to its
    /// transaction's open edit (opening it), a commit closes the open edit of its transaction (appending its last operations
    /// first) or, with none open, records its operations as one edit carrying the ref, an abort reverts the open edit, and an
    /// emission outside a transaction applies or coalesces. An empty list publishes nothing.
    fn artifact_lane_commands<M>(mutations: Vec<M>, description: Option<String>, coalesce_key: Option<String>, transaction: Option<protocol::TransactionRef>, phase: TransactionPhase, open: Option<&store::OpenToolTransaction>) -> Vec<ArtifactCommand<M>> {
        let open = open.zip(transaction.as_ref()).is_some_and(|(open, transaction)| open.transaction.id == transaction.id);
        let append = |mutations: Vec<M>, transaction: protocol::TransactionRef| (!mutations.is_empty()).then(|| ArtifactCommand::AppendTransaction { mutations, transaction });
        match (transaction, phase) {
            (Some(transaction), TransactionPhase::Stream) => append(mutations, transaction).into_iter().collect(),
            (Some(transaction), TransactionPhase::Commit) if open => {
                let transaction_id = transaction.id.clone();
                append(mutations, transaction).into_iter().chain([ArtifactCommand::CommitTransaction { transaction_id }]).collect()
            }
            (Some(transaction), TransactionPhase::Abort) => open.then(|| ArtifactCommand::AbortTransaction { transaction_id: transaction.id }).into_iter().collect(),
            (_, _) if mutations.is_empty() => Vec::new(),
            (transaction, _) => vec![match coalesce_key {
                Some(key) => ArtifactCommand::AmendLast { mutations, coalesce_key: Some(key) },
                None => ArtifactCommand::Apply { mutations, description, transaction },
            }],
        }
    }

    /// 🧯️ A batched store publication's refusal: the history and tool-transaction refusals keep their own fault codes
    /// (`history.full`, `toolTransaction.open`, `toolTransaction.unknown` — the shells show them as notices); every other one is
    /// the SDK's publication fault.
    fn store_publication_fault(error: vcs::VcsError) -> Fault {
        match error {
            vcs::VcsError::TransactionOpen { .. } | vcs::VcsError::UnknownTransaction(_) | vcs::VcsError::HistoryFull { .. } => error.into_fault(),
            error => plugin_sdk_fault(error.to_string()),
        }
    }
"""

DESTRUCTURE_OLD = """            let Emit { artifact_mutations, config_mutations, window_config_mutations, draft_mutations, description, coalesce_key, transaction, effects, extension_invocations, events, ui_scope, child_emits, interaction_writes, tasks } = emit;
            if let Some(fault) = tool_transaction_shape_fault(verb, transaction.as_ref(), coalesce_key.is_some()) {
"""
DESTRUCTURE_NEW = """            let Emit { artifact_mutations, config_mutations, window_config_mutations, draft_mutations, description, coalesce_key, transaction, transaction_phase, effects, extension_invocations, events, ui_scope, child_emits, interaction_writes, tasks } = emit;
            if let Some(fault) = tool_transaction_shape_fault(verb, transaction.as_ref(), transaction_phase, coalesce_key.is_some(), !child_emits.is_empty(), !artifact_mutations.is_empty(), artifact_mutations.iter().any(Mutation::may_emit_foreign_steps)) {
"""

TAIL_OLD = """            if artifact_mutations.is_empty() {
                let kind = match self.invocation_kind {
                    Some(kind) => kind,
                    None => match self.registry.get(verb).map(|definition| definition.kind) {
                        Some(kind) => kind,
                        None => match self.registry.get_command(verb) {
                            Some(command) => command.kind,
                            None => ActionKind::View,
                        },
                    },
                };
                self.apply_interaction_writes(&interaction_writes, meta).await?;
                if !(published_window_config && config_edit_id.is_none() && matches!(kind, ActionKind::View)) {
                    let label = self.authored_row_label(verb, description.as_deref());
                    self.record_command(verb, kind, label, None, config_edit_id, None);
                }
                return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
            }
            self.store.set_local_actor_id(Some(meta.actor.clone())).map_err(|error| error.into_fault())?;
            let before_edit_id = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
            let (before_forwards_len, before_backwards_len) = self.store.edit_mutations().map_or((0, 0), |(f, b, _)| (f.len(), b.len()));
            let log_label = self.authored_row_label(verb, description.as_deref());
            let vcs_command = match coalesce_key {
                Some(key) => ArtifactCommand::AmendLast { mutations: artifact_mutations, coalesce_key: Some(key) },
                None => ArtifactCommand::Apply { mutations: artifact_mutations, description, transaction },
            };
            self.store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.store.dispatch(vcs_command).await;
            self.store.set_authoring_verb(None);
            match dispatched {
                Ok(receipt) => self.record_dispatch_receipt(receipt),
                Err(vcs::VcsError::Rejected { policy, messages }) => return Err(self.record_rejected_dispatch(policy, messages).await),
                Err(error) => return Err(error.into_fault()),
            }
            .await;
            self.revalidate_interaction_state_after_document_change(meta).await?;
            self.apply_interaction_writes(&interaction_writes, meta).await?;
            let amended_same_edit"""
TAIL_NEW = """            let in_transaction = transaction.is_some();
            let log_label = self.authored_row_label(verb, description.as_deref());
            let vcs_commands = artifact_lane_commands(artifact_mutations, description, coalesce_key, transaction, transaction_phase, self.store.open_transaction());
            if vcs_commands.is_empty() {
                let kind = match self.invocation_kind {
                    Some(kind) => kind,
                    None => match self.registry.get(verb).map(|definition| definition.kind) {
                        Some(kind) => kind,
                        None => match self.registry.get_command(verb) {
                            Some(command) => command.kind,
                            None => ActionKind::View,
                        },
                    },
                };
                self.apply_interaction_writes(&interaction_writes, meta).await?;
                if !in_transaction && !(published_window_config && config_edit_id.is_none() && matches!(kind, ActionKind::View)) {
                    self.record_command(verb, kind, log_label, None, config_edit_id, None);
                }
                return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
            }
            let aborting = matches!(vcs_commands.as_slice(), [ArtifactCommand::AbortTransaction { .. }]);
            self.store.set_local_actor_id(Some(meta.actor.clone())).map_err(|error| error.into_fault())?;
            let before_edit_id = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
            let (before_forwards_len, before_backwards_len) = self.store.edit_mutations().map_or((0, 0), |(f, b, _)| (f.len(), b.len()));
            for vcs_command in vcs_commands {
                self.store.set_authoring_verb(Some(verb.to_string()));
                let dispatched = self.store.dispatch(vcs_command).await;
                self.store.set_authoring_verb(None);
                match dispatched {
                    Ok(receipt) => self.record_dispatch_receipt(receipt),
                    Err(vcs::VcsError::Rejected { policy, messages }) => return Err(self.record_rejected_dispatch(policy, messages).await),
                    Err(error) => return Err(error.into_fault()),
                }
                .await;
            }
            self.revalidate_interaction_state_after_document_change(meta).await?;
            self.apply_interaction_writes(&interaction_writes, meta).await?;
            if aborting {
                self.retire_displaced_document_rows();
                return Ok(Self::empty_result(verb, meta, effects, events, ui_scope).await);
            }
            let amended_same_edit"""

INNER_END_OLD = """            let tail_offset = if amended_same_edit { (before_forwards_len, before_backwards_len) } else { (0, 0) };
            Ok(self.result_from_last_edit(verb, meta, effects, events, ui_scope, tail_offset).await)
        }
"""
INNER_END_NEW = INNER_END_OLD + """
        /// 🎗️ The store half of a migrated emission that closes a streamed tool transaction without new operations (design
        /// §15): an abort reverts the transaction's open edit and retires its history row, a commit with no last operations
        /// closes the edit. Either is consumed, so the publication continues with the emission's other lanes; a closing commit
        /// with operations and every stream are left to the batched route.
        async fn close_streamed_transaction_unit(&mut self, verb: &str, emit: &mut Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>) -> Result<(), Fault> {
            let Some(transaction) = emit.transaction.as_ref() else { return Ok(()) };
            let abort = emit.transaction_phase == TransactionPhase::Abort;
            if !abort && (emit.transaction_phase == TransactionPhase::Stream || !emit.artifact_mutations.is_empty()) {
                return Ok(());
            }
            let open = self.store.open_transaction().is_some_and(|open| open.transaction.id == transaction.id);
            let transaction_id = transaction.id.clone();
            emit.transaction = None;
            emit.transaction_phase = TransactionPhase::Commit;
            if !open {
                return Ok(());
            }
            let command = if abort { ArtifactCommand::AbortTransaction { transaction_id } } else { ArtifactCommand::CommitTransaction { transaction_id } };
            self.store.set_authoring_verb(Some(verb.to_string()));
            let dispatched = self.store.dispatch(command).await;
            self.store.set_authoring_verb(None);
            dispatched.map_err(|error| error.into_fault())?;
            if abort {
                self.retire_displaced_document_rows();
            }
            Ok(())
        }

        /// 🚧️ The typed refusal of a batched artifact publication while a tool transaction it does not carry is open on the
        /// document store (the store refuses its admission too, untyped).
        fn open_transaction_admission_fault(&self, transaction: Option<&protocol::TransactionRef>) -> Option<Fault> {
            let open = self.store.open_transaction()?;
            transaction.is_none_or(|transaction| transaction.id != open.transaction.id).then(|| vcs::VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() }.into_fault())
        }
"""

MIGRATED_OLD = """                    if let Some(fault) = tool_transaction_shape_fault(&mounted.verb, emit.transaction.as_ref(), emit.coalesce_key.is_some()) {
                        return Err(fault);
                    }
                    if (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty()) && self.time_travel.freezes_local_emits() {
                        return Err(time_travel_frozen_fault(&mounted.verb));
                    }
                    if emit.child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                        let mutations = std::mem::take(&mut emit.artifact_mutations);
"""
MIGRATED_NEW = """                    if let Some(fault) = tool_transaction_shape_fault(&mounted.verb, emit.transaction.as_ref(), emit.transaction_phase, emit.coalesce_key.is_some(), !emit.child_emits.is_empty(), !emit.artifact_mutations.is_empty(), emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps)) {
                        return Err(fault);
                    }
                    if (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty()) && self.time_travel.freezes_local_emits() {
                        return Err(time_travel_frozen_fault(&mounted.verb));
                    }
                    self.close_streamed_transaction_unit(&mounted.verb, emit).await?;
                    if emit.child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                        if let Some(fault) = self.open_transaction_admission_fault(emit.transaction.as_ref()) {
                            return Err(fault);
                        }
                        let streams = emit.transaction_phase == TransactionPhase::Stream;
                        let mutations = std::mem::take(&mut emit.artifact_mutations);
"""

PUBLICATION_OLD = """                                publication.set_coalesce_key(emit.coalesce_key.take());
                                publication.set_verb(Some(mounted.verb.clone()));
                                mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Artifact(publication));
"""
PUBLICATION_NEW = """                                publication.set_coalesce_key(emit.coalesce_key.take());
                                publication.set_transaction_open(streams);
                                publication.set_verb(Some(mounted.verb.clone()));
                                mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Artifact(publication));
"""

ADVANCE_OLD = "PendingArtifactStorePublication::Artifact(publication) => self.store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,"
ADVANCE_NEW = "PendingArtifactStorePublication::Artifact(publication) => self.store.advance_apply_batch(publication, grant).map_err(store_publication_fault)?,"

PLUGIN_EDITS = [
    ("emit field", EMIT_FIELD_OLD, EMIT_FIELD_NEW),
    ("emit default", "                transaction: None,\n                effects: Vec::new(),\n                extension_invocations: Vec::new(),\n", "                transaction: None,\n                transaction_phase: TransactionPhase::Commit,\n                effects: Vec::new(),\n                extension_invocations: Vec::new(),\n"),
    ("phase enum", "    impl<Mutation, ConfigMutation, DraftMutation> Default for Emit<Mutation, ConfigMutation, DraftMutation> {\n", PHASE_ENUM),
    ("constructors", COMMIT_OLD, COMMIT_NEW),
    ("shape rule", SHAPE_OLD, SHAPE_NEW),
    ("destructure", DESTRUCTURE_OLD, DESTRUCTURE_NEW),
    ("artifact lane", TAIL_OLD, TAIL_NEW),
    ("route helpers", INNER_END_OLD, INNER_END_NEW),
    ("migrated route", MIGRATED_OLD, MIGRATED_NEW),
    ("migrated publication", PUBLICATION_OLD, PUBLICATION_NEW),
    ("advance fault", ADVANCE_OLD, ADVANCE_NEW),
]

TIME_TRAVEL_EDITS = [
    ("busy guard", "        if self.tool_runs.holds_mutating_run() || self.pending_transaction.is_some() {\n            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));\n        }\n        let store = time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE)",
     "        if self.tool_runs.holds_mutating_run() || self.pending_transaction.is_some() || self.store.open_transaction().is_some() {\n            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));\n        }\n        let store = time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE)"),
]


def rewrite(path: pathlib.Path, edits: list) -> str:
    text = path.read_text()
    for label, old, new in edits:
        count = text.count(old)
        if count != 1:
            sys.exit(f"[{path.name} {label}] anchor matched {count}x, expected 1")
        text = text.replace(old, new)
    return text


def main() -> None:
    plugin = rewrite(PLUGIN, PLUGIN_EDITS)
    time_travel = rewrite(TIME_TRAVEL, TIME_TRAVEL_EDITS)
    if "--write" in sys.argv:
        PLUGIN.write_text(plugin)
        TIME_TRAVEL.write_text(time_travel)
        print("written")
    else:
        print("dry run ok")


main()
