"""📡️ S2-W2A G9 runtime adoption (design §16.6): the document store defers remote reprojections, the time-travel driver steps
them per reactor turn with pause/resume through the session's cancel/rerun verbs, and the undo or redo of a finalize is
authored resumably (`begin_report_replay` + steps + `commit_finished_replay`)."""
import pathlib

P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs")
t = P.read_text()


def rep(a, b):
    global t
    if t.count(b) == 1 and t.count(a) == 0:
        return
    assert t.count(a) == 1, (t.count(a), a[:90])
    t = t.replace(a, b)


rep("""use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};""", """use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryRemoteReplay, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};""")
rep("""/// ⏱️ Wall budget of one replay slice per reactor turn.
pub const TIME_TRAVEL_TURN_WALL_US: u64 = 4_000;
""", """/// ⏱️ Wall budget of one replay slice per reactor turn.
pub const TIME_TRAVEL_TURN_WALL_US: u64 = 4_000;
/// 📡️ Operations one reactor turn replays of a remote history change before the document store adopts it (design §16.6,
/// `ArtifactStore::defer_remote_replays`).
pub const TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS: usize = 256;
""")
rep("""    refreshed_done: u32,
    patch_due: bool,
    patch: Option<HistoryPatch>,
}

impl<A: ArtifactApp> Default for TimeTravelLedger<A> {""", """    refreshed_done: u32,
    patch_due: bool,
    patch: Option<HistoryPatch>,
    remote_paused: bool,
    remote_fault: Option<String>,
    remote_refreshed_ms: u64,
    authoring: Option<SupersedeAuthoring<A::Snapshot, A::Mutation>>,
}

/// ✍️ The undo or redo of a finalize being authored resumably: the Report replay of its inputs over the applied history,
/// stepped per reactor turn, and how it finalizes (unscoped, or within the history edit's own line).
pub(crate) struct SupersedeAuthoring<P, Mu: ::protocol::Mutation<P>> {
    replay: store::EditReplay<P, Mu>,
    finalization: store::HistoryFinalization,
}

/// 🧮️ Where authoring the undo or redo of a finalize stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SupersedeAuthored {
    Landed,
    Pending,
    Refused,
    Busy,
}

impl<A: ArtifactApp> Default for TimeTravelLedger<A> {""")
rep("""            refreshed_done: 0,
            patch_due: false,
            patch: None,
        }
    }
}""", """            refreshed_done: 0,
            patch_due: false,
            patch: None,
            remote_paused: false,
            remote_fault: None,
            remote_refreshed_ms: 0,
            authoring: None,
        }
    }
}""")
rep("""    /// 🏃️ Whether a driver turn has work: a live replay, retirement, an owed UI scope or an owed history patch.
    pub fn has_pending_work(&self) -> bool {
        let replaying = self.session.stage == TimeTravelStage::Replaying && self.owners().any(TimeTravelOwners::replaying);
        replaying || self.owners().any(TimeTravelOwners::has_pending_work) || self.is_ui_dirty() || self.patch_due || self.patch.is_some()
    }""", """    /// 🏃️ Whether a driver turn has work: a live replay, an undo or redo of a finalize being authored, retirement, an owed
    /// UI scope or an owed history patch.
    pub fn has_pending_work(&self) -> bool {
        let replaying = self.session.stage == TimeTravelStage::Replaying && self.owners().any(TimeTravelOwners::replaying);
        replaying || self.authoring.is_some() || self.owners().any(TimeTravelOwners::has_pending_work) || self.is_ui_dirty() || self.patch_due || self.patch.is_some()
    }

    /// ⏸️ Whether the user paused the replay of a waiting remote history change.
    pub fn remote_replay_paused(&self) -> bool {
        self.remote_paused
    }""")
