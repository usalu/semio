//! 🔮️ The inference sessions of the mounted instances, shared by the editor and the viewer: one `ModelInferenceSession` of the model graph per instance, the ONE path every window, gesture,
//! export and job reads derived values through. A render after a mutation brings its instance's session up to the snapshot from the concrete diffs the editor recorded
//! ([`record_mutations`]); a change nobody explains (undo, a peer's edit, a load) walks the plan and the cache serves every node whose dependency chain is unchanged. A job that
//! must not block steps the same session ([`begin`], [`step`], [`finish`]) with progress and cancellation. A probe (a hypothetical model such as a room under the pointer) runs on a
//! session of its own so the document's state is never touched.

use super::{kinds, ModelInferenceSession, RunProgress, SessionRun, UpdateReport};
use super::super::spaces::SpaceRoom;
use crate::{ModelDiff, ModelInference, ModelMutation, ModelSnapshot};
use protocol::{InferenceError, Mutation, MutationDiff};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::thread::LocalKey;

//#region 🔖️Registry
/// 🪪️ The pseudo instance of renders that carry no mounted instance (fixtures, previews).
const UNMOUNTED: u32 = u32::MAX;

type Sessions = RefCell<BTreeMap<u32, ModelInferenceSession>>;

thread_local! {
    static SESSIONS: Sessions = const { RefCell::new(BTreeMap::new()) };
    static PROBES: Sessions = const { RefCell::new(BTreeMap::new()) };
}

/// 🧠️ Lends the session of `instance` to `act` and puts it back. The registry is not borrowed while `act` runs, so a read may itself reach the registry.
fn lend<R>(registry: &'static LocalKey<Sessions>, instance: Option<u32>, act: impl FnOnce(&mut ModelInferenceSession) -> R) -> R {
    let key = instance.unwrap_or(UNMOUNTED);
    let mut session = registry.with(|sessions| sessions.borrow_mut().remove(&key)).unwrap_or_default();
    let out = act(&mut session);
    registry.with(|sessions| sessions.borrow_mut().insert(key, session));
    out
}

