"""🧮️ S3-W2A W2A-2 (design §20.13, L4): config-lane edits are never history rows — no row, no mutation, no undo/redo or revert
stepping over them. The command log loses every config field; config-only dispatches record nothing; the config revert lane
goes away. Plugin root + time-travel module (the crate's own tests follow in `🧪️s3-w2a-l4-tests.py`)."""
import pathlib

P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
ROOT = P / "🦀️.rs"
TT = P / "⏪️time-travel/🦀️.rs"

root = [
    # A. CommandLogAppend
    ("        edit_id: Option<String>,\n        config_edit_id: Option<String>,\n        transition_id: Option<String>,\n        timestamp: Option<String>,\n        inverse: Option<InverseAction>,\n    }\n",
     "        edit_id: Option<String>,\n        transition_id: Option<String>,\n        timestamp: Option<String>,\n        inverse: Option<InverseAction>,\n    }\n"),
    # B. CommandLogEntry config ids + docs
    ("""        /// 🔗️ Set iff this command created/amended a DOCUMENT VCS edit — `None` for pure cursor
        /// motion (undo/redo/revert) and config-only dispatches that never touch the document store.
        pub edit_id: Option<String>,
        /// 🧮️ B1: the CONFIG-store twin of `edit_id` — every CONFIG edit this command created
        /// (the former "View"-kind self-computed `InverseAction` path: a config op carries a real
        /// `inverse`, so reverting it is a real config-store undo-to-position, not a memory replay). A
        /// `Vec` (not a single id) because a folded row may accumulate several distinct config edits — one
        /// per fold tick, since a config-only "View" dispatch is a plain `Apply` (unlike a streamed document
        /// transaction, whose ticks grow one open edit) — `backfill_command_log`
        /// needs every one of them to correctly recognize "already logged". `CommandView::config_edit_id`
        /// exposes just the LATEST for display/revert purposes. A single dispatch may also set `edit_id`
        /// (touching both stores at once); `revertToCommand` prefers `edit_id` when both are present.
        pub config_edit_ids: Vec<String>,
        /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): every CHILD document edit id this
        /// command's composite `dispatch_group` call produced — the `config_edit_ids` precedent
        /// applied to the composition seam. Unlike `config_edit_ids` (one store, possibly several
        /// fold-ticks), this is one entry per touched CHILD member of a single group dispatch (never
        /// folded — a group dispatch always issues a plain `Apply`, see `Emit::child_emits`'s own doc
        /// comment), so it needs no "latest" reduction on the `CommandView` side.
        pub child_edit_ids: Vec<String>,
""",
     """        /// 🔗️ Set iff this command created/amended a DOCUMENT VCS edit — `None` for pure cursor motion (undo/redo/revert).
        /// A config-lane edit is never a history row (design §20.13, L4), so no row carries one and a config-only dispatch
        /// records nothing.
        pub edit_id: Option<String>,
        /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): every CHILD document edit id this command's composite `dispatch_group`
        /// call produced — one entry per touched CHILD member of a single group dispatch (never folded — a group dispatch
        /// always issues a plain `Apply`, see `Emit::child_emits`'s own doc comment).
        pub child_edit_ids: Vec<String>,
"""),
    ("        /// ⏪️ A real inverse for a `🐚️Shell`-kind row with neither `edit_id` nor `config_edit_ids`\n",
     "        /// ⏪️ A real inverse for a `🐚️Shell`-kind row with no `edit_id`\n"),
    # C. CommandView
    ("        pub edit_id: Option<String>,\n        /// 🧮️ The LATEST of `CommandLogEntry::config_edit_ids` — the id `revertToCommand` targets.\n        pub config_edit_id: Option<String>,\n",
     "        pub edit_id: Option<String>,\n"),
    # E. applied predicate
    ("""    /// A row publishes into up to three stores. `undo`/`redo` dispatch against the PARENT document
    /// store alone (`interaction_store`'s own doc says so), so the parent lane is authoritative for
    /// any row that published a parent edit — a multi-lane row (`Artifact` + `Config`, which is what
    /// 💠️lowpoly's `addPrimitive` and 🎥️shooting's `addShot` declare) keeps a live config edit after
    /// its document edit is undone, and answering `document || config || child` left it applied for
    /// ever. A row with NO parent edit at all (🌊️flow's `addWidget`, 🎬️sequence's `addStep`,
    /// 🌀️procedural's `generate`) has nothing else to answer with, and asking only the parent lane
    /// made those read as UNDONE the moment they landed.
    ///
    /// Named rather than inlined so the law can drive all eight lane combinations without an app, a
    /// retained multi-lane factory or a pinned proof roster — the two shapes that could not be
    /// written in this suite (ticket 26/09/18 S11 §3.4b).
    pub fn history_row_applied_v1(has_parent_edit: bool, document_applied: bool, config_applied: bool, child_applied: bool) -> bool {
        if has_parent_edit {
            document_applied
        } else {
            config_applied || child_applied
        }
    }
""",
     """    /// A row publishes into the parent document store and its children; a config-lane edit is never part of a row (design
    /// §20.13, L4). `undo`/`redo` dispatch against the PARENT document store alone (`interaction_store`'s own doc says so), so
    /// the parent lane is authoritative for any row that published a parent edit, and a row with NO parent edit (🌊️flow's
    /// `addWidget`, 🎬️sequence's `addStep` on the child lane) answers with its children.
    ///
    /// Named rather than inlined so the law can drive every lane combination without an app (ticket 26/09/18 S11 §3.4b).
    pub fn history_row_applied(has_parent_edit: bool, document_applied: bool, child_applied: bool) -> bool {
        if has_parent_edit {
            document_applied
        } else {
            child_applied
        }
    }
"""),
    # F. record_typed_operation_lane
    ("""        /// be filed under `apply` by the time the slot retired. A second durable lane of the SAME
        /// operation (a verb writing both the document and the config store) folds its edit into the row
        /// the first lane opened, exactly as `dispatch_emit` carries both ids on one row. A coalesced
        /// gesture amends the edit its first dispatch already logged, so later dispatches append no
        /// second row — and they mark that row dirty, so the admitting completion re-upserts it.
        fn record_typed_operation_lane(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, artifact_lane: bool) {
            if mounted.terminal_fault.is_some() {
                return;
            }
            let edit = if artifact_lane { self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) } else { self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) };
            let Some(edit_id) = edit else { return };
            let logged_seq = self.command_log.iter().find(|entry| if artifact_lane { entry.edit_id.as_deref() == Some(edit_id.as_str()) } else { entry.config_edit_ids.iter().any(|logged| logged == &edit_id) }).map(|entry| entry.seq);
            if let Some(seq) = logged_seq {
                mounted.command_logged = true;
                self.history_dirty_sequences.insert(seq);
                return;
            }
            if mounted.command_logged {
                if let Some(entry) = self.command_log.last_mut().filter(|entry| entry.action_id == mounted.verb) {
                    if artifact_lane {
                        entry.edit_id = Some(edit_id);
                    } else {
                        entry.config_edit_ids.push(edit_id);
                    }
                    self.history_dirty_sequences.insert(entry.seq);
                    self.log_generation += 1;
                }
                return;
            }
            mounted.command_logged = true;
            let kind = self.typed_operation_command_kind(&mounted.verb, artifact_lane);
            let verb = mounted.verb.clone();
            let label = self.verb_label(&verb);
            if artifact_lane {
                self.record_command(&verb, kind, label, Some(edit_id), None, None);
            } else {
                self.record_command(&verb, kind, label, None, Some(edit_id), None);
            }
        }
""",
     """        /// be filed under `apply` by the time the slot retired. Only the document lane opens or amends a row: a config-lane
        /// edit is never a history row (design §20.13, L4). A coalesced gesture amends the edit its first dispatch already
        /// logged, so later dispatches append no second row — and they mark that row dirty, so the admitting completion
        /// re-upserts it.
        fn record_typed_operation_lane(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) {
            if mounted.terminal_fault.is_some() {
                return;
            }
            let Some(edit_id) = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) else { return };
            let logged_seq = self.command_log.iter().find(|entry| entry.edit_id.as_deref() == Some(edit_id.as_str())).map(|entry| entry.seq);
            if let Some(seq) = logged_seq {
                mounted.command_logged = true;
                self.history_dirty_sequences.insert(seq);
                return;
            }
            if mounted.command_logged {
                if let Some(entry) = self.command_log.last_mut().filter(|entry| entry.action_id == mounted.verb) {
                    entry.edit_id = Some(edit_id);
                    self.history_dirty_sequences.insert(entry.seq);
                    self.log_generation += 1;
                }
                return;
            }
            mounted.command_logged = true;
            let kind = self.typed_operation_command_kind(&mounted.verb, true);
            let verb = mounted.verb.clone();
            let label = self.verb_label(&verb);
            self.record_command(&verb, kind, label, Some(edit_id), Vec::new(), None);
        }
"""),
    ("        /// [`Self::backfill_command_log`]'s anonymous `apply` / `configApply` row — losing the verb, its\n",
     "        /// [`Self::backfill_command_log`]'s anonymous `apply` row — losing the verb, its\n"),
    ("            self.record_command(&verb, kind, None, None, None, None);\n        }\n",
     "            self.record_command(&verb, kind, None, None, Vec::new(), None);\n        }\n"),
    # G. push_log_entry
    ("            let CommandLogAppend { action_id, label, kind, edit_id, config_edit_id, transition_id, timestamp, inverse } = append;\n",
     "            let CommandLogAppend { action_id, label, kind, edit_id, transition_id, timestamp, inverse } = append;\n"),
    ("                edit_id,\n                config_edit_ids: config_edit_id.into_iter().collect(),\n                child_edit_ids: Vec::new(),\n",
     "                edit_id,\n                child_edit_ids: Vec::new(),\n"),
    # H. record_command
    ("""        fn record_command(&mut self, action_id: &str, kind: ActionKind, label: Option<LocalizedLabel>, edit_id: Option<String>, config_edit_id: Option<String>, inverse: Option<InverseAction>) {
            if !history_row_is_recorded(kind, edit_id.is_some() || config_edit_id.is_some(), inverse.is_some()) {
                return;
            }
            let label = label.or_else(|| self.verb_label(action_id)).unwrap_or_else(|| LocalizedLabel::data(action_id));
            self.push_log_entry(CommandLogAppend { action_id, label, kind, edit_id, config_edit_id, transition_id: None, timestamp: None, inverse });
""",
     """        fn record_command(&mut self, action_id: &str, kind: ActionKind, label: Option<LocalizedLabel>, edit_id: Option<String>, child_edit_ids: Vec<String>, inverse: Option<InverseAction>) {
            if !history_row_is_recorded(kind, edit_id.is_some() || !child_edit_ids.is_empty(), inverse.is_some()) {
                return;
            }
            let label = label.or_else(|| self.verb_label(action_id)).unwrap_or_else(|| LocalizedLabel::data(action_id));
            self.push_log_entry(CommandLogAppend { action_id, label, kind, edit_id, transition_id: None, timestamp: None, inverse });
            if let Some(last) = self.command_log.last_mut() {
                last.child_edit_ids = child_edit_ids;
            }
"""),
    # I. backfill
    ("label, kind: ActionKind::Mutation, edit_id: Some(edit_id), config_edit_id: None, transition_id: None, timestamp: Some(timestamp), inverse: None });",
     "label, kind: ActionKind::Mutation, edit_id: Some(edit_id), transition_id: None, timestamp: Some(timestamp), inverse: None });"),
    ("label, kind: ActionKind::Mutation, edit_id: None, config_edit_id: None, transition_id: None, timestamp: Some(timestamp), inverse: None });",
     "label, kind: ActionKind::Mutation, edit_id: None, transition_id: None, timestamp: Some(timestamp), inverse: None });"),
    ("label, kind: ActionKind::History, edit_id: None, config_edit_id: None, transition_id: Some(transition_id), timestamp: Some(timestamp), inverse: None }),",
     "label, kind: ActionKind::History, edit_id: None, transition_id: Some(transition_id), timestamp: Some(timestamp), inverse: None }),"),
    ("""            let logged_config: HashSet<&str> = self.command_log.iter().flat_map(|entry| entry.config_edit_ids.iter().map(String::as_str)).collect();
            let mut missing_config: Vec<(String, LocalizedLabel, String)> = Vec::new();
            for edit in self.config_store.envelope().vcs.edits.iter().filter(|edit| !logged_config.contains(edit.id.as_str())) {
                let label = backfilled_edit_label(edit, edit.verb.as_deref().and_then(|verb| self.verb_label(verb)), |_| LocalizedLabel::native("Change settings", "Einstellungen ändern"));
                missing_config.push((edit.id.clone(), label, edit.started_at.clone()));
            }
            for (config_edit_id, label, timestamp) in missing_config {
                self.push_log_entry(CommandLogAppend { action_id: "configApply", label, kind: ActionKind::Mutation, edit_id: None, config_edit_id: Some(config_edit_id), transition_id: None, timestamp: Some(timestamp), inverse: None });
            }
        }
""",
     "        }\n"),
    # J. projection
    ("            let config_applied_ids: HashSet<&str> = self.config_store.applied_edit_ids().iter().map(String::as_str).collect();\n            let config_local_actor = self.config_store.local_actor_id();\n",
     ""),
    ("            let envelope = self.store.envelope();\n            let config_envelope = self.config_store.envelope();\n            let edits_by_id: HashMap<&str, &protocol::Edit<A::Mutation>> = envelope.vcs.edits.iter().map(|edit| (edit.id.as_str(), edit)).collect();\n            let config_edits_by_id: HashMap<&str, &protocol::Edit<A::ConfigMutation>> = config_envelope.vcs.edits.iter().map(|edit| (edit.id.as_str(), edit)).collect();\n",
     "            let envelope = self.store.envelope();\n            let edits_by_id: HashMap<&str, &protocol::Edit<A::Mutation>> = envelope.vcs.edits.iter().map(|edit| (edit.id.as_str(), edit)).collect();\n"),
    ("                        edit_id: None,\n                        config_edit_id: None,\n                        child_edit_ids: Vec::new(),\n                        transition_id: Some(record.transition_id.clone()),\n",
     "                        edit_id: None,\n                        child_edit_ids: Vec::new(),\n                        transition_id: Some(record.transition_id.clone()),\n"),
    ("""                let latest_config_edit_id = entry.config_edit_ids.last().map(String::as_str);
                let config_edit = latest_config_edit_id.and_then(|edit_id| config_edits_by_id.get(edit_id).copied());
                let config_applied = latest_config_edit_id.is_some_and(|edit_id| config_applied_ids.contains(edit_id));
                let child_applied = entry.child_edit_ids.iter().any(|edit_id| child_applied_tails.contains(edit_id));
                let applied = history_row_applied_v1(entry.edit_id.is_some(), document_applied, config_applied, child_applied);
                let revertible = (document_applied && edit.is_some_and(|edit| edit.actor.is_none() || edit.actor.as_deref() == local_actor))
                    || (config_applied && config_edit.is_some_and(|edit| edit.actor.is_none() || edit.actor.as_deref() == config_local_actor))
                    || child_applied
""",
     """                let child_applied = entry.child_edit_ids.iter().any(|edit_id| child_applied_tails.contains(edit_id));
                let applied = history_row_applied(entry.edit_id.is_some(), document_applied, child_applied);
                let revertible = (document_applied && edit.is_some_and(|edit| edit.actor.is_none() || edit.actor.as_deref() == local_actor))
                    || child_applied
"""),
    ("                    edit_id: entry.edit_id.clone(),\n                    config_edit_id: latest_config_edit_id.map(str::to_string),\n                    child_edit_ids: entry.child_edit_ids.clone(),\n",
     "                    edit_id: entry.edit_id.clone(),\n                    child_edit_ids: entry.child_edit_ids.clone(),\n"),
    ("                    || self.command_log.iter().any(|entry| entry.inverse.is_some() && entry.edit_id.is_none() && entry.config_edit_ids.is_empty() && !self.shell_undone.contains(&entry.seq))\n",
     "                    || self.command_log.iter().any(|entry| entry.inverse.is_some() && entry.edit_id.is_none() && !self.shell_undone.contains(&entry.seq))\n"),
    # K. dispatch_typed
    ("""            let mut config_edit_id: Option<String> = None;
            if !config_mutations.is_empty() {
                self.config_store.set_local_actor_id(Some(meta.actor.clone())).map_err(|error| error.into_fault())?;
                let before_config_edit_id = self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
                let config_command = ArtifactCommand::Apply { mutations: config_mutations, description: None, transaction: None };
                self.config_store.set_authoring_verb(Some(verb.to_string()));
                let dispatched = self.config_store.dispatch(config_command).await;
                self.config_store.set_authoring_verb(None);
                dispatched.map_err(|error| error.into_fault())?;
                let amended_same_config_edit = before_config_edit_id.is_some() && self.config_store.envelope().vcs.edits.last().map(|edit| &edit.id) == before_config_edit_id.as_ref();
                config_edit_id = if amended_same_config_edit { before_config_edit_id } else { self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) };
            }
""",
     """            let config_edited = !config_mutations.is_empty();
            if config_edited {
                self.config_store.set_local_actor_id(Some(meta.actor.clone())).map_err(|error| error.into_fault())?;
                let config_command = ArtifactCommand::Apply { mutations: config_mutations, description: None, transaction: None };
                self.config_store.set_authoring_verb(Some(verb.to_string()));
                let dispatched = self.config_store.dispatch(config_command).await;
                self.config_store.set_authoring_verb(None);
                dispatched.map_err(|error| error.into_fault())?;
            }
"""),
    ("                let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, None, effects, events, ui_scope, config_edit_id, meta, None, transaction).await?;\n",
     "                let result = self.dispatch_emit_group(verb, &artifact_mutations, &child_emits, None, effects, events, ui_scope, meta, None, transaction).await?;\n"),
    ("                if !in_transaction && !(published_window_config && config_edit_id.is_none() && matches!(kind, ActionKind::View)) {\n                    self.record_command(verb, kind, log_label, None, config_edit_id, None);\n                }\n",
     "                if !in_transaction && !config_edited {\n                    self.record_command(verb, kind, log_label, None, Vec::new(), None);\n                }\n"),
    ("                    self.record_command(verb, kind, log_label, Some(edit_id), config_edit_id, None);\n",
     "                    self.record_command(verb, kind, log_label, Some(edit_id), Vec::new(), None);\n"),
    # L. dispatch_emit_group
    ("        /// 🧾️ One command-log row for the whole group — `child_edit_ids` follows the existing\n        /// `config_edit_ids` precedent (see `CommandLogEntry::child_edit_ids`'s own doc comment).\n",
     "        /// 🧾️ One command-log row for the whole group, carrying every touched child's edit in `child_edit_ids` (see\n        /// `CommandLogEntry::child_edit_ids`'s own doc comment); a config-lane edit never joins it (design §20.13, L4).\n"),
    ("            ui_scope: UiDirtyScope,\n            config_edit_id: Option<String>,\n            meta: &ActionMeta,\n            group_id: Option<String>,\n",
     "            ui_scope: UiDirtyScope,\n            meta: &ActionMeta,\n            group_id: Option<String>,\n"),
    ("            self.record_command(verb, kind, label, parent_edit_id, config_edit_id, None);\n            if let Some(last) = self.command_log.last_mut() {\n                last.child_edit_ids = child_edit_ids;\n            }\n",
     "            self.record_command(verb, kind, label, parent_edit_id, child_edit_ids, None);\n"),
    ("&children, description, Vec::new(), Vec::new(), UiDirtyScope::Full, None, meta, Some(txn_id.to_string()), None)\n",
     "&children, description, Vec::new(), Vec::new(), UiDirtyScope::Full, meta, Some(txn_id.to_string()), None)\n"),
    ("&pending.child_emits, None, Vec::new(), Vec::new(), UiDirtyScope::None, None, &mounted.meta, None, pending.transaction.clone()).await;\n",
     "&pending.child_emits, None, Vec::new(), Vec::new(), UiDirtyScope::None, &mounted.meta, None, pending.transaction.clone()).await;\n"),
    # M. revert lane
    ("        Revert { lane: FrameworkRevertLane, edit_id: String },\n    }\n\n    /// ⏪️ The store a `revertToCommand` walks back one undo per unit.\n    #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n    enum FrameworkRevertLane {\n        Document,\n        Configuration,\n    }\n",
     "        Revert { edit_id: String },\n    }\n"),
    ("""                    let target =
                        entry_seq.and_then(|seq| self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.config_edit_id.clone())));
                    match target {
                        Some((Some(edit_id), _)) => {
                            let applied = self.store.applied_edit_ids();
                            Ok(applied.iter().position(|id| *id == edit_id).map_or(1, |position| applied.len().saturating_sub(position + 1).max(1)))
                        }
                        Some((None, Some(config_edit_id))) => {
                            let applied = self.config_store.applied_edit_ids();
                            Ok(applied.iter().position(|id| *id == config_edit_id).map_or(1, |position| applied.len().saturating_sub(position + 1).max(1)))
                        }
                        _ => Ok(1),
                    }
""",
     """                    let target = entry_seq.and_then(|seq| self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).and_then(|entry| entry.edit_id.clone()));
                    match target {
                        Some(edit_id) => {
                            let applied = self.store.applied_edit_ids();
                            Ok(applied.iter().position(|id| *id == edit_id).map_or(1, |position| applied.len().saturating_sub(position + 1).max(1)))
                        }
                        None => Ok(1),
                    }
"""),
    ("                            Err((lane, edit_id)) => {\n                                commit.stage = FrameworkReservedCommitStage::Revert { lane, edit_id };\n",
     "                            Err(edit_id) => {\n                                commit.stage = FrameworkReservedCommitStage::Revert { edit_id };\n"),
    ("                FrameworkReservedCommitStage::Revert { lane, edit_id } => {\n                    if self.framework_revert_unit(lane, &edit_id).await? {\n                        commit.total = commit.total.saturating_add(1);\n                        commit.stage = FrameworkReservedCommitStage::Revert { lane, edit_id };\n",
     "                FrameworkReservedCommitStage::Revert { edit_id } => {\n                    if self.framework_revert_unit(&edit_id).await? {\n                        commit.total = commit.total.saturating_add(1);\n                        commit.stage = FrameworkReservedCommitStage::Revert { edit_id };\n"),
    ("                    self.record_command(REVERT_TO_COMMAND_ACTION_ID, ActionKind::History, None, None, None, None);\n",
     "                    self.record_command(REVERT_TO_COMMAND_ACTION_ID, ActionKind::History, None, None, Vec::new(), None);\n"),
    ("""        async fn begin_framework_revert_route(&mut self, args: Option<&DslValue>, meta: &ActionMeta) -> Result<Result<InvocationResult, (FrameworkRevertLane, String)>, Fault> {""",
     """        async fn begin_framework_revert_route(&mut self, args: Option<&DslValue>, meta: &ActionMeta) -> Result<Result<InvocationResult, String>, Fault> {"""),
    ("""            let target = entry_seq.and_then(|seq| {
                self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.config_edit_id.clone(), entry.kind, entry.inverse.clone()))
            });
            let (lane, edit_id) = match target {
                Some((Some(edit_id), _, _, _)) => (FrameworkRevertLane::Document, edit_id),
                Some((None, Some(config_edit_id), _, _)) => (FrameworkRevertLane::Configuration, config_edit_id),
                Some((None, None, ActionKind::Shell, Some(inverse))) => return Ok(Ok(Self::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], Vec::new(), UiDirtyScope::None).await)),
                _ => return Ok(Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            };
            let applied = match lane {
                FrameworkRevertLane::Document => self.store.applied_edit_ids(),
                FrameworkRevertLane::Configuration => self.config_store.applied_edit_ids(),
            };
            if !applied.iter().any(|id| *id == edit_id) {
                return Ok(Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await));
            }
            Ok(Err((lane, edit_id)))
        }

        /// ⏪️ One revert unit: undoes the newest edit of `lane` while `edit_id` is not yet its tail.
        /// Answers whether an undo was applied (`false` once the target is the tail or the lane refuses).
        async fn framework_revert_unit(&mut self, lane: FrameworkRevertLane, edit_id: &str) -> Result<bool, Fault> {
            let pending = match lane {
                FrameworkRevertLane::Document => {
                    let applied = self.store.applied_edit_ids();
                    applied.iter().any(|id| id.as_str() == edit_id) && applied.last().is_some_and(|tail| tail.as_str() != edit_id)
                }
                FrameworkRevertLane::Configuration => {
                    let applied = self.config_store.applied_edit_ids();
                    applied.iter().any(|id| id.as_str() == edit_id) && applied.last().is_some_and(|tail| tail.as_str() != edit_id)
                }
            };
            if !pending {
                return Ok(false);
            }
            let undone = match lane {
                FrameworkRevertLane::Document => self.store.dispatch(ArtifactCommand::Undo).await.map(|_| ()),
                FrameworkRevertLane::Configuration => self.config_store.dispatch(ArtifactCommand::Undo).await.map(|_| ()),
            };
            match undone {
""",
     """            let target = entry_seq.and_then(|seq| self.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.kind, entry.inverse.clone())));
            let edit_id = match target {
                Some((Some(edit_id), _, _)) => edit_id,
                Some((None, ActionKind::Shell, Some(inverse))) => return Ok(Ok(Self::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], Vec::new(), UiDirtyScope::None).await)),
                _ => return Ok(Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            };
            if !self.store.applied_edit_ids().iter().any(|id| *id == edit_id) {
                return Ok(Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await));
            }
            Ok(Err(edit_id))
        }

        /// ⏪️ One revert unit: undoes the newest document edit while `edit_id` is not yet the document store's tail.
        /// Answers whether an undo was applied (`false` once the target is the tail or the store refuses).
        async fn framework_revert_unit(&mut self, edit_id: &str) -> Result<bool, Fault> {
            let applied = self.store.applied_edit_ids();
            if !(applied.iter().any(|id| id.as_str() == edit_id) && applied.last().is_some_and(|tail| tail.as_str() != edit_id)) {
                return Ok(false);
            }
            let undone = self.store.dispatch(ArtifactCommand::Undo).await.map(|_| ());
            match undone {
"""),
    # N. chrome undo
    ("""            let applied: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();
            let config_applied: HashSet<&str> = self.config_store.applied_edit_ids().iter().map(String::as_str).collect();
            let target = self.command_log.iter().rev().find(|entry| {
                let shell = entry.inverse.is_some() && entry.edit_id.is_none() && entry.config_edit_ids.is_empty() && !self.shell_undone.contains(&entry.seq);
                let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id));
                let config = entry.config_edit_ids.iter().any(|id| config_applied.contains(id.as_str()));
                let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
                shell || document || config || child
            });
            let Some(entry) = target else {
                return Ok(None);
            };
            if entry.edit_id.is_some() || !entry.config_edit_ids.is_empty() || entry.inverse.is_none() {
""",
     """            let applied: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();
            let target = self.command_log.iter().rev().find(|entry| {
                let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.shell_undone.contains(&entry.seq);
                let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id));
                let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
                shell || document || child
            });
            let Some(entry) = target else {
                return Ok(None);
            };
            if entry.edit_id.is_some() || entry.inverse.is_none() {
"""),
    # O. configCommand
    ("            self.cache = None;\n            let config_edit_id = self.config_store.envelope().vcs.edits.last().map(|edit| edit.id.clone());\n            self.record_command(\"configCommand\", ActionKind::Shell, None, None, config_edit_id, None);\n            permit.finish();\n",
     "            self.cache = None;\n            permit.finish();\n"),
    # P. close retained fields
    ("""            if let Some(entry) = self.command_log.last_mut() {
                if let Some(value) = entry.config_edit_ids.last_mut() {
                    if let Some(step) = Self::close_retained_string_page(value, maximum_bytes) {
                        return step;
                    }
                    drop(entry.config_edit_ids.pop());
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if let Some(value) = entry.child_edit_ids.last_mut() {
""",
     """            if let Some(entry) = self.command_log.last_mut() {
                if let Some(value) = entry.child_edit_ids.last_mut() {
"""),
]

