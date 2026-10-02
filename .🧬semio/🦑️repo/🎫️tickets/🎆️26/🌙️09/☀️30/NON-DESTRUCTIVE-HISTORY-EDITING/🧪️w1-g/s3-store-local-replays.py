#!/usr/bin/env python3
"""🐢️ S3-W1G (N17, design §16.6): resumable local history steps in the store — anchored, all-or-nothing rewrite."""
import sys

path = sys.argv[1]
source = open(path, encoding="utf-8").read()

EDITS = [
# 1. PendingReprojection + PendingLocalStep
("""/// 🐢️ A remote history change this store admitted but has not adopted yet ([`ArtifactStore::defer_remote_replays`]): its
/// transitions wait outside the log while their Report replay steps turn by turn, and the replay restarts whenever the
/// history it started from changed (the content revision it names).
struct PendingReprojection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    transitions: Vec<crate::os_spr::MutationEnvelope>,
    replay: Option<([u8; 32], EditReplay<P, Mutation>)>,
}
""",
"""/// 🐢️ A history change this store admitted but has not adopted yet (design §16.6): the remote `transitions` it admitted
/// ([`ArtifactStore::defer_remote_replays`]) and at most one `local` step this replica authored
/// ([`ArtifactStore::defer_local_replays`]) wait outside the log while their Report replay steps turn by turn; the replay
/// restarts whenever the history it started from changed (the content revision it names).
struct PendingReprojection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    transitions: Vec<crate::os_spr::MutationEnvelope>,
    local: Option<PendingLocalStep>,
    replay: Option<([u8; 32], EditReplay<P, Mutation>)>,
}

impl<P, Mutation> PendingReprojection<P, Mutation>
where
    Mutation: self::Mutation<P>,
{
    /// 🫙️ Nothing admitted yet.
    fn empty() -> Self {
        Self { transitions: Vec::new(), local: None, replay: None }
    }
}

/// 🖋️ A local history step whose Report replay outlasts its dispatch: the transitions it authored (neither recorded nor
/// announced yet), the head `(line, checkpoint)` it moves this replica to, and whether it finalizes a supersession — whose
/// report must not block finalizing, or the step is refused with nothing of it recorded.
struct PendingLocalStep {
    envelopes: Vec<crate::os_spr::MutationEnvelope>,
    head: (Option<String>, Option<String>),
    finalizing: bool,
}
"""),
# 2. moves_history
("""    /// 🏷️ Every kind of history transition this command may author, in authoring order: what the store's
    /// [`crate::os_spr::HistoryShape`] must admit before the command runs.
    pub fn history_transition_kinds(&self) -> &'static [crate::os_spr::HistoryTransitionKind] {""",
"""    /// 🐢️ Whether this command moves history positions — an undo, redo, checkpoint, alternative, checkout, supersession or
    /// conflict resolution: what a store whose local history step still replays refuses ([`VcsError::HistoryReplaying`]).
    pub fn moves_history(&self) -> bool {
        !self.history_transition_kinds().is_empty() || matches!(self, Self::ResolveConflict { .. })
    }

    /// 🏷️ Every kind of history transition this command may author, in authoring order: what the store's
    /// [`crate::os_spr::HistoryShape`] must admit before the command runs.
    pub fn history_transition_kinds(&self) -> &'static [crate::os_spr::HistoryTransitionKind] {"""),
# 3. store fields
("""    /// 🐢️ Operations one turn of a deferred remote reprojection replays; `None` replays inside the ingest.
    replay_budget: Option<usize>,
    /// 🐢️ The remote history change admitted but not adopted yet ([`Self::step_reprojection`]).
    pending_reprojection: Option<PendingReprojection<P, Mutation>>,""",
"""    /// 🐢️ Operations one turn of a deferred remote reprojection replays; `None` replays inside the ingest.
    replay_budget: Option<usize>,
    /// 🐢️ Operations one turn of a deferred local history step replays; `None` replays inside the dispatch.
    local_replay_budget: Option<usize>,
    /// 🐢️ The history change admitted but not adopted yet ([`Self::step_reprojection`]).
    pending_reprojection: Option<PendingReprojection<P, Mutation>>,"""),
# 4a/4b constructors (two indentations)
("""        authoring_verb: None,
        replay_budget: None,
        pending_reprojection: None,""",
"""        authoring_verb: None,
        replay_budget: None,
        local_replay_budget: None,
        pending_reprojection: None,"""),
("""            authoring_verb: None,
            replay_budget: None,
            pending_reprojection: None,""",
"""            authoring_verb: None,
            replay_budget: None,
            local_replay_budget: None,
            pending_reprojection: None,"""),
# 5. admit_remote_transitions tail
("""        if self.pending_reprojection.is_none() && (self.replay_budget.is_none() || fresh.is_empty()) {
            return self.adopt_remote_transitions(fresh, None).await;
        }
        let mut transitions = self.pending_reprojection.as_ref().map(|pending| pending.transitions.clone()).unwrap_or_default();
        transitions.extend(fresh);
        self.prospective_fold(&transitions)?;
        self.pending_reprojection = Some(PendingReprojection { transitions, replay: None });
        match self.advance_reprojection().await? {""",
"""        if self.pending_reprojection.is_none() && (self.replay_budget.is_none() || fresh.is_empty()) {
            return self.adopt_remote_transitions(fresh, None).await;
        }
        let mut pending = self.pending_reprojection.take().unwrap_or_else(PendingReprojection::empty);
        let known = pending.transitions.len();
        pending.transitions.extend(fresh);
        if let Err(error) = self.pending_fold(&pending) {
            pending.transitions.truncate(known);
            self.pending_reprojection = (!pending.transitions.is_empty() || pending.local.is_some()).then_some(pending);
            return Err(error);
        }
        drop(pending.replay.take());
        self.pending_reprojection = Some(pending);
        match self.advance_reprojection().await? {"""),
# 6. install_transitions deferral
("""    /// 🏗️ Records locally authored transition envelopes and materializes the projection once — adopting `replayed` when it
    /// is the Report replay of exactly the resulting log, replaying otherwise. All of them leave in this dispatch's outbound
    /// batch, or none is recorded.
    async fn install_transitions(&mut self, envelopes: Vec<crate::os_spr::MutationEnvelope>, clock: HybridLogicalTimestamp, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<(), VcsError> {
        if let Err(error) = self.admit_history_shape(&envelopes) {
            drop(replayed);
            return Err(error);
        }
        let identities""",
"""    /// 🏗️ Records locally authored transition envelopes and materializes the projection once — adopting `replayed` when it
    /// is the Report replay of exactly the resulting log, replaying otherwise (under [`Self::defer_local_replays`] a replay
    /// longer than one budget waits for later turns). All of them leave in this dispatch's outbound batch, or none is
    /// recorded.
    async fn install_transitions(&mut self, envelopes: Vec<crate::os_spr::MutationEnvelope>, clock: HybridLogicalTimestamp, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<(), VcsError> {
        if let Err(error) = self.admit_history_shape(&envelopes) {
            drop(replayed);
            return Err(error);
        }
        let envelopes = match replayed {
            Some(_) => envelopes,
            None => match self.defer_local_step(PendingLocalStep { envelopes, head: self.viewer_head_now(), finalizing: false }, clock).await? {
                Some(step) => step.envelopes,
                None => return Ok(()),
            },
        };
        let identities"""),
# 7. reprojection_replay doc
("""    /// Synchronous by budget: one command or ingest turn replays the applied history in place. A longer history still
    /// replays here; a resumable reprojection is a separate gap.
    fn reprojection_replay(""",
"""    /// Synchronous: one command or ingest replays the applied history in place. A remote change under
    /// [`Self::defer_remote_replays`] and a local step under [`Self::defer_local_replays`] replay resumably instead
    /// ([`Self::step_reprojection`]).
    fn reprojection_replay("""),
# 8. switch / checkout -> move_head
("""            self.envelope.active_alternative_id = Some(alternative_id);
        } else {
            self.envelope.active_alternative_id = None;
        }
        self.envelope.viewer_checkpoint_id = None;
        self.reproject().await?;
        self.bump()
    }
""",
"""            self.move_head((Some(alternative_id), None)).await
        } else {
            self.move_head((None, None)).await
        }
    }
"""),
("""        self.envelope.active_alternative_id = self.branched_alternative(line);
        self.envelope.viewer_checkpoint_id = Some(checkpoint_id);
        self.reproject().await?;
        self.bump()
    }
""",
"""        self.move_head((self.branched_alternative(line), Some(checkpoint_id))).await
    }

    /// 👁 Moves this replica's head to `(line, checkpoint)` and materializes the projection; a refused move keeps the head it
    /// had. Under [`Self::defer_local_replays`] a move whose replay is longer than one budget waits for later turns.
    async fn move_head(&mut self, head: (Option<String>, Option<String>)) -> Result<(), VcsError> {
        let Some(PendingLocalStep { head, .. }) = self.defer_local_step(PendingLocalStep { envelopes: Vec::new(), head, finalizing: false }, self.clock).await? else {
            return Ok(());
        };
        let saved = self.viewer_head_now();
        (self.envelope.active_alternative_id, self.envelope.viewer_checkpoint_id) = head;
        if let Err(error) = self.reproject().await {
            (self.envelope.active_alternative_id, self.envelope.viewer_checkpoint_id) = saved;
            return Err(error);
        }
        self.bump()
    }

    /// 👁 This replica's head `(line, checkpoint)` as the envelope holds it.
    fn viewer_head_now(&self) -> (Option<String>, Option<String>) {
        (self.envelope.active_alternative_id.clone(), self.envelope.viewer_checkpoint_id.clone())
    }

    /// 👁 The fold head of `(line, checkpoint)`: an absent line (or the trunk's id) is the trunk.
    fn viewer_head(&self, (line, checkpoint): &(Option<String>, Option<String>)) -> crate::os_spr::ViewerHead {
        let trunk = self.trunk_alternative_id();
        crate::os_spr::ViewerHead { line_id: line.clone().filter(|line| *line != trunk).unwrap_or(trunk), checkpoint_id: checkpoint.clone() }
    }
"""),
# 9. DeferredReprojection region
("""    //#region 🔖️DeferredReprojection
    /// 🐢️ Steps the Report replay a remote history change needs — a supersession, undo or redo another replica authored —
    /// in budgets of at most `operations` per turn ([`Self::step_reprojection`], also every [`Self::tick`]) instead of inside
    /// the ingest that admitted it, so a long downstream replay never freezes this replica. Until the change is adopted the
    /// replica shows the history before it, and the adoption is exactly the one an ingest without a budget makes. `None`
    /// replays inside the ingest.
    pub fn defer_remote_replays(&mut self, operations: Option<usize>) {
        self.replay_budget = operations.filter(|operations| *operations > 0);
    }

    /// 📶️ The progress of the deferred remote reprojection (`done` 0 until its replay starts), `None` when nothing waits.
    pub fn reprojection_progress(&self) -> Option<ReplayProgress> {
        self.pending_reprojection.as_ref().map(|pending| pending.replay.as_ref().map_or_else(ReplayProgress::default, |(_, replay)| replay.progress()))
    }

    /// ✋️ Drops the running replay of the deferred remote reprojection: the store is untouched, the transitions stay admitted
    /// and the next step replays them from the start. Answers whether a replay was running.
    pub fn cancel_reprojection(&mut self) -> bool {
        self.pending_reprojection.as_mut().and_then(|pending| pending.replay.take()).is_some()
    }

    /// ⏭️ Advances the deferred remote reprojection by one budget of operations and adopts it once its replay finished.
    /// Answers its progress while it still waits, `None` once nothing waits.
    pub async fn step_reprojection(&mut self) -> Result<Option<ReplayProgress>, VcsError> {
        match self.advance_reprojection().await? {
            Some(ReprojectionAdvance::Pending(progress)) => Ok(Some(progress)),
            Some(ReprojectionAdvance::Adopted(_)) => {
                self.bump()?;
                self.last_projection_cause = Some(ArtifactProjectionCause::Replay);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    /// 🧮️ One budget of the deferred remote reprojection: (re)starts its Report replay when the history it started from
    /// changed, steps it, and adopts the transitions with the finished replay; a change that needs no replay is adopted at
    /// once. A refused adoption drops the change and answers the refusal.
    async fn advance_reprojection(&mut self) -> Result<Option<ReprojectionAdvance>, VcsError> {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return Ok(None);
        };
        if pending.replay.as_ref().is_none_or(|(revision, _)| *revision != self.content_revision) {
            drop(pending.replay.take());
            let fold = self.prospective_fold(&pending.transitions)?;
            if fold.supersessions == *self.supersessions && self.applied_edit_ids.starts_with(&fold.applied) {
                return self.adopt_remote_transitions(pending.transitions, None).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed)));
            }
            let (from, horizon) = self.replay_window(&fold.applied, &fold.supersessions)?;
            let replay = self.report_replay(&fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len()))?;
            pending.replay = Some((self.content_revision, replay));
        }
        let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
        let mut left = self.replay_budget.unwrap_or(usize::MAX);
        let step = replay.step(&self.envelope.vcs.edits, &mut || {
            left = left.saturating_sub(1);
            left == 0
        })?;
        if let ReplayStep::Pending(progress) = step {
            self.pending_reprojection = Some(pending);
            return Ok(Some(ReprojectionAdvance::Pending(progress)));
        }
        let (_, replay) = pending.replay.take().expect("a finished deferred replay");
        let result = replay.finish()?;
        self.adopt_remote_transitions(pending.transitions, Some(result)).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed)))
    }
    //#endregion 🔖️DeferredReprojection""",
"""    //#region 🔖️DeferredReprojection
    /// 🐢️ Steps the Report replay a remote history change needs — a supersession, undo or redo another replica authored —
    /// in budgets of at most `operations` per turn ([`Self::step_reprojection`], also every [`Self::tick`]) instead of inside
    /// the ingest that admitted it, so a long downstream replay never freezes this replica. Until the change is adopted the
    /// replica shows the history before it, and the adoption is exactly the one an ingest without a budget makes. `None`
    /// replays inside the ingest.
    pub fn defer_remote_replays(&mut self, operations: Option<usize>) {
        self.replay_budget = operations.filter(|operations| *operations > 0);
    }

    /// 🐢️ Steps the Report replay a local history step needs — an interior undo or redo, a checkout or alternative switch
    /// away from the applied tail, a supersession (finalize) authored without a finished replay (design §16.6) — in budgets
    /// of at most `operations` per turn instead of inside its dispatch, which replays the first budget and answers. Until
    /// the step is adopted ([`Self::step_reprojection`], also every [`Self::tick`]) the replica shows the history before it,
    /// nothing of the step is recorded or announced, every further history step is refused
    /// ([`VcsError::HistoryReplaying`]) and [`Self::discard_local_step`] drops it with zero trace; edits and remote changes
    /// meanwhile restart its replay. The adoption is exactly the one an undeferred dispatch makes, a supersession whose
    /// report blocks finalizing included (refused with `Rejected { policy: Normal }`). Steps that need no replay adopt
    /// inside their dispatch. `None` replays inside the dispatch.
    pub fn defer_local_replays(&mut self, operations: Option<usize>) {
        self.local_replay_budget = operations.filter(|operations| *operations > 0);
    }

    /// 🖋️ Whether a local history step still replays ([`Self::defer_local_replays`]).
    pub fn local_step_pending(&self) -> bool {
        self.pending_reprojection.as_ref().is_some_and(|pending| pending.local.is_some())
    }

    /// 🗑️ Drops the local history step that still replays ([`Self::defer_local_replays`]): nothing of it was recorded,
    /// announced or shown, so the store is exactly as before its dispatch; a waiting remote change keeps waiting and
    /// replays again from its start. Answers whether a step was waiting.
    pub fn discard_local_step(&mut self) -> bool {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return false;
        };
        let discarded = pending.local.take().is_some();
        if discarded {
            drop(pending.replay.take());
        }
        self.pending_reprojection = (!pending.transitions.is_empty() || pending.local.is_some()).then_some(pending);
        discarded
    }

    /// 📶️ The progress of the deferred reprojection — a remote change, a local step or both (`done` 0 until its replay
    /// starts), `None` when nothing waits.
    pub fn reprojection_progress(&self) -> Option<ReplayProgress> {
        self.pending_reprojection.as_ref().map(|pending| pending.replay.as_ref().map_or_else(ReplayProgress::default, |(_, replay)| replay.progress()))
    }

    /// ✋️ Drops the running replay of the deferred reprojection: the store is untouched, the change stays admitted (a
    /// local step too — [`Self::discard_local_step`] drops it) and the next step replays it from the start. Answers whether a
    /// replay was running.
    pub fn cancel_reprojection(&mut self) -> bool {
        self.pending_reprojection.as_mut().and_then(|pending| pending.replay.take()).is_some()
    }

    /// ⏭️ Advances the deferred reprojection by one budget of operations and adopts it once its replay finished, announcing
    /// a local step's transitions. Answers its progress while it still waits, `None` once nothing waits.
    pub async fn step_reprojection(&mut self) -> Result<Option<ReplayProgress>, VcsError> {
        match self.advance_reprojection().await? {
            Some(ReprojectionAdvance::Pending(progress)) => Ok(Some(progress)),
            Some(ReprojectionAdvance::Adopted(_)) => {
                self.bump()?;
                self.last_projection_cause = Some(ArtifactProjectionCause::Replay);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    /// 🐢️ Takes a local history step whose Report replay must not run inside its dispatch ([`Self::defer_local_replays`]):
    /// joins it to the waiting change (with any remote transitions already waiting), advances the clock past its
    /// transitions, steps one budget and adopts everything once the replay finished — here or in a later turn. Answers the
    /// step back when it is adopted synchronously instead: local replays are not deferred, or a plain step needs no replay.
    async fn defer_local_step(&mut self, step: PendingLocalStep, clock: HybridLogicalTimestamp) -> Result<Option<PendingLocalStep>, VcsError> {
        if self.local_replay_budget.is_none() {
            return Ok(Some(step));
        }
        if !step.finalizing {
            let fold = self.prospective_fold_at(&step.envelopes, &self.viewer_head(&step.head))?;
            if fold.supersessions == *self.supersessions && self.applied_edit_ids.starts_with(&fold.applied) {
                return Ok(Some(step));
            }
        }
        let mut pending = self.pending_reprojection.take().unwrap_or_else(PendingReprojection::empty);
        if pending.local.is_some() {
            self.pending_reprojection = Some(pending);
            return Err(VcsError::HistoryReplaying);
        }
        drop(pending.replay.take());
        pending.local = Some(step);
        self.pending_reprojection = Some(pending);
        self.clock = clock;
        match self.advance_reprojection().await? {
            Some(ReprojectionAdvance::Adopted(_)) => self.bump().map(|()| None),
            Some(ReprojectionAdvance::Pending(_)) | None => Ok(None),
        }
    }

    /// 🧮️ One budget of the deferred reprojection: (re)starts its Report replay when the history it started from changed,
    /// steps it, and adopts the change with the finished replay; a change that needs no replay is adopted at once. A
    /// refused adoption drops the change — a refused local step only, its remote transitions wait again — and answers the
    /// refusal.
    async fn advance_reprojection(&mut self) -> Result<Option<ReprojectionAdvance>, VcsError> {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return Ok(None);
        };
        if pending.replay.as_ref().is_none_or(|(revision, _)| *revision != self.content_revision) {
            drop(pending.replay.take());
            let started = self.pending_fold(&pending).and_then(|fold| {
                if fold.supersessions == *self.supersessions && self.applied_edit_ids.starts_with(&fold.applied) {
                    return Ok(None);
                }
                let (from, horizon) = self.replay_window(&fold.applied, &fold.supersessions)?;
                self.report_replay(&fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len())).map(Some)
            });
            match started {
                Ok(Some(replay)) => pending.replay = Some((self.content_revision, replay)),
                Ok(None) => return self.adopt_pending(pending, None).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed))),
                Err(error) => {
                    self.refuse_local_step(pending);
                    return Err(error);
                }
            }
        }
        let budget = pending.local.as_ref().and(self.local_replay_budget).or(self.replay_budget).unwrap_or(usize::MAX);
        let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
        let mut left = budget;
        let step = replay.step(&self.envelope.vcs.edits, &mut || {
            left = left.saturating_sub(1);
            left == 0
        });
        match step {
            Ok(ReplayStep::Pending(progress)) => {
                self.pending_reprojection = Some(pending);
                Ok(Some(ReprojectionAdvance::Pending(progress)))
            }
            Ok(ReplayStep::Finished(_)) => match pending.replay.take().expect("a finished deferred replay").1.finish() {
                Ok(result) => self.adopt_pending(pending, Some(result)).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed))),
                Err(error) => {
                    self.refuse_local_step(pending);
                    Err(error)
                }
            },
            Err(error) => {
                self.refuse_local_step(pending);
                Err(error)
            }
        }
    }

    /// 🔮️ The history fold of the event log with a waiting change joined — its remote transitions, and its local step's
    /// transitions at the head that step moves to — touching nothing.
    fn pending_fold(&self, pending: &PendingReprojection<P, Mutation>) -> Result<crate::os_spr::HistoryFold, VcsError> {
        let local = pending.local.as_ref();
        let candidates: Vec<crate::os_spr::MutationEnvelope> = pending.transitions.iter().chain(local.map_or(&[][..], |local| local.envelopes.as_slice())).cloned().collect();
        let head = local.map_or_else(|| envelope_viewer_head(&self.envelope), |local| self.viewer_head(&local.head));
        self.prospective_fold_at(&candidates, &head)
    }

    /// 🚫️ Drops a refused waiting change: its local step is refused with nothing of it recorded, and its remote transitions
    /// wait again (their replay restarts) — or, refused themselves, are dropped.
    fn refuse_local_step(&mut self, mut pending: PendingReprojection<P, Mutation>) {
        drop(pending.replay.take());
        if pending.local.take().is_some() && !pending.transitions.is_empty() {
            self.pending_reprojection = Some(pending);
        }
    }

    /// 🧷️ Adopts a waiting change with its finished replay (`None`: it needs none): its remote transitions alone through
    /// [`Self::adopt_remote_transitions`]; with a local step, both are recorded in the log, this replica moves to the step's
    /// head, the projection follows and the step's transitions are announced. A finalizing step whose report blocks is
    /// refused (`Rejected { policy: Normal }`, every replay message) with nothing of it recorded while its remote transitions
    /// wait again. Atomic: a refused adoption keeps the log and the head.
    async fn adopt_pending(&mut self, pending: PendingReprojection<P, Mutation>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let PendingReprojection { transitions, local, replay } = pending;
        drop(replay);
        let Some(local) = local else {
            return self.adopt_remote_transitions(transitions, replayed).await;
        };
        let report = match replayed.as_ref().filter(|_| local.finalizing).map(|result| self.replay_report(result)).transpose() {
            Ok(report) => report,
            Err(error) => {
                drop(replayed);
                self.refuse_local_step(PendingReprojection { transitions, local: Some(local), replay: None });
                return Err(error);
            }
        };
        if let Some(report) = report.as_ref().filter(|report| report.blocks_finalize()) {
            drop(replayed);
            let messages = report.outcomes.iter().flat_map(|outcome| outcome.messages.iter().cloned()).collect();
            self.refuse_local_step(PendingReprojection { transitions, local: Some(local), replay: None });
            return Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages });
        }
        let PendingLocalStep { envelopes, head, .. } = local;
        let saved = self.viewer_head_now();
        let admitted: Vec<MutationId> = transitions.iter().chain(envelopes.iter()).map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in transitions.into_iter().chain(envelopes.iter().cloned()) {
            self.insert_transition(envelope);
        }
        (self.envelope.active_alternative_id, self.envelope.viewer_checkpoint_id) = head;
        let projected = match replayed {
            Some(result) => self.reproject_replayed(result),
            None => self.reproject().await,
        };
        let replayed = match projected {
            Ok(replayed) => replayed,
            Err(error) => {
                self.envelope.transitions.retain(|known| !admitted.contains(&known.mutation_id));
                (self.envelope.active_alternative_id, self.envelope.viewer_checkpoint_id) = saved;
                return Err(error);
            }
        };
        for envelope in &envelopes {
            self.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        }
        if let Some(report) = report {
            self.record_replay_report(&report);
        }
        self.pending_report.outbound.extend(envelopes);
        self.flush_outbound().await?;
        Ok(replayed)
    }
    //#endregion 🔖️DeferredReprojection"""),
# 10. supersede_command / create_alternative_with_supersede map
("""        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(scope, inputs, target, None).await.map(|_| ())
    }""",
"""        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(scope, inputs, target, None).await
    }"""),
("""        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(SupersedeScope::NewAlternative(name), inputs, target, None).await.map(|_| ())
    }""",
"""        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(SupersedeScope::NewAlternative(name), inputs, target, None).await
    }"""),
# 11. commit_finished_replay gate
("""        self.ensure_durable_group_idle()?;
        if let Some(open) = self.envelope.open_transaction.as_ref() {
            return Err(VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() });
        }
        if finished.generation != self.generation || finished.revision != self.content_revision {""",
"""        self.ensure_durable_group_idle()?;
        if let Some(open) = self.envelope.open_transaction.as_ref() {
            return Err(VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() });
        }
        if self.local_step_pending() {
            return Err(VcsError::HistoryReplaying);
        }
        if finished.generation != self.generation || finished.revision != self.content_revision {"""),
# 12. author_supersession doc + signature + deferral + return
("""    /// ✍️ The one authoring path of every supersession: builds its transitions (`Document`/`Alternative`: one `Supersede`;
    /// `NewAlternative`: the pending-edit commit, a `Branch` at the head and the scoped `Supersede`), takes `replayed` when it
    /// is the Report replay of exactly the resulting log and dry-runs it otherwise, refuses a report that blocks finalizing
    /// (`Error`/`Fatal`, the hub check-in strictness) with `Rejected { policy: Normal }` and every replay message, and installs
    /// everything atomically without replaying again.
    async fn author_supersession(&mut self, scope: SupersedeScope, inputs: Vec<protocol::SupersededInput>, target: Vec<String>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<protocol::ReplayReport, VcsError> {""",
"""    /// ✍️ The one authoring path of every supersession: builds its transitions (`Document`/`Alternative`: one `Supersede`;
    /// `NewAlternative`: the pending-edit commit, a `Branch` at the head and the scoped `Supersede`), takes `replayed` when it
    /// is the Report replay of exactly the resulting log and dry-runs it otherwise (under [`Self::defer_local_replays`] the
    /// dry run is a local step stepped across turns), refuses a report that blocks finalizing (`Error`/`Fatal`, the hub
    /// check-in strictness) with `Rejected { policy: Normal }` and every replay message, and installs everything atomically
    /// without replaying again.
    async fn author_supersession(&mut self, scope: SupersedeScope, inputs: Vec<protocol::SupersededInput>, target: Vec<String>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<(), VcsError> {"""),
("""        let result = match replayed {
            Some(result) if result.order == fold.applied && same_effective_inputs(&result.supersessions, &fold.supersessions) => result,
            other => {
                drop(other);
                match self.dry_run(&fold) {""",
"""        let result = match replayed {
            Some(result) if result.order == fold.applied && same_effective_inputs(&result.supersessions, &fold.supersessions) => result,
            other if self.local_replay_budget.is_some() => {
                drop((other, fold));
                let head = self.viewer_head_now();
                restore_head(self);
                return self.defer_local_step(PendingLocalStep { envelopes, head, finalizing: true }, clock).await.map(|_| ());
            }
            other => {
                drop(other);
                match self.dry_run(&fold) {"""),
("""        if let Err(error) = self.install_transitions(envelopes, clock, Some(result)).await {
            restore_head(self);
            return Err(error);
        }
        self.record_replay_report(&report);
        Ok(report)
    }""",
"""        if let Err(error) = self.install_transitions(envelopes, clock, Some(result)).await {
            restore_head(self);
            return Err(error);
        }
        self.record_replay_report(&report);
        Ok(())
    }"""),
# 13. prospective_fold -> _at
("""    /// 🔮️ The history fold of the event log with `candidates` joined, touching nothing.
    fn prospective_fold(&self, candidates: &[crate::os_spr::MutationEnvelope]) -> Result<crate::os_spr::HistoryFold, VcsError> {
        let mut transitions = self.envelope.transitions.clone();
        transitions.extend(candidates.iter().cloned());
        let head = envelope_viewer_head(&self.envelope);
        fold_event_log::<P, Mutation>(&self.envelope.id, &self.envelope.vcs.edits.iter().collect::<Vec<_>>(), &transitions, &self.envelope.conflicts, &head)
    }

    /// 🧪️ The Report replay of everything `fold` changes against the live projection, driven to completion, touching nothing.
    /// Synchronous by budget: one actor turn replays the applied history in place. A longer history still replays here;
    /// a resumable replay is a separate gap.
    fn dry_run(""",
"""    /// 🔮️ The history fold of the event log with `candidates` joined, touching nothing.
    fn prospective_fold(&self, candidates: &[crate::os_spr::MutationEnvelope]) -> Result<crate::os_spr::HistoryFold, VcsError> {
        self.prospective_fold_at(candidates, &envelope_viewer_head(&self.envelope))
    }

    /// 🔮️ [`Self::prospective_fold`] for `head` instead of this replica's head.
    fn prospective_fold_at(&self, candidates: &[crate::os_spr::MutationEnvelope], head: &crate::os_spr::ViewerHead) -> Result<crate::os_spr::HistoryFold, VcsError> {
        let mut transitions = self.envelope.transitions.clone();
        transitions.extend(candidates.iter().cloned());
        fold_event_log::<P, Mutation>(&self.envelope.id, &self.envelope.vcs.edits.iter().collect::<Vec<_>>(), &transitions, &self.envelope.conflicts, head)
    }

    /// 🧪️ The Report replay of everything `fold` changes against the live projection, driven to completion, touching nothing.
    /// Synchronous: the supersede command API (agents, text, hub) gets its verdict inside the dispatch. Under
    /// [`Self::defer_local_replays`] [`Self::author_supersession`] steps the same replay across turns instead.
    fn dry_run("""),
# 14. dispatch_inner gate
("""            if !admitted {
                let transaction_id = open.transaction.id.clone();
                retire_command::<P, Mutation>(command);
                return Err(VcsError::TransactionOpen { transaction_id });
            }
        }
        match command {""",
"""            if !admitted {
                let transaction_id = open.transaction.id.clone();
                retire_command::<P, Mutation>(command);
                return Err(VcsError::TransactionOpen { transaction_id });
            }
        }
        if command.moves_history() && self.local_step_pending() {
            retire_command::<P, Mutation>(command);
            return Err(VcsError::HistoryReplaying);
        }
        match command {"""),
]

for old, new in EDITS:
    count = source.count(old)
    if count != 1:
        sys.exit(f"anchor matched {count} times: {old[:120]!r}")
    source = source.replace(old, new)
open(path, "w", encoding="utf-8").write(source)
print("applied", len(EDITS))
