import re
P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
s = open(P).read()

def rep(old, new, count=1):
    global s
    n = s.count(old)
    assert n == count, (n, old[:120])
    s = s.replace(old, new)

# A: types
rep("""    struct PendingFrameworkReserved {
        action: String,
        args: Option<DslValue>,
        meta: ActionMeta,
        permit: FrameworkReservedCommitPermit,
    }
""", """    struct PendingFrameworkReserved {
        action: String,
        args: Option<DslValue>,
        meta: ActionMeta,
        permit: FrameworkReservedCommitPermit,
    }

    /// 🧾️ A framework-reserved route whose spawned job finished and whose commit the app now owns.
    /// [`VcsArtifactApp::step_framework_reserved_commit`] advances it one bounded unit per call from the
    /// reactor's typed-operation continuation, so a commit that fans out over composed children
    /// (checkpoint pins, checkout cascade) or walks history (revert) is driven across turns instead of
    /// having to finish inside the turn that saw its `JobCompleted`. Every unit re-reads the
    /// operation's cancellation lease, and `applied`/`total` is its progress.
    struct FrameworkReservedCommit {
        action: String,
        args: Option<DslValue>,
        meta: ActionMeta,
        permit: FrameworkReservedCommitPermit,
        log_generation_before: u64,
        stage: FrameworkReservedCommitStage,
        applied: u64,
        total: u64,
    }

    /// 🪜️ The next unit a [`FrameworkReservedCommit`] runs.
    enum FrameworkReservedCommitStage {
        Validate,
        CheckpointChildren { message: String, authors: Vec<vcs::Author>, keys: Vec<(String, String)>, next: usize, pins: Vec<vcs::CompositionPin> },
        Route { pins: Vec<vcs::CompositionPin> },
        CheckoutChildren { pins: Vec<vcs::CompositionPin>, next: usize, result: InvocationResult },
        Revert { lane: FrameworkRevertLane, edit_id: String },
    }

    /// ⏪️ The store a `revertToCommand` walks back one undo per unit.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum FrameworkRevertLane {
        Document,
        Configuration,
    }

    /// 📊️ Progress of the framework-reserved commit at the head of an app's commit queue: `applied` of
    /// `total` bounded units, for the operation that admitted it.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct FrameworkReservedCommitProgress {
        pub operation: u64,
        pub applied: u64,
        pub total: u64,
    }
""")

# B: fields + init
rep("""        pending_reserved: ArtifactFixedRegistry<PendingFrameworkReserved>,
        media_closures""", """        pending_reserved: ArtifactFixedRegistry<PendingFrameworkReserved>,
        reserved_commits: std::collections::VecDeque<FrameworkReservedCommit>,
        reserved_commit_outcome: Option<Result<InvocationResult, Fault>>,
        media_closures""")
rep("""                pending_reserved: ArtifactFixedRegistry::new(),
                media_closures""", """                pending_reserved: ArtifactFixedRegistry::new(),
                reserved_commits: std::collections::VecDeque::with_capacity(ARTIFACT_LIVE_OUTPUT_SLOTS),
                reserved_commit_outcome: None,
                media_closures""")

# C: cascades -> units
start = s.index("        async fn commit_children_for_checkpoint(")
end = s.index("        /// @emoji 📌️ Records `pins` on the checkpoint the parent's dispatch just created.")
s = s[:start] + """        /// @emoji 📌️ One checkpoint-cascade unit: commits the child at `slot`/`child_id` when it is dirty,
        /// republishes its content root, and answers the pin its current checkpoint contributes.
        async fn checkpoint_child_unit(&mut self, message: &str, authors: &[vcs::Author], slot: &str, child_id: &str) -> Result<Option<vcs::CompositionPin>, Fault> {
            let publication_generation = self.admit_child_content_publication()?;
            let (dialect, checkpoint_id) = {
                let entry = self.children.get_mut(&(slot.to_string(), child_id.to_string())).ok_or_else(|| plugin_sdk_fault("checkpoint child authority changed during bounded publication"))?;
                if entry.member.is_dirty().await {
                    entry.member.commit_checkpoint(message.to_string(), authors.to_vec()).await.map_err(|error| error.into_fault())?;
                }
                (entry.reference.dialect.clone(), entry.member.current_checkpoint_id().await)
            };
            self.publish_child_content_member(publication_generation, slot, child_id).await?;
            Ok(checkpoint_id.map(|checkpoint_id| vcs::CompositionPin { child_ref: ArtifactRef { artifact_id: child_id.to_string(), dialect }, checkpoint_id }))
        }

""" + s[end:]

