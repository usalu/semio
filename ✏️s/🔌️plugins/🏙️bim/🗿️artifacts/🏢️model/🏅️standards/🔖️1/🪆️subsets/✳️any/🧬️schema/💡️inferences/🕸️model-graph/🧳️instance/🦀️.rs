//! 🧳️ The inference sessions of a mounted instance. The framework owns them: one `ModelInferenceSession` per mounted artifact instance lives in the instance's `ArtifactInstanceOperationOwnerHandle` (see `with_inference_session`),
//! created on first use, dropped by the close ladder with the instance and never held by a process or thread global. This module only supplies what the BIM model adds: the session type, the names of its two sessions
//! (the document and the probe) and the calls every window, gesture, export and job reads derived values through. A render after a mutation brings its instance's session up to the snapshot from the concrete diffs the editor
//! recorded ([`record_mutations`]); a change nobody explains (undo, a peer's edit, a load) walks the plan and the cache serves every node whose dependency chain is unchanged. A job that must not block steps the same session
//! ([`begin`], [`step`], [`finish`]) with progress and cancellation. A probe (a hypothetical model such as a room under the pointer) runs on a session of its own so the document's state is never touched. A call without an
//! instance (`None`: a one-shot export, a fixture) works on a throwaway session.
//!
//! An engine fault never shows as an empty model: a read keeps the last complete inference and adds the finding [`DiagnosticCode::InferenceFault`], which the diagnostics panel and the problem counters show in English and German
//! ([`shown`]); the calls whose caller can refuse (`try_*`, exports, jobs) hand the fault over as a value.

use super::{kinds, ModelInferenceSession, RunProgress, SessionRun, UpdateReport};
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{ordered, Diagnostic, DiagnosticCode, DiagnosticIndex};
use crate::standards::v1::subsets::any::schema::inferences::spaces::SpaceRoom;
use crate::{ModelDiff, ModelInference, ModelMutation, ModelSnapshot};
use protocol::{InferenceError, Mutation, MutationDiff};
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use std::borrow::Cow;
use std::collections::BTreeMap;

//#region 🔖️Sessions
/// 🧳️ The mounted instance a call works for: its handle, or `None` for a one-shot call that keeps nothing.
pub type Instance<'a> = Option<&'a ArtifactInstanceOperationOwnerHandle>;

/// 🔮️ The name of the document's session in the instance.
pub const DOCUMENT: &str = "bim.model-graph";
/// 🔭️ The name of the probe session in the instance.
pub const PROBE: &str = "bim.model-graph.probe";

