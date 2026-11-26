//! 🧳️ The inference sessions of a mounted instance. The framework owns them: one `ModelInferenceSession` per mounted artifact instance lives in the instance's `ArtifactInstanceOperationOwnerHandle` (see `with_inference_session`),
//! created on first use, dropped by the close ladder with the instance and never held by a process or thread global. This module only supplies what the BIM model adds: the session type, the names of its two sessions
//! (the document and the probe) and the calls every window, gesture, export and job reads derived values through. A render after a mutation brings its instance's session up to the snapshot from the concrete diffs the editor
//! recorded ([`record_mutations`]); a change nobody explains (undo, a peer's edit, a load) walks the plan and the cache serves every node whose dependency chain is unchanged. A job that must not block steps the same session
//! ([`begin`], [`step`], [`finish`]) with progress and cancellation. A probe (a hypothetical model such as a room under the pointer) runs on a session of its own so the document's state is never touched. A call without an
//! instance (`None`: a one-shot export, a fixture) works on a throwaway session.
//!
//! An engine fault never shows as an empty model: a read keeps the last complete inference and adds the finding [`DiagnosticCode::InferenceFault`], which the diagnostics panel and the problem counters show in English and German
//! ([`shown`]); the calls whose caller can refuse (`try_*`, exports, jobs) hand the fault over as a value.

use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, ModelInferenceSession, RunProgress, SessionRun, UpdateReport};
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::{ordered, Diagnostic, DiagnosticCode, DiagnosticIndex};
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::SpaceRoom;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{ColdMutation, ColdMutationDiff, InferenceError, InferenceWorkFailure};
use semio_plugin::ArtifactInstanceOperationOwnerHandle;
use std::borrow::Cow;
use std::collections::BTreeMap;

//#region 🔖️Sessions
/// 🧳️ The mounted instance a call works for: its handle, or `None` for a one-shot call that keeps nothing.
pub type Instance<'a> = Option<&'a ArtifactInstanceOperationOwnerHandle>;

/// 🔮️ The name of the document's session in the instance.
pub const DOCUMENT: &str = "bim.model-graph";
/// 🔭️ The name of the probe session in the instance.
pub const PROBE: &str = "bim.model-graph.probe";
const PREVIEW: &str = "bim.model-graph.preview";

/// 🧠️ Lends the session `name` of `handle` to `act`, creating it on first use. An instance whose close ladder already dropped its sessions serves `act` a throwaway session instead.
pub fn lend_handle<R>(handle: &ArtifactInstanceOperationOwnerHandle, name: &'static str, act: impl FnOnce(&mut ModelInferenceSession) -> R) -> R {
    let mut act = Some(act);
    match handle.with_inference_session_cold(name, ModelInferenceSession::default, |session| Ok(act.take().map(|act| act(session)))) {
        Ok(Some(out)) => out,
        _ => act.take().map_or_else(|| unreachable!("the session closure ran without producing a value"), |act| act(&mut ModelInferenceSession::default())),
    }
}

/// 🧠️ Lends the session `name` of `instance` to `act`; a call without an instance works on a throwaway session.
fn lend<R>(instance: Instance<'_>, name: &'static str, act: impl FnOnce(&mut ModelInferenceSession) -> R) -> R {
    match instance {
        Some(handle) => lend_handle(handle, name, act),
        None => act(&mut ModelInferenceSession::default()),
    }
}

/// 🚨️ What a read shows when the run ended with `result`: the inference of the run, or on a fault the last complete inference with the finding [`DiagnosticCode::InferenceFault`] added, never an empty model.
pub fn shown<'a>(session: &'a ModelInferenceSession, result: Result<(), InferenceError>) -> Cow<'a, ModelInference> {
    match result {
        Ok(()) => Cow::Borrowed(session.inference()),
        Err(error) => {
            let mut last = session.inference().clone();
            last.diagnostics.push(Diagnostic::new(DiagnosticCode::InferenceFault, &[]).lacking(&error.to_string()));
            last.diagnostics = ordered(last.diagnostics);
            last.diagnostic_index = DiagnosticIndex::of(&last.diagnostics);
            Cow::Owned(last)
        }
    }
}