tt = [
    ("label, kind: ActionKind::History, edit_id: None, config_edit_id: None, transition_id: None, timestamp: None, inverse: None });",
     "label, kind: ActionKind::History, edit_id: None, transition_id: None, timestamp: None, inverse: None });"),
    ("        let applied: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();\n        let config_applied: HashSet<&str> = self.config_store.applied_edit_ids().iter().map(String::as_str).collect();\n        let envelope = self.store.envelope();\n",
     "        let applied: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();\n        let envelope = self.store.envelope();\n"),
    ("            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && entry.config_edit_ids.is_empty() && !self.shell_undone.contains(&entry.seq);\n            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id) && own_edit(id));\n            let config = entry.config_edit_ids.iter().any(|id| config_applied.contains(id.as_str()));\n            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));\n            if shell || document || config || child {\n",
     "            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.shell_undone.contains(&entry.seq);\n            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id) && own_edit(id));\n            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));\n            if shell || document || child {\n"),
]

def apply(path, edits):
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            raise SystemExit(f"{path.name}: anchor count {text.count(old)}: {old[:120]!r}")
        text = text.replace(old, new)
    return text

texts = {ROOT: apply(ROOT, root), TT: apply(TT, tt)}
texts[ROOT] = texts[ROOT].replace("self.record_typed_operation_lane(mounted, true);", "self.record_typed_operation_lane(mounted);")
gone = "                                self.record_typed_operation_lane(mounted, false);\n"
if texts[ROOT].count(gone) != 1:
    raise SystemExit("config lane typed record anchor")
texts[ROOT] = texts[ROOT].replace(gone, "")
before = texts[ROOT].count(", None, None, None, None);")
texts[ROOT] = texts[ROOT].replace(", None, None, None, None);", ", None, None, Vec::new(), None);")
texts[ROOT] = texts[ROOT].replace("self.record_command(command_id, ActionKind::Shell, Some(label), None, None, inverse);", "self.record_command(command_id, ActionKind::Shell, Some(label), None, Vec::new(), inverse);")
texts[ROOT] = texts[ROOT].replace("description.map(LocalizedLabel::data), Some(edit_id.clone()), None, None);", "description.map(LocalizedLabel::data), Some(edit_id.clone()), Vec::new(), None);")
for path, text in texts.items():
    path.write_text(text, encoding="utf-8")
left = [line for line in texts[ROOT].splitlines() if "config_edit_id" in line or "FrameworkRevertLane" in line or "history_row_applied_v1" in line]
print(f"L4: {len(root)} + {len(tt)} anchored edits, {before} record_command None→Vec rewrites; leftovers: {left}")