rep("""        self.replace_editor(None);
        self.ui_dirty = false;
        self.document_dirty = false;
        self.patch_due = false;
        self.patch = None;
    }

    pub fn terminal_is_empty(&self) -> bool {
        !self.is_active() && self.editor.is_none() && self.member.is_none() && self.owners().all(TimeTravelOwners::terminal_is_empty)
    }""", """        self.replace_editor(None);
        self.ui_dirty = false;
        self.document_dirty = false;
        self.patch_due = false;
        self.patch = None;
        self.remote_paused = false;
        self.remote_fault = None;
        self.authoring = None;
    }

    pub fn terminal_is_empty(&self) -> bool {
        !self.is_active() && self.editor.is_none() && self.member.is_none() && self.authoring.is_none() && self.owners().all(TimeTravelOwners::terminal_is_empty)
    }""")
rep("""    /// 🏃️ Ledger work, or a base change the watch has not delivered to an open session yet.
    pub(crate) fn time_travel_has_pending_work(&self) -> bool {
        self.time_travel.has_pending_work() || (self.time_travel.is_active() && self.time_travel_base() != Some(self.time_travel.session.base))
    }""", """    /// 🏃️ Ledger work, a remote history change the document store still replays (unless paused), or a base change the
    /// watch has not delivered to an open session yet.
    pub(crate) fn time_travel_has_pending_work(&self) -> bool {
        self.time_travel.has_pending_work() || (!self.time_travel.remote_paused && self.store.reprojection_progress().is_some()) || (self.time_travel.is_active() && self.time_travel_base() != Some(self.time_travel.session.base))
    }

    /// 📡️ The remote history change the document store replays before adopting it, as the history wire and body show it:
    /// its progress, whether the user paused it and the code of a refused adoption; `None` while none waits.
    pub(crate) fn remote_replay_status(&self) -> Option<HistoryRemoteReplay> {
        match (self.store.reprojection_progress(), self.time_travel.remote_fault.as_ref()) {
            (Some(progress), fault) => Some(HistoryRemoteReplay { done: progress.done, total: progress.total, paused: self.time_travel.remote_paused, fault: fault.cloned() }),
            (None, Some(fault)) => Some(HistoryRemoteReplay { done: 0, total: 0, paused: false, fault: Some(fault.clone()) }),
            (None, None) => None,
        }
    }

    /// ⏭️ One budget of the waiting remote history change (design §16.6): progress refreshes the history at most every
    /// [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; the adoption swaps the document in, delivers `BaseMoved` and refreshes every
    /// body; a refused adoption (the store drops the change) keeps its code on the remote row.
    async fn step_remote_replay(&mut self) -> Result<(), Fault> {
        let generation = self.store.generation();
        match self.store.step_reprojection().await {
            Ok(Some(_)) => {
                self.time_travel.remote_fault = None;
                let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
                if now_ms.saturating_sub(self.time_travel.remote_refreshed_ms) >= TIME_TRAVEL_PROGRESS_REFRESH_MS {
                    self.time_travel.remote_refreshed_ms = now_ms;
                    self.note_time_travel_changed(false, false);
                }
            }
            Ok(None) => {
                self.time_travel.remote_fault = None;
                if self.store.generation() != generation {
                    self.cache = None;
                    self.deliver_base_moved().await?;
                }
                self.note_time_travel_changed(true, true);
            }
            Err(error) => {
                self.time_travel.remote_fault = Some(error.into_fault().code.0);
                self.cache = None;
                self.note_time_travel_changed(true, true);
            }
        }
        Ok(())
    }

    /// ⏸️ `historyEditCancelReplay` while no session is open: drops the running replay of the waiting remote history change
    /// (the store keeps the change) and stops driving it until `historyEditRerun`.
    fn pause_remote_replay(&mut self) -> TimeTravelActionOutcome {
        if self.time_travel.remote_paused || self.store.reprojection_progress().is_none() {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.store.cancel_reprojection();
        self.time_travel.remote_paused = true;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }

    /// ▶️ `historyEditRerun` while no session is open: drives the paused remote replay again from its start.
    fn resume_remote_replay(&mut self) -> TimeTravelActionOutcome {
        if !self.time_travel.remote_paused {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.time_travel.remote_paused = false;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }""")