/// 💡️ Reads the inference of `snapshot` for one mounted instance, bringing that instance's session up to `snapshot` first. A run that fails reads as the last complete inference plus the fault finding (see [`shown`]).
pub fn with_inference<R>(instance: Instance<'_>, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> R {
    lend(instance, DOCUMENT, |session| {
        let result = session.try_sync(snapshot).map(|_| ());
        read(&shown(session, result))
    })
}

/// 🪟️ Associates the mounted view's authenticated document revision with its completed inference.
pub fn with_view_inference<R>(instance: Instance<'_>, doc: &semio_plugin::ArtifactView<'_, ModelSnapshot>, read: impl FnOnce(&ModelInference) -> R) -> R {
    let Some(operation) = doc.committed_render_operation() else {
        return lend(instance, PREVIEW, |session| {
            let result = session.try_sync(doc.snapshot).map(|_| ());
            read(&shown(session, result))
        });
    };
    lend(instance, DOCUMENT, |session| {
        let result = session.try_sync(doc.snapshot).map(|_| ());
        if result.is_ok() {
            session.record_rendered_revision(operation.canonical_base_revision);
        }
        read(&shown(session, result))
    })
}

/// 💡️ [`with_inference`] whose run failure is a value: a fault of the engine is returned instead of shown.
pub fn try_with_inference<R>(instance: Instance<'_>, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> Result<R, InferenceError> {
    lend(instance, DOCUMENT, |session| session.try_sync(snapshot).map(read))
}

/// 📝️ Records the concrete diffs of `mutations` the editor emits against `snapshot`, in order, for the instance's next sync. Each mutation's diff is read against the state the ones
/// before it leave; a mutation whose diff does not apply records nothing and the next sync walks the plan.
pub fn record_mutations<Op: ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff>>(instance: Instance<'_>, snapshot: &ModelSnapshot, mutations: &[Op]) {
    let Some(handle) = instance else { return };
    let mut sum = ModelDiff::default();
    let mut state: Option<ModelSnapshot> = None;
    for (index, mutation) in mutations.iter().enumerate() {
        let base = state.as_ref().unwrap_or(snapshot);
        let (diff, _) = mutation.diff(base).into_parts();
        if index + 1 < mutations.len() {
            match protocol::apply_diff_cold(&diff, base) {
                Ok(next) => state = Some(next),
                Err(_) => return,
            }
        }
        sum.absorb(diff);
    }
    lend_handle(handle, DOCUMENT, |session| session.record(sum));
}

/// 📊️ What the last run of one instance's session did: the proof an edit recomputed only what it touched.
pub fn report(instance: Instance<'_>) -> UpdateReport {
    lend(instance, DOCUMENT, |session| session.report().clone())
}

/// 🔭️ Reads what the kinds `WANT` select of a probe model (a model that is not the document: a room under the pointer, an import in progress) on the probe session of the instance, whose cache serves every node the probe leaves alone; the document's session is never touched.
pub fn probe<const WANT: u64, R>(instance: Instance<'_>, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> Result<R, InferenceError> {
    lend(instance, PROBE, |session| session.probe::<WANT>(snapshot).map(|inference| read(&inference)))
}

/// 🏠️ The rooms of a probe model (the document plus a hypothetical space).
pub fn probe_rooms(instance: Instance<'_>, snapshot: &ModelSnapshot) -> BTreeMap<String, SpaceRoom> {
    probe::<{ kinds::ROOMS }, _>(instance, snapshot, |inference| inference.spaces.clone()).unwrap_or_default()
}
//#endregion 🔖️Sessions

//#region 🔖️Runs
/// 🚪️ Opens the run that brings the instance's session to `snapshot`, for a job that steps it.
pub fn begin(instance: Instance<'_>, snapshot: &ModelSnapshot) -> SessionRun {
    lend(instance, DOCUMENT, |session| session.begin_sync(snapshot))
}