/// 💡️ Reads the inference of `snapshot` for one mounted instance, bringing that instance's session up to `snapshot` first. A run that fails reads as the empty inference (the fault is in [`report`]), never as the inference of another snapshot.
pub fn with_inference<R>(instance: Option<u32>, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> R {
    lend(&SESSIONS, instance, |session| match session.try_sync(snapshot) {
        Ok(inference) => read(inference),
        Err(_) => read(&ModelInference::default()),
    })
}

/// 💡️ [`with_inference`] whose run failure is a value: a fault of the engine is returned instead of an empty inference.
pub fn try_with_inference<R>(instance: Option<u32>, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> Result<R, InferenceError> {
    lend(&SESSIONS, instance, |session| session.try_sync(snapshot).map(read))
}

/// 📝️ Records the concrete diffs of `mutations` the editor emits against `snapshot`, in order, for the instance's next sync. Each mutation's diff is read against the state the ones
/// before it leave; a mutation whose diff does not apply records nothing and the next sync walks the plan.
pub fn record_mutations(instance: Option<u32>, snapshot: &ModelSnapshot, mutations: &[ModelMutation]) {
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
    lend(&SESSIONS, instance, |session| session.record(sum));
}

/// 📊️ What the last run of one instance's session did: the proof an edit recomputed only what it touched.
pub fn report(instance: Option<u32>) -> UpdateReport {
    SESSIONS.with(|sessions| sessions.borrow().get(&instance.unwrap_or(UNMOUNTED)).map(|session| session.report().clone()).unwrap_or_default())
}

/// 🔭️ Reads what the kinds `WANT` select of a probe model (a model that is not the document: a room under the pointer, an import in progress) on the probe session of the instance, whose cache serves every node the probe leaves alone; the document's session is never touched.
pub fn probe<const WANT: u32, R>(instance: Option<u32>, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> Result<R, InferenceError> {
    lend(&PROBES, instance, |session| session.probe::<WANT>(snapshot).map(|inference| read(&inference)))
}

/// 🏠️ The rooms of a probe model (the document plus a hypothetical space).
pub fn probe_rooms(instance: Option<u32>, snapshot: &ModelSnapshot) -> BTreeMap<String, SpaceRoom> {
    probe::<{ kinds::ROOMS }, _>(instance, snapshot, |inference| inference.spaces.clone()).unwrap_or_default()
}

/// 🧹️ Drops the sessions of a closed instance.
pub fn close(instance: u32) {
    SESSIONS.with(|sessions| sessions.borrow_mut().remove(&instance));
    PROBES.with(|sessions| sessions.borrow_mut().remove(&instance));
}

/// ✅️ Whether the instance holds no session any more.
pub fn terminal_is_empty(instance: u32) -> bool {
    !SESSIONS.with(|sessions| sessions.borrow().contains_key(&instance)) && !PROBES.with(|sessions| sessions.borrow().contains_key(&instance))
}
//#endregion 🔖️Registry

//#region 🔖️Runs
/// 🚪️ Opens the run that brings the instance's session to `snapshot`, for a job that steps it.
pub fn begin(instance: Option<u32>, snapshot: &ModelSnapshot) -> SessionRun {
    lend(&SESSIONS, instance, |session| session.begin_sync(snapshot))
}

/// ⏩️ Spends at most `fuel` node computes of `run`; the progress it answers never decreases.
pub fn step(instance: Option<u32>, run: &mut SessionRun, snapshot: &ModelSnapshot, fuel: usize) -> Result<RunProgress, InferenceError> {
    lend(&SESSIONS, instance, |session| session.step(run, snapshot, fuel))
}

/// 🏁️ Ends `run`: a finished run settles the instance's session at `snapshot`, a cancelled or failed one leaves it as it was with every finished node cached.
pub fn finish(instance: Option<u32>, run: SessionRun, snapshot: &ModelSnapshot) -> Result<(), InferenceError> {
    lend(&SESSIONS, instance, |session| session.finish(run, snapshot).map(|_| ()))
}
//#endregion 🔖️Runs

//#region 🔖️Analysis
/// 🧵️ A stepped, cancellable bring-up of one instance's session to a snapshot: what a job holds between its steps. The first [`advance`](Analysis::advance) opens the run, every later one spends the
/// fuel on it, the one that finishes it settles the session. [`cancel`](Analysis::cancel) closes the run as cancelled, so the nodes it finished stay cached for the next run.
pub struct Analysis {
    instance: Option<u32>,
    fuel: usize,
    run: Option<SessionRun>,
    done: bool,
}

impl Analysis {
    /// 🏗️ An analysis of the document that `instance` shows, finishing at most `fuel` graph nodes per step (at least one).
    pub fn new(instance: Option<u32>, fuel: usize) -> Self {
        Self { instance, fuel: fuel.max(1), run: None, done: false }
    }

    /// ✅️ Whether the session is settled at the snapshot.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// 📈️ How far the run is, as a fraction of its plan; it never decreases.
    pub fn fraction(&self) -> f32 {
        if self.done { 1.0 } else { self.run.as_ref().map_or(0.0, |run| run.progress().fraction) }
    }

    /// ⏩️ Does one bounded step against `snapshot`; a run that fails or finishes is closed here.
    pub fn advance(&mut self, snapshot: &ModelSnapshot) -> Result<RunProgress, InferenceError> {
        if self.done {
            return Ok(RunProgress { fraction: 1.0, done: true, ..RunProgress::default() });
        }
        let mut run = self.run.take().unwrap_or_else(|| begin(self.instance, snapshot));
        let progress = match step(self.instance, &mut run, snapshot, self.fuel) {
            Ok(progress) => progress,
            Err(error) => {
                let _ = finish(self.instance, run, snapshot);
                return Err(error);
            }
        };
        if progress.done {
            finish(self.instance, run, snapshot)?;
            self.done = true;
        } else {
            self.run = Some(run);
        }
        Ok(progress)
    }

    /// 🛑️ Cancels the analysis: the run is closed as cancelled and the session keeps every node it finished.
    pub fn cancel(&mut self, snapshot: &ModelSnapshot) {
        if let Some(mut run) = self.run.take() {
            run.cancel();
            let _ = finish(self.instance, run, snapshot);
        }
    }
}
//#endregion 🔖️Analysis

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