rep("""    pub async fn apply_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        self.watch_time_travel_base().await?;
        let generation""", """    pub async fn apply_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        self.watch_time_travel_base().await?;
        if !self.time_travel.is_active() {
            match action {
                HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID => return Ok(self.pause_remote_replay()),
                HISTORY_EDIT_RERUN_ACTION_ID => return Ok(self.resume_remote_replay()),
                _ => {}
            }
        }
        let generation""")
rep("""        if self.tool_runs.holds_mutating_run() || self.pending_transaction.is_some() || self.store.open_transaction().is_some() {""", """        if self.tool_runs.holds_mutating_run() || self.pending_transaction.is_some() || self.store.open_transaction().is_some() || self.time_travel.authoring.is_some() {""")
rep("""        let moved = self.watch_time_travel_base().await?;
        if !moved && self.time_travel.session.stage == TimeTravelStage::Replaying && self.time_travel.replaying() {
            self.step_time_travel_replay().await?;
        }
""", """        let moved = self.watch_time_travel_base().await?;
        if !moved && self.time_travel.session.stage == TimeTravelStage::Replaying && self.time_travel.replaying() {
            self.step_time_travel_replay().await?;
        }
        if self.time_travel.authoring.is_some() && self.step_supersede_authoring().await? != SupersedeAuthored::Pending {
            self.note_time_travel_changed(true, true);
        }
        if !self.time_travel.remote_paused && self.store.reprojection_progress().is_some() {
            self.step_remote_replay().await?;
        }
""")
start = t.index("""    /// ✍️ Authors `inputs` as one `Supersede` in `scope` through the store's one supersede path — the restore of an undo""") if "SupersedeAuthored::Busy);" not in t else None
if start is not None:
    end = t.index("""    /// ⏪️⏩️ A plain `undo` or `redo` whose newest own target is a history edit""")
    t = t[:start] + """    /// ✍️ Authors `inputs` within `scope` (unscoped: every line) resumably (design §16.6) — the restore of an undo or the
    /// re-authoring of a redo: a Report replay of the inputs over the applied history, stepped within this turn's budget and
    /// then per reactor turn, finalized by `commit_finished_replay` without replaying again. Answers whether it landed, is
    /// still replaying, was refused (an input no longer applied, or a report that blocks finalizing; nothing changes) or
    /// waits for an authoring already running.
    async fn author_supersede(&mut self, scope: Option<String>, inputs: Vec<protocol::SupersededInput>) -> Result<SupersedeAuthored, Fault> {
        if self.time_travel.authoring.is_some() {
            return Ok(SupersedeAuthored::Busy);
        }
        let drafts: BTreeMap<MutationId, protocol::InputReplacement> = inputs.into_iter().map(|input| (input.target, input.replacement)).collect();
        let finalization = match scope {
            Some(alternative_id) => store::HistoryFinalization::Scope { alternative_id },
            None => store::HistoryFinalization::Overwrite,
        };
        match self.store.begin_report_replay(&drafts, None) {
            Ok(replay) => self.time_travel.authoring = Some(SupersedeAuthoring { replay, finalization }),
            Err(vcs::VcsError::ValidationFailed(_) | vcs::VcsError::Rejected { .. }) => return Ok(SupersedeAuthored::Refused),
            Err(error) => return Err(error.into_fault()),
        }
        self.step_supersede_authoring().await
    }

    /// ⏭️ One turn of the undo or redo of a finalize being authored: steps its replay until the turn deadline, then commits
    /// the finished replay in its scope; a store that moved meanwhile replays the same inputs again from the new base.
    async fn step_supersede_authoring(&mut self) -> Result<SupersedeAuthored, Fault> {
        let deadline_us = semio_framework_job::default_now_us().unwrap_or(0).saturating_add(TIME_TRAVEL_TURN_WALL_US);
        let stepped = {
            let VcsArtifactApp { store, time_travel, .. } = self;
            let Some(authoring) = time_travel.authoring.as_mut() else { return Ok(SupersedeAuthored::Refused) };
            let mut deadline = || semio_framework_job::default_now_us().is_none_or(|now| now >= deadline_us);
            authoring.replay.step(store.replay_edits(), &mut deadline)
        };
        match stepped {
            Ok(store::ReplayStep::Pending(_)) => return Ok(SupersedeAuthored::Pending),
            Ok(store::ReplayStep::Finished(_)) => {}
            Err(_) => {
                self.time_travel.authoring = None;
                return Ok(SupersedeAuthored::Refused);
            }
        }
        let SupersedeAuthoring { replay, finalization } = self.time_travel.authoring.take().expect("a finished authoring replay was held");
        let Ok(result) = replay.finish() else { return Ok(SupersedeAuthored::Refused) };
        let drafts = result.drafts().clone();
        match self.store.commit_finished_replay(result, finalization.clone()).await {
            Ok(_) => {
                self.cache = None;
                Ok(SupersedeAuthored::Landed)
            }
            Err(vcs::VcsError::Stale { .. }) => match self.store.begin_report_replay(&drafts, None) {
                Ok(replay) => {
                    self.time_travel.authoring = Some(SupersedeAuthoring { replay, finalization });
                    Ok(SupersedeAuthored::Pending)
                }
                Err(_) => Ok(SupersedeAuthored::Refused),
            },
            Err(vcs::VcsError::ValidationFailed(_) | vcs::VcsError::Rejected { .. } | vcs::VcsError::UnknownAlternative(_)) => Ok(SupersedeAuthored::Refused),
            Err(error) => Err(error.into_fault()),
        }
    }

""" + t[end:]
rep("""        let Some((scope, inputs)) = authored.filter(|(_, inputs)| !inputs.is_empty()) else { return Ok(None) };
        let landed = self.author_supersede(scope, inputs).await?;
        let (events, scope) = if landed { (vec![history_changed_event().await], UiDirtyScope::Full) } else { (Vec::new(), UiDirtyScope::None) };
        Ok(Some(Self::empty_result(action, meta, Vec::new(), events, scope).await))
    }""", """        let Some((scope, inputs)) = authored.filter(|(_, inputs)| !inputs.is_empty()) else { return Ok(None) };
        let authored = self.author_supersede(scope, inputs).await?;
        Ok(Some(self.supersede_authored_result(action, meta, authored).await))
    }

    /// 🧾️ The result of a verb that authored the undo or redo of a finalize: a landed one changed the history (every body
    /// refreshes), a pending one lands in a later reactor turn, an authoring already running answers `timeTravel.busy`.
    async fn supersede_authored_result(&mut self, action: &str, meta: &ActionMeta, authored: SupersedeAuthored) -> InvocationResult {
        let (events, scope) = match authored {
            SupersedeAuthored::Landed => (vec![history_changed_event().await], UiDirtyScope::Full),
            SupersedeAuthored::Pending | SupersedeAuthored::Refused | SupersedeAuthored::Busy => (Vec::new(), UiDirtyScope::None),
        };
        let mut result = Self::empty_result(action, meta, Vec::new(), events, scope).await;
        if authored == SupersedeAuthored::Busy {
            result.output = DslValue::Object(vec![("rejected".into(), DslValue::String(TIME_TRAVEL_BUSY_CODE.into()))]);
        }
        result
    }""")
rep("""        let landed = match target.map(|entry| self.supersedes.restore(entry)).filter(|(_, inputs)| !inputs.is_empty()) {
            Some((scope, inputs)) => self.author_supersede(scope, inputs).await?,
            None => false,
        };
        let (events, scope) = if landed { (vec![history_changed_event().await], UiDirtyScope::Full) } else { (Vec::new(), UiDirtyScope::None) };
        Ok(Self::empty_result(REVERT_TO_COMMAND_ACTION_ID, meta, Vec::new(), events, scope).await)
    }""", """        let authored = match target.map(|entry| self.supersedes.restore(entry)).filter(|(_, inputs)| !inputs.is_empty()) {
            Some((scope, inputs)) => self.author_supersede(scope, inputs).await?,
            None => SupersedeAuthored::Refused,
        };
        Ok(self.supersede_authored_result(REVERT_TO_COMMAND_ACTION_ID, meta, authored).await)
    }""")
P.write_text(t)
print("ok")