start = s.index("        /// @emoji ⏮️ Checkout half of the cascade:")
end = s.index("        //#endregion 🔖️CheckpointCascade")
s = s[:start] + """        /// @emoji ⏮️ Checkout half of the cascade: the pins the parent's now-current checkpoint recorded,
        /// each restored by one [`Self::checkout_child_unit`]. A pin naming a child that is not currently
        /// open is QUEUED (`pending_child_pins`) rather than dropped, so a child adopted later still lands
        /// on its pinned state instead of silently staying at head — see `open_child`.
        fn checkout_cascade_pins(&mut self) -> Result<Vec<vcs::CompositionPin>, Fault> {
            let Some(checkpoint_id) = self.store.current_checkpoint_id().map(str::to_string) else { return Ok(Vec::new()) };
            let Some(pins) = self.store.envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).map(|checkpoint| checkpoint.composition_pins.clone()) else { return Ok(Vec::new()) };
            self.pending_child_pins.clear();
            self.admit_child_content_publication_span(pins.len())?;
            Ok(pins)
        }

        /// @emoji ⏮️ One checkout-cascade unit: restores one live child to `pin`, or queues the pin.
        async fn checkout_child_unit(&mut self, pin: &vcs::CompositionPin) -> Result<(), Fault> {
            let child_key = self
                .children
                .entries()
                .find(|entry| entry.reference.artifact_id == pin.child_ref.artifact_id)
                .map(|entry| (entry.owner.slot.clone(), entry.reference.artifact_id.clone()));
            let Some((slot, child_id)) = child_key else {
                self.pending_child_pins.push(pin.clone());
                return Ok(());
            };
            let publication_generation = self.admit_child_content_publication()?;
            let entry = self.children.get_mut(&(slot.clone(), child_id.clone())).ok_or_else(|| plugin_sdk_fault("checkout child authority changed during bounded publication"))?;
            let alternative_id = entry.member.current_alternative_id().await.unwrap_or_default();
            let _ = entry.member.checkout(&pin.checkpoint_id, &alternative_id).await;
            self.publish_child_content_member(publication_generation, &slot, &child_id).await
        }
""" + s[end:]

# D: completion -> admit + step
start = s.index("        async fn complete_reserved_spawned_job_inner(")
end = s.index("        async fn dispatch_chrome_history_action(")
s = s[:start] + """        /// 🕰️ Hands one finished framework-reserved spawn-job to the app's commit queue. Nothing here
        /// suspends: the commit itself runs unit by unit in [`Self::step_framework_reserved_commit`].
        /// Answers `false` for a job this app never admitted.
        fn admit_framework_reserved_commit(&mut self, job: u64, output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
            let Some(pending) = self.pending_reserved.remove(job) else {
                return Ok(false);
            };
            if let Err(fault) = output {
                pending.permit.finish();
                return Err(fault);
            }
            if self.reserved_commits.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
                pending.permit.finish();
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-capacity"), format!("framework route '{}' found the reserved commit queue full", pending.action)));
            }
            let PendingFrameworkReserved { action, args, meta, permit } = pending;
            self.reserved_commits.push_back(FrameworkReservedCommit { action, args, meta, permit, log_generation_before: self.log_generation, stage: FrameworkReservedCommitStage::Validate, applied: 0, total: 1 });
            Ok(true)
        }

        /// 📊️ Progress of the commit at the head of the reserved-commit queue.
        pub fn framework_reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
            self.reserved_commits.front().map(|commit| FrameworkReservedCommitProgress { operation: commit.permit.operation.operation.0, applied: commit.applied, total: commit.total })
        }

        /// 🪜️ Runs ONE bounded unit of the commit at the head of the queue. Commits are strictly FIFO,
        /// and the head waits while a finished outcome is still untaken, so at most one outcome is ever
        /// owed to the host. A finished commit leaves its result in `reserved_commit_outcome`.
        async fn step_framework_reserved_commit(&mut self) -> Result<(), Fault> {
            if self.reserved_commit_outcome.is_some() {
                return Ok(());
            }
            let Some(mut commit) = self.reserved_commits.pop_front() else {
                return Ok(());
            };
            match self.run_framework_reserved_commit_unit(&mut commit).await {
                Ok(Some(result)) => {
                    let result = self.finish_recorded(commit.log_generation_before, &commit.action, result).await;
                    let _ = self.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: commit.permit.operation.operation.0, ui_scope: result.ui_scope.clone() });
                    commit.permit.finish();
                    self.reserved_commit_outcome = Some(Ok(result));
                }
                Ok(None) => {
                    commit.applied = commit.applied.saturating_add(1);
                    self.reserved_commits.push_front(commit);
                }
                Err(fault) => {
                    commit.permit.finish();
                    self.reserved_commit_outcome = Some(Err(fault));
                }
            }
            Ok(())
        }

        /// 🧾️ Takes the result of the last finished reserved commit.
        fn take_framework_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
            self.reserved_commit_outcome.take()
        }

        async fn run_framework_reserved_commit_unit(&mut self, commit: &mut FrameworkReservedCommit) -> Result<Option<InvocationResult>, Fault> {
            if commit.permit.is_cancelled().await {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{}' was cancelled between commit units", commit.action)));
            }
            match std::mem::replace(&mut commit.stage, FrameworkReservedCommitStage::Validate) {
                FrameworkReservedCommitStage::Validate => {
                    self.validate_framework_reserved_commit(&commit.action, &commit.permit).await?;
                    commit.stage = match Self::history_command(&commit.action, commit.args.as_ref()).await {
                        Some(ArtifactCommand::CommitCheckpoint { message, authors }) => {
                            let keys: Vec<(String, String)> = self.children.entries().map(|entry| (entry.owner.slot.clone(), entry.reference.artifact_id.clone())).collect();
                            self.admit_child_content_publication_span(keys.len())?;
                            commit.total = keys.len() as u64 + 2;
                            FrameworkReservedCommitStage::CheckpointChildren { message: message.unwrap_or_else(|| "checkpoint".to_string()), authors, keys, next: 0, pins: Vec::new() }
                        }
                        _ => {
                            commit.total = 2;
                            FrameworkReservedCommitStage::Route { pins: Vec::new() }
                        }
                    };
                    Ok(None)
                }
                FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next, mut pins } => {
                    if let Some((slot, child_id)) = keys.get(next) {
                        if let Some(pin) = self.checkpoint_child_unit(&message, &authors, slot, child_id).await? {
                            pins.push(pin);
                        }
                        commit.stage = FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next: next + 1, pins };
                    } else {
                        pins.sort_by(|left, right| left.child_ref.artifact_id.cmp(&right.child_ref.artifact_id));
                        commit.stage = FrameworkReservedCommitStage::Route { pins };
                    }
                    Ok(None)
                }
                FrameworkReservedCommitStage::Route { pins } => {
                    let action = commit.action.clone();
                    if HISTORY_ACTION_IDS.contains(&action.as_str()) {
                        let result = self.commit_framework_history_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, pins).await?;
                        if action == "checkoutCheckpoint" && result.ui_scope != UiDirtyScope::None {
                            let pins = self.checkout_cascade_pins()?;
                            if !pins.is_empty() {
                                commit.total = commit.total.saturating_add(pins.len() as u64);
                                commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: 0, result };
                                return Ok(None);
                            }
                        }
                        Ok(Some(result))
                    } else if action == REVERT_TO_COMMAND_ACTION_ID {
                        match self.begin_framework_revert_route(commit.args.as_ref(), &commit.meta).await? {
                            Ok(result) => Ok(Some(result)),
                            Err((lane, edit_id)) => {
                                commit.stage = FrameworkReservedCommitStage::Revert { lane, edit_id };
                                Ok(None)
                            }
                        }
                    } else {
                        self.commit_framework_shared_host_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit).await.map(Some)
                    }
                }
                FrameworkReservedCommitStage::CheckoutChildren { pins, next, result } => {
                    let Some(pin) = pins.get(next) else { return Ok(Some(result)) };
                    self.checkout_child_unit(pin).await?;
                    commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: next + 1, result };
                    Ok(None)
                }
                FrameworkReservedCommitStage::Revert { lane, edit_id } => {
                    if self.framework_revert_unit(lane, &edit_id).await? {
                        commit.total = commit.total.saturating_add(1);
                        commit.stage = FrameworkReservedCommitStage::Revert { lane, edit_id };
                        return Ok(None);
                    }
                    self.cache = None;
                    self.record_command(REVERT_TO_COMMAND_ACTION_ID, ActionKind::History, None, None, None, None);
                    Ok(Some(Self::empty_result(REVERT_TO_COMMAND_ACTION_ID, &commit.meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await))
                }
            }
        }

""" + s[end:]

