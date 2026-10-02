"""🐢️ W1-G session 2: G9 / design §16.6 — the Report replay of a remote history change becomes a resumable job (deferred
reprojection) with progress and cancel, stepped per turn instead of inside the ingest. One atomic, count-asserted rewrite.
"""
import pathlib
import re
import sys

STORE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs")

PENDING_STRUCT_ANCHOR = """pub struct OpenToolTransaction {
    pub transaction: protocol::TransactionRef,
    pub edit_id: String,
}
"""
PENDING_STRUCT = PENDING_STRUCT_ANCHOR + """
/// 🐢️ A remote history change this store admitted but has not adopted yet ([`ArtifactStore::defer_remote_replays`]): its
/// transitions wait outside the log while their Report replay steps turn by turn, and the replay restarts whenever the
/// history it started from changed (the content revision it names).
struct PendingReprojection<P, Mutation> {
    transitions: Vec<crate::os_spr::MutationEnvelope>,
    replay: Option<([u8; 32], EditReplay<P, Mutation>)>,
}

/// ⏩️ Where one advance of a deferred reprojection left it.
enum ReprojectionAdvance {
    Pending(ReplayProgress),
    Adopted(Vec<crate::os_spr::EditMessages>),
}
"""

FIELDS_OLD = "    authoring_verb: Option<String>,\n    pending_report:"
FIELDS_NEW = (
    "    authoring_verb: Option<String>,\n"
    "    /// 🐢️ Operations one turn of a deferred remote reprojection replays; `None` replays inside the ingest.\n"
    "    replay_budget: Option<usize>,\n"
    "    /// 🐢️ The remote history change admitted but not adopted yet ([`Self::step_reprojection`]).\n"
    "    pending_reprojection: Option<PendingReprojection<P, Mutation>>,\n"
    "    pending_report:"
)

ADMIT_OLD = """    async fn admit_remote_transitions(&mut self, transitions: Vec<crate::os_spr::MutationEnvelope>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let mut admitted: Vec<MutationId> = Vec::new();
        for envelope in transitions {
            if let Err(error) = self.admit_history_shape(std::slice::from_ref(&envelope)) {
                self.envelope.transitions.retain(|known| !admitted.contains(&known.mutation_id));
                return Err(error);
            }
            if let Some(known) = self.envelope.transitions.iter().find(|known| known.mutation_id == envelope.mutation_id) {
                if !Self::same_operation_identity_and_payload(known, &envelope) {
                    return Err(VcsError::ValidationFailed(format!("remote transition {} conflicts with its established payload", envelope.mutation_id.0)));
                }
                continue;
            }
            self.clock.merge(&envelope.timestamp);
            admitted.push(envelope.mutation_id.clone());
            self.insert_transition(envelope);
        }
        match self.reproject().await {
            Ok(replayed) => Ok(replayed),
            Err(error) => {
                self.envelope.transitions.retain(|known| !admitted.contains(&known.mutation_id));
                Err(error)
            }
        }
    }
"""
ADMIT_NEW = """    async fn admit_remote_transitions(&mut self, transitions: Vec<crate::os_spr::MutationEnvelope>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let mut fresh: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
        for envelope in transitions {
            self.admit_history_shape(std::slice::from_ref(&envelope))?;
            let pending = self.pending_reprojection.as_ref().map_or(&[][..], |pending| pending.transitions.as_slice());
            if let Some(known) = self.envelope.transitions.iter().chain(pending).chain(fresh.iter()).find(|known| known.mutation_id == envelope.mutation_id) {
                if !Self::same_operation_identity_and_payload(known, &envelope) {
                    return Err(VcsError::ValidationFailed(format!("remote transition {} conflicts with its established payload", envelope.mutation_id.0)));
                }
                continue;
            }
            self.clock.merge(&envelope.timestamp);
            fresh.push(envelope);
        }
        if self.pending_reprojection.is_none() && (self.replay_budget.is_none() || fresh.is_empty()) {
            return self.adopt_remote_transitions(fresh, None).await;
        }
        let mut transitions = self.pending_reprojection.as_ref().map(|pending| pending.transitions.clone()).unwrap_or_default();
        transitions.extend(fresh);
        self.prospective_fold(&transitions)?;
        self.pending_reprojection = Some(PendingReprojection { transitions, replay: None });
        match self.advance_reprojection().await? {
            Some(ReprojectionAdvance::Adopted(replayed)) => Ok(replayed),
            Some(ReprojectionAdvance::Pending(_)) | None => Ok(Vec::new()),
        }
    }

    /// 📥️ Records admitted remote transitions in the log and materializes the projection they fold to — adopting `replayed`
    /// when it is their finished Report replay — or records none of them.
    async fn adopt_remote_transitions(&mut self, transitions: Vec<crate::os_spr::MutationEnvelope>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let admitted: Vec<MutationId> = transitions.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in transitions {
            self.insert_transition(envelope);
        }
        let projected = match replayed {
            Some(result) => self.reproject_replayed(result),
            None => self.reproject().await,
        };
        projected.inspect_err(|_| self.envelope.transitions.retain(|known| !admitted.contains(&known.mutation_id)))
    }
"""

TICK_OLD = """    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.ensure_durable_group_idle()?;
        self.pump().await
    }
"""
TICK_NEW = """    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.ensure_durable_group_idle()?;
        let pumped = self.pump().await?;
        let pending = self.pending_reprojection.is_some();
        let progress = self.step_reprojection().await?;
        Ok(pumped || (pending && progress.is_none()))
    }
"""

REGION_ANCHOR = "    //#endregion 🔖️ToolTransactions\n"
REGION = REGION_ANCHOR + """
    //#region 🔖️DeferredReprojection
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
    //#endregion 🔖️DeferredReprojection
"""


def main() -> None:
    text = STORE.read_text()
    for label, old, new in [("pending struct", PENDING_STRUCT_ANCHOR, PENDING_STRUCT), ("fields", FIELDS_OLD, FIELDS_NEW), ("admit", ADMIT_OLD, ADMIT_NEW), ("tick", TICK_OLD, TICK_NEW), ("region", REGION_ANCHOR, REGION)]:
        count = text.count(old)
        if count != 1:
            sys.exit(f"[{label}] anchor matched {count}x, expected 1")
        text = text.replace(old, new)
    text, inits = re.subn(r"(\n(\s*)authoring_verb: None,)", r"\1\n\2replay_budget: None,\n\2pending_reprojection: None,", text)
    if inits != 2:
        sys.exit(f"[ctor] matched {inits}x, expected 2")
    if "--write" in sys.argv:
        STORE.write_text(text)
        print("written")
    else:
        print("dry run ok")


main()