/// 🧠️ Lends the session `name` of `handle` to `act`, creating it on first use. An instance whose close ladder already dropped its sessions serves `act` a throwaway session instead.
fn lend_handle<R>(handle: &ArtifactInstanceOperationOwnerHandle, name: &'static str, act: impl FnOnce(&mut ModelInferenceSession) -> R) -> R {
    let mut act = Some(act);
    match handle.with_inference_session(name, ModelInferenceSession::default, |session| Ok(act.take().map(|act| act(session)))) {
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

/// 💡️ [`with_inference`] whose run failure is a value: a fault of the engine is returned instead of shown.
pub fn try_with_inference<R>(instance: Instance<'_>, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> Result<R, InferenceError> {
    lend(instance, DOCUMENT, |session| session.try_sync(snapshot).map(read))
}

/// 📝️ Records the concrete diffs of `mutations` the editor emits against `snapshot`, in order, for the instance's next sync. Each mutation's diff is read against the state the ones
/// before it leave; a mutation whose diff does not apply records nothing and the next sync walks the plan.
pub fn record_mutations(instance: Instance<'_>, snapshot: &ModelSnapshot, mutations: &[ModelMutation]) {
    let Some(handle) = instance else { return };
    let mut sum = ModelDiff::default();
    let mut state: Option<ModelSnapshot> = None;
    for (index, mutation) in mutations.iter().enumerate() {
        let base = state.as_ref().unwrap_or(snapshot);
        let (diff, _) = mutation.diff(base).into_parts();
        if index + 1 < mutations.len() {
            match protocol::apply_diff(&diff, base) {
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
pub fn step(instance: Instance<'_>, run: &mut SessionRun, snapshot: &ModelSnapshot, fuel: usize) -> Result<RunProgress, InferenceError> {
    lend(instance, DOCUMENT, |session| session.step(run, snapshot, fuel))
}

/// 🏁️ Ends `run`: a finished run settles the instance's session at `snapshot`, a cancelled or failed one leaves it as it was with every finished node cached.
pub fn finish(instance: Instance<'_>, run: SessionRun, snapshot: &ModelSnapshot) -> Result<(), InferenceError> {
    lend(instance, DOCUMENT, |session| session.finish(run, snapshot).map(|_| ()))
}
//#endregion 🔖️Runs

//#region 🔖️Analysis
/// 🧵️ A stepped, cancellable bring-up of one instance's session to a snapshot: what a job holds between its steps. The first [`advance`](Analysis::advance) opens the run, every later one spends the
/// fuel on it, the one that finishes it settles the session. [`cancel`](Analysis::cancel) closes the run as cancelled, so the nodes it finished stay cached for the next run. An analysis without an instance
/// settles a session of its own, so its runs belong to one session all the way.
pub struct Analysis {
    instance: Option<ArtifactInstanceOperationOwnerHandle>,
    local: Option<ModelInferenceSession>,
    fuel: usize,
    run: Option<SessionRun>,
    done: bool,
}

impl Analysis {
    /// 🏗️ An analysis of the document that `instance` shows, finishing at most `fuel` graph nodes per step (at least one).
    pub fn new(instance: Instance<'_>, fuel: usize) -> Self {
        Self { instance: instance.cloned(), local: None, fuel: fuel.max(1), run: None, done: false }
    }

    /// ✅️ Whether the session is settled at the snapshot.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// 📈️ How far the run is, as a fraction of its plan; it never decreases.
    pub fn fraction(&self) -> f32 {
        if self.done { 1.0 } else { self.run.as_ref().map_or(0.0, |run| run.progress().fraction) }
    }

    fn session<R>(&mut self, act: impl FnOnce(&mut ModelInferenceSession, &mut Option<SessionRun>, usize) -> R) -> R {
        let Self { instance, local, run, fuel, .. } = self;
        match instance {
            Some(handle) => lend_handle(handle, DOCUMENT, |session| act(session, run, *fuel)),
            None => act(local.get_or_insert_with(ModelInferenceSession::default), run, *fuel),
        }
    }

    /// ⏩️ Does one bounded step against `snapshot`; a run that fails or finishes is closed here.
    pub fn advance(&mut self, snapshot: &ModelSnapshot) -> Result<RunProgress, InferenceError> {
        if self.done {
            return Ok(RunProgress { fraction: 1.0, done: true, ..RunProgress::default() });
        }
        let progress = self.session(|session, slot, fuel| {
            let mut run = slot.take().unwrap_or_else(|| session.begin_sync(snapshot));
            match session.step(&mut run, snapshot, fuel) {
                Ok(progress) if progress.done => session.finish(run, snapshot).map(|_| progress),
                Ok(progress) => {
                    *slot = Some(run);
                    Ok(progress)
                }
                Err(error) => {
                    let _ = session.finish(run, snapshot);
                    Err(error)
                }
            }
        })?;
        self.done = progress.done;
        Ok(progress)
    }

    /// 🛑️ Cancels the analysis: the run is closed as cancelled and the session keeps every node it finished.
    pub fn cancel(&mut self, snapshot: &ModelSnapshot) {
        self.session(|session, slot, _| {
            if let Some(mut run) = slot.take() {
                run.cancel();
                let _ = session.finish(run, snapshot);
            }
        });
    }
}
//#endregion 🔖️Analysis

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