/// ⏩️ Spends at most `fuel` node computes of `run`; the progress it answers never decreases.
pub fn step(instance:&ArtifactInstanceOperationOwnerHandle,run:&mut SessionRun,snapshot:&ModelSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<RunProgress,InferenceWorkFailure>{
    let mut access=std::mem::take(&mut run.instance_access);
    let result=instance.with_inference_session(DOCUMENT,&mut access,cx,ModelInferenceSession::new_funded,|session,cx|session.step(run,snapshot,cx));
    run.instance_access=access;
    result.map_err(InferenceWorkFailure::from)?.unwrap_or_else(||Ok(run.progress()))
}

/// 🏁️ Ends `run`: a finished run settles the instance's session at `snapshot`, a cancelled or failed one leaves it as it was with every finished node cached.
pub fn finish(instance:&ArtifactInstanceOperationOwnerHandle,run:&mut SessionRun,snapshot:&ModelSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<()>,InferenceWorkFailure>{
    let mut access=std::mem::take(&mut run.instance_access);
    let result=instance.with_inference_session(DOCUMENT,&mut access,cx,ModelInferenceSession::new_funded,|session,cx|session.finish(run,snapshot,cx).map(|_|()));
    run.instance_access=access;
    result.map_err(InferenceWorkFailure::from)?.transpose()
}
//#endregion 🔖️Runs

//#region 🔖️Analysis
/// 🎟️ The original caller's cursor for the mounted inference owner.
pub type MountedAnalysisAccess = semio_plugin::ArtifactInferenceSessionAccess<ModelInferenceSession>;

/// 🕳️ A read of an exact revision that no committed view has published.
pub const PROJECTION_UNAVAILABLE: &str = "BIM inference has no completed render for this document revision";

/// 🧵️ A stepped, cancellable bring-up of one instance's session to a snapshot: what a job holds between its steps. The first [`advance`](Analysis::advance) opens the run, every later one spends the
/// fuel on it, the one that finishes it settles the session. [`cancel`](Analysis::cancel) closes the run as cancelled, so the nodes it finished stay cached for the next run. An analysis without an instance
/// settles a session of its own, so its runs belong to one session all the way.
pub struct Analysis {
    access:MountedAnalysisAccess,
    instance: Option<ArtifactInstanceOperationOwnerHandle>,
    local: Option<ModelInferenceSession>,
    run: Option<SessionRun>,
    rendered_revision: Option<[u8; 32]>,
    done: bool,
}

impl Analysis {
    /// 🏗️ An analysis of the document that `instance` shows.
    pub fn new(instance: Instance<'_>) -> Self {
        Self { access:Default::default(),instance: instance.cloned(), local: None, run: None, rendered_revision: None, done: false }
    }

    /// 🪪️ Uses an already rendered projection only for the admitted command's exact document revision.
    pub fn for_rendered_revision(instance: &ArtifactInstanceOperationOwnerHandle, revision: [u8; 32]) -> Self {
        Self { rendered_revision: Some(revision), ..Self::new(Some(instance)) }
    }

    /// 📖️ Lends the completed projection for this exact revision under the original caller.
    pub fn read_rendered<R>(
        instance: &ArtifactInstanceOperationOwnerHandle,
        access: &mut MountedAnalysisAccess,
        revision: [u8; 32],
        cx: &mut semio_framework_job::StepContext<'_>,
        read: impl FnOnce(&ModelInference, &mut semio_framework_job::StepContext<'_>) -> R,
    ) -> Result<Option<R>, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let unavailable = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, PROJECTION_UNAVAILABLE);
        instance.with_inference_session(DOCUMENT, access, cx, |_| Err(unavailable().into()), |session, cx| {
            if !session.has_rendered_revision(revision) {
                return Err(unavailable().into());
            }
            Ok(read(session.inference(), cx))
        })?.transpose()
    }

    /// ✅️ Whether the session is settled at the snapshot.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// 📈️ How far the run is, as a fraction of its plan; it never decreases.
    pub fn fraction(&self) -> f32 {
        if self.done { 1.0 } else { self.run.as_ref().map_or(0.0, |run| run.progress().fraction) }
    }

    fn session<R>(&mut self,cx:&mut semio_framework_job::StepContext<'_>,act:impl FnOnce(&mut ModelInferenceSession,&mut Option<SessionRun>,&mut semio_framework_job::StepContext<'_>)->R)->Result<Option<R>,InferenceWorkFailure> {
        let Self { access,instance, local, run, .. } = self;
        match instance {
            Some(handle)=>handle.with_inference_session(DOCUMENT,access,cx,ModelInferenceSession::new_funded,|session,cx|act(session,run,cx)).map_err(InferenceWorkFailure::from),
            None=>{if local.is_none(){*local=ModelInferenceSession::new_funded(cx)?;return Ok(None)}Ok(Some(act(local.as_mut().unwrap(),run,cx)))},
        }
    }

    /// ⏩️ Does one bounded step against `snapshot`; a run that fails or finishes is closed here.
    pub fn advance(&mut self,snapshot:&ModelSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<RunProgress,InferenceWorkFailure> {
        if cx.is_cancelled() {
            return Err(InferenceError::Cancelled.into());
        }
        if self.done {
            return Ok(RunProgress { fraction: 1.0, done: true, ..RunProgress::default() });
        }
        let rendered_revision = self.rendered_revision;
        let progress = self.session(cx,|session,slot,cx| {
            if rendered_revision.is_some_and(|revision| session.has_rendered_revision(revision)) {
                return Ok(if cx.charge_work_unit() { RunProgress { fraction: 1.0, done: true, ..RunProgress::default() } } else { RunProgress::default() });
            }
            let Some(mut run)=slot.take()else{*slot=session.begin_sync_funded(cx)?;return Ok(RunProgress::default())};
            match session.step(&mut run,snapshot,cx) {
                Ok(progress) if progress.done=>{let result=session.finish(&mut run,snapshot,cx).map(|_|progress);if result.is_err(){*slot=Some(run)}result},
                Ok(progress) => {
                    *slot = Some(run);
                    Ok(progress)
                }
                Err(error) => {
                    *slot=Some(run);
                    Err(error)
                }
            }
        })?.unwrap_or_else(||Ok(RunProgress::default()))?;
        self.done = progress.done;
        Ok(progress)
    }

    fn reference_session<R>(&mut self, act: impl FnOnce(&mut ModelInferenceSession, &mut Option<SessionRun>) -> R) -> R {
        let Self { instance, local, run, .. } = self;
        match instance {
            Some(handle) => lend_handle(handle, DOCUMENT, |session| act(session, run)),
            None => act(local.get_or_insert_with(ModelInferenceSession::default), run),
        }
    }

    /// 🔬️ Advances a cold analysis by up to `fuel` uncached node computations.
    pub fn advance_cold(&mut self, snapshot: &ModelSnapshot, fuel: usize) -> Result<RunProgress, InferenceError> {
        if self.done {
            return Ok(RunProgress { fraction: 1.0, done: true, ..RunProgress::default() });
        }
        let progress = self.reference_session(|session, slot| {
            let mut run = slot.take().unwrap_or_else(|| session.begin_sync(snapshot));
            match session.step_cold(&mut run, snapshot, fuel) {
                Ok(progress) if progress.done => session.finish_cold(run, snapshot).map(|_| progress),
                Ok(progress) => {
                    *slot = Some(run);
                    Ok(progress)
                }
                Err(error) => {
                    let _ = session.finish_cold(run, snapshot);
                    Err(error)
                }
            }
        })?;
        self.done = progress.done;
        Ok(progress)
    }

    /// 🛑️ Closes a cold analysis as cancelled and preserves the last published inference.
    pub fn cancel_cold(&mut self, snapshot: &ModelSnapshot) {
        self.reference_session(|session, slot| {
            if let Some(mut run) = slot.take() {
                run.cancel();
                let _ = session.finish_cold(run, snapshot);
            }
        });
    }

    /// 📖️ Reads the reference inference at the requested snapshot.
    pub fn read_cold<R>(&mut self, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> Result<R, InferenceError> {
        self.reference_session(|session, _| session.try_sync(snapshot).map(read))
    }

    /// 🛑️ Cancels the analysis: the run is closed as cancelled and the session keeps every node it finished.
    pub fn cancel(&mut self,_snapshot:&ModelSnapshot) {
        if let Some(run)=self.run.as_mut(){run.cancel();}
    }
}
//#endregion 🔖️Analysis