# E: history route pins param, no cascades
rep("""        async fn commit_framework_history_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit) -> Result<InvocationResult, Fault> {""",
    """        async fn commit_framework_history_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, pins: Vec<vcs::CompositionPin>) -> Result<InvocationResult, Fault> {""")
rep("""            let pending_pins = match &command {
                ArtifactCommand::CommitCheckpoint { message, authors } => self.commit_children_for_checkpoint(message.clone(), authors.clone(), permit).await?,
                _ => Vec::new(),
            };
            match self.store.dispatch(command).await {
                Ok(_) => {
                    self.stamp_checkpoint_composition_pins(pending_pins).await?;
                    if action == "checkoutCheckpoint" {
                        self.cascade_checkout_to_children(permit).await?;
                    }
                    self.cache = None;""", """            match self.store.dispatch(command).await {
                Ok(_) => {
                    self.stamp_checkpoint_composition_pins(pins).await?;
                    self.cache = None;""")

# F: revert route -> begin + unit
start = s.index("        async fn commit_framework_revert_route(")
end = s.index("        async fn ensure_reserved_emit_bounded(")
s = s[:start] + """        /// ⏪️ Resolves a `revertToCommand` target. A document or configuration edit answers the lane and
        /// edit id [`Self::framework_revert_unit`] walks back one undo per commit unit; every other target
        /// answers its complete result.
        async fn begin_framework_revert_route(&mut self, args: Option<&DslValue>, meta: &ActionMeta) -> Result<Result<InvocationResult, (FrameworkRevertLane, String)>, Fault> {
            let action = REVERT_TO_COMMAND_ACTION_ID;
            self.refresh_cache().await?;
            let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
            let target = entry_seq.and_then(|seq| {
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
                Ok(()) => Ok(true),
                Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(false),
                Err(error) => Err(error.into_fault()),
            }
        }

""" + s[end:]
open(P, "w").write(s)
print("ok")
