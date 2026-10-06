"""🧳️ Session-5 landing waves of S5-TOOLS (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, design §22.10, report
`📓️s4-tools-a-report.md` § Session 5). Every wave is re-derived from the CURRENT tree by anchored replacements (fail closed on a
moved anchor), staged as copies + a patch, and written into the tree only with `--land`, under the `landing` lock, together
with its gated `cargo check`.

- `slot`    (wave B, after F21 `drive`): the pure tool-machine crate — host facts, the framework-owned persisted gesture, the
            statechart tool on the shared runner, the per-window ledger — its schema, corpus, TS twin and laws. Delegates to
            `🧪️s5-tools-slot-{schema,ts,rust}.py` beside this file and the corpus generator.
- `runtime` (wave C, after `slot`): the plugin runtime owns the window gesture slot, ends it on host facts (a history edit
            freezes every window: `frozen`), folds it into the render overlay, and hands every dispatch its slot.

Usage (cwd: repo root):
  python3 🧪️s5-tools-stage.py <wave>          stage under 🗑️generated/s5-tools/staged/<wave>/ (copies + <wave>.patch)
  python3 🧪️s5-tools-stage.py <wave> --land   write the wave into the tree
"""

import difflib
import pathlib
import shutil
import subprocess
import sys

TICKET = pathlib.Path(__file__).resolve().parent
REPO = TICKET.parents[6]
GENERATED = TICKET / "🗑️generated/s5-tools"
TM = "🧰️framework/🔨️modules/🛠️tool-machine"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
TM_FILES = ["🦀️.rs", "🟦️.ts", "🧬️schema/🔣️.json", "🧫️fixtures/🧫️gesture-drive-law/🔣️.json", "🧪️tests/🧪️gesture-drive-law/🟦️.ts", "🧪️tests/🧪️gesture-drive-law/🦀️.rs", "🔮️oracles/🔣️.json"]


def replace(old, new):
    def apply(text):
        if text.count(old) != 1:
            raise SystemExit(f"anchor matched {text.count(old)} times: {old[:100]!r}")
        return text.replace(old, new)

    return apply


def append(tail):
    return lambda text: text.rstrip("\n") + "\n" + tail


def slot():
    """🧪️ Wave B: a fresh copy of the module's touched files, edited by the stage scripts, with the corpus regenerated."""
    work = GENERATED / "staged/slot-work"
    shutil.rmtree(work, ignore_errors=True)
    for relative in TM_FILES:
        source = REPO / TM / relative
        if not source.exists():
            raise SystemExit(f"{TM}/{relative} is missing: land F21 `drive` first")
        (work / relative).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, work / relative)
    for command in (["python3", str(TICKET / "🧪️s5-tools-slot-schema.py"), str(work / "🧬️schema/🔣️.json")], ["python3", str(TICKET / "🧪️s5-tools-slot-ts.py"), str(work)], ["python3", str(TICKET / "🧪️s5-tools-slot-rust.py"), str(work)], [str(REPO / ".venv/bin/python"), str(TICKET / "🧪️s4-tools-a-gesture-drive-law.py"), "--module", str(work)]):
        subprocess.run(command, check=True, cwd=REPO)
    oracles = work / "🔮️oracles/🔣️.json"
    text = oracles.read_text(encoding="utf-8")
    old = "as an xstate machine against `driveGesture` on every fixture row and random sequence."
    if text.count(old) != 1:
        raise SystemExit("the xstate oracle rationale moved")
    oracles.write_text(text.replace(old, "as an xstate machine against `driveGesture` on every fixture row and random sequence, and against the per-window `GestureLedger` on every slot scenario and host fact."), encoding="utf-8")
    return [(f"{TM}/{relative}", (REPO / TM / relative).read_text(encoding="utf-8"), (work / relative).read_text(encoding="utf-8")) for relative in TM_FILES]


#region 🖐️Runtime
DRIVER_DOC_OLD = """//! Every render seam reads the committed document (and config) overlaid with every open press's and run's provisional
//! leaves; every committed transaction is published stamped with its `TransactionRef`. Domain-neutral machines:
//! `semio_framework_tool_machine` (`ScrubMachine`, `TypingMachine`).
"""
DRIVER_DOC_NEW = """//! - 🖐️ gestures (§5, §22.10): every window owns ONE gesture slot in the instance's [`GestureLedger`]. A dispatch drives its
//!   window's streamed tool against a copy of the slot ([`GestureSlot::drive`]); the slot follows only when the dispatch
//!   publishes. Host facts end a window's gesture here, never in an editor: a blur `blur`, a lost pointer capture
//!   `captureLost`, a utility switch or a closing window `retired`, a moved base `baseMoved` (a gesture pinned to a revision),
//!   and an opened history edit `frozen` for every window — zero trace each time.
//!
//! Every render seam reads the committed document (and config) overlaid with every open press's, gesture's and run's
//! provisional leaves; every committed transaction is published stamped with its `TransactionRef`. Domain-neutral machines:
//! `semio_framework_tool_machine` (`ScrubMachine`, `TypingMachine`, `ChartGesture`).
"""

GESTURE_REGION = '''//#endregion 🔖️Tag

//#region 🖐️Gesture
/// 🎰️ The dispatching window's gesture slot as ONE admitted dispatch sees it (design §22.10): the gesture the window held
/// at admission, the document revision the dispatch runs on, and what the dispatch decided — kept by the runtime only when
/// the dispatch publishes, and only while the window still holds the gesture it was admitted on. Shared with the dispatch's
/// retained job, so its state sits behind one lock.
pub struct GestureSlot<M> {
    window: String,
    base_revision: String,
    state: std::sync::Mutex<GestureSlotState<M>>,
}

struct GestureSlotState<M> {
    admitted: Option<GestureState<M>>,
    decided: Option<Option<GestureState<M>>>,
}

impl<M> GestureSlot<M> {
    /// 🕳️ The slot of a dispatch outside a live window (a preview, a view without command authority): it holds no gesture
    /// and nothing it decides is kept.
    pub fn detached() -> Self {
        Self::of(String::new(), String::new(), None)
    }

    fn of(window: String, base_revision: String, admitted: Option<GestureState<M>>) -> Self {
        Self { window, base_revision, state: std::sync::Mutex::new(GestureSlotState { admitted, decided: None }) }
    }

    /// 🪟️ The window this slot belongs to.
    pub fn window(&self) -> &str {
        &self.window
    }

    fn take(&self) -> Option<(Option<GestureState<M>>, Option<GestureState<M>>)> {
        let mut state = self.state.lock().ok()?;
        let decided = state.decided.take()?;
        Some((state.admitted.take(), decided))
    }

    /// 🔦️ The window's gesture as this dispatch sees it: what it decided so far, else the gesture the window held when the
    /// dispatch was admitted.
    pub fn open(&self) -> Option<GestureState<M>>
    where
        M: Clone,
    {
        let state = self.state.lock().ok()?;
        match &state.decided {
            Some(decided) => decided.clone(),
            None => state.admitted.clone(),
        }
    }

    /// 🚃️ Drives the window's streamed tool `T` through ONE dispatch of `verb` ([`semio_framework_tool_machine::drive_gesture`])
    /// against this slot — a second drive of the same dispatch continues from the first — and answers the transaction it
    /// committed (publish it as ONE edit: [`gesture_emit`]). A tool that is not base-bound pins its gesture to no revision. A
    /// refused start or tick is the dispatch's fault (`toolTransaction.closed` | `toolTransaction.unclosed`) and decides nothing.
    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(&self, verb: &str, phase: GesturePhase, tick: Option<T::Tick>, authoring_seed: &str) -> Result<Option<(protocol::TransactionRef, Vec<M>)>, Fault> {
        let mut state = self.state.lock().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("toolGesture.slot-poisoned"), format!("the gesture slot of window {:?} is poisoned", self.window)))?;
        let persisted = match state.decided.as_ref() {
            Some(decided) => decided.as_ref(),
            None => state.admitted.as_ref(),
        };
        let drive = semio_framework_tool_machine::drive_gesture::<T>(persisted, verb, phase, tick, authoring_seed, if T::BASE_BOUND { self.base_revision.as_str() } else { "" })
            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("gesture tool {verb:?} refused its dispatch")))?;
        if let Some(decided) = drive.next {
            state.decided = Some(decided);
        }
        Ok(drive.committed)
    }
}

/// 📤️ What one gesture dispatch publishes: its committed transaction as ONE edit stamped with the ref — plainly when the
/// dispatch carries no admission (`authoring_seed` empty: a render or test view without command authority) — or nothing.
pub fn gesture_emit<Mutation, ConfigMutation, DraftMutation>(committed: Option<(protocol::TransactionRef, Vec<Mutation>)>, authoring_seed: &str) -> Emit<Mutation, ConfigMutation, DraftMutation> {
    match committed {
        Some((transaction, mutations)) if !authoring_seed.is_empty() => Emit::commit_transaction(transaction, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    }
}

/// 📡️ The gesture fact a host event is (law table `hostEvents` of the tool-machine gesture-drive corpus).
pub(super) fn gesture_host_event(event: &HostEvent) -> GestureHostEvent {
    match event {
        HostEvent::WindowBlurred { .. } => GestureHostEvent::Blur,
        HostEvent::PointerCaptureLost { .. } => GestureHostEvent::CaptureLost,
        HostEvent::UtilityChanged { .. } => GestureHostEvent::UtilityChanged,
        HostEvent::Retiring { .. } => GestureHostEvent::Retiring,
        HostEvent::TimeTravelFrozen { .. } => GestureHostEvent::TimeTravelFrozen,
        HostEvent::BaseMoved { .. } => GestureHostEvent::BaseMoved,
    }
}
//#endregion 🖐️Gesture
'''

DRIVER_METHODS = '''    /// 🎟️ The gesture slot of the dispatch `meta` admits as `operation` on the document revision `base`: the gesture its
    /// window holds now, bound to the operation until its completion publishes (the oldest slot of an operation that never
    /// completed yields its place). Windows that left the dispatching view's roster retire their gestures first.
    pub(super) fn admit_gesture_slot(&mut self, operation: u64, base: &[u8; 32], meta: &ActionMeta) -> Arc<GestureSlot<A::Mutation>> {
        let view = meta.view_state.as_ref();
        if let Some(view) = view.filter(|view| !view.window_instances.is_empty()) {
            let retired = self.tool_machines.gestures.retain_windows(|window| window.is_empty() || view.window_instances.iter().any(|instance| instance.id == window));
            if !retired.is_empty() {
                self.tool_machines.gesture_ended = true;
                self.follow_tool_machines(true);
            }
        }
        let window = view.and_then(|view| view.window_id.clone()).unwrap_or_default();
        let base_revision = base.iter().map(|byte| format!("{byte:02x}")).collect();
        let open = self.tool_machines.gestures.open(&window).cloned();
        let slot = Arc::new(GestureSlot::of(window, base_revision, open));
        if self.tool_machines.gesture_operations.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            self.tool_machines.gesture_operations.remove(0);
        }
        self.tool_machines.gesture_operations.push((operation, Arc::clone(&slot)));
        slot
    }

    /// 🖋️ Keeps what `operation`'s dispatch decided for its window's gesture, now that it publishes: only while no history
    /// edit freezes the document and the window still holds the gesture the dispatch was admitted on (a host fact or another
    /// dispatch that moved the slot since wins). Answers whether the dispatch drove its slot at all.
    pub(super) fn settle_gesture_slot(&mut self, operation: u64, publishes: bool) -> bool {
        let Some(index) = self.tool_machines.gesture_operations.iter().position(|(owner, _)| *owner == operation) else { return false };
        let slot = self.tool_machines.gesture_operations.remove(index).1;
        let Some((admitted, decided)) = slot.take() else { return false };
        if publishes && !self.time_travel.freezes_local_emits() && self.tool_machines.gestures.open(slot.window()) == admitted.as_ref() {
            self.tool_machines.gestures.settle(slot.window(), decided);
            self.follow_tool_machines(true);
        }
        true
    }

    /// 🛎️ A host fact of one window: its open gesture ends with the fact's reason and zero trace — the ONE place a blur, a
    /// lost capture, a utility switch, a closing window, a moved base or a history edit ends a gesture; no editor maps it.
    pub(super) fn end_window_gesture(&mut self, event: &HostEvent) {
        if matches!(self.tool_machines.gestures.host_event(event.window_id(), gesture_host_event(event)), ToolStep::Aborted(..)) {
            self.tool_machines.gesture_ended = true;
            self.follow_tool_machines(true);
        }
    }

    /// 🌐️ An instance-wide host fact (a history edit froze the document, a remote edit moved the base): every window's
    /// open gesture ends with the fact's reason and zero trace, whether or not a view lists the window.
    pub(super) fn end_every_gesture(&mut self, event: &HostEvent) {
        if !self.tool_machines.gestures.host_event_all(gesture_host_event(event)).is_empty() {
            self.tool_machines.gesture_ended = true;
            self.follow_tool_machines(true);
        }
    }

    /// 🎨️ Whether a host fact ended a gesture since the last ask: the windows that previewed it owe a repaint.
    pub(super) fn take_gesture_ended(&mut self) -> bool {
        std::mem::take(&mut self.tool_machines.gesture_ended)
    }

    /// 🪞️ Refolds the tool overlays'''

SETTLE_OLD = """    pub(super) fn settle_tool_operation(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, publication: &mut ArtifactToolCompletionValue<A>) -> Result<(), Fault> {
        let Some(tag) = self.tool_machines.take_operation(mounted.operation.operation.0) else { return Ok(()) };
        let ArtifactToolCompletionValue::Emit(Ok(emit), _) = publication else { return Ok(()) };
        match tag {
"""
SETTLE_NEW = """    pub(super) fn settle_tool_operation(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, publication: &mut ArtifactToolCompletionValue<A>) -> Result<(), Fault> {
        let operation = mounted.operation.operation.0;
        let driven = self.settle_gesture_slot(operation, matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _)));
        let tag = self.tool_machines.take_operation(operation);
        let ArtifactToolCompletionValue::Emit(Ok(emit), _) = publication else { return Ok(()) };
        if driven && emit.transaction.is_none() && emit.artifact_mutations.is_empty() {
            mounted.command_logged = true;
        }
        let Some(tag) = tag else { return Ok(()) };
        match tag {
"""

LAW = '''//! 🖐️ Runtime laws of the framework-owned gesture slot (design §22.10 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING):
//! every window owns ONE slot in the instance's `GestureLedger`; a dispatch drives its window's tool against a copy of the
//! slot and the slot follows only when the dispatch publishes; every render reads committed ⊕ the open gestures; and host
//! facts end a gesture in the runtime with zero trace — an opened history edit `frozen` for every window — for an app that
//! answers no host event of its own (the toy history app). The drive itself is pinned by the tool-machine corpus
//! (`🧫️gesture-drive-law`: `rows`, `hostEvents`, `slots`).

use super::*;
use semio_framework_tool_machine::{GesturePhase, GestureState, GestureTool, ToolAbortReason, ToolRefusal, ToolStep};

const NUDGE_TOOL: &str = "s.test.time-travel@1/*#editor#nudge";

/// 🧮️ The law's streamed tool: every tick replaces its ONE absolute leaf, the release commits it, and its persisted form is
/// the framework's `GestureState`.
struct NudgeTool {
    verb: String,
    authoring_seed: String,
    base_revision: String,
    transaction: Option<protocol::TransactionRef>,
    leaf: Option<TestMutation>,
}

impl GestureTool for NudgeTool {
    type Gesture = GestureState<TestMutation>;
    type Tick = TestMutation;
    type Mutation = TestMutation;

    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        Ok(Self { verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string(), transaction: None, leaf: None })
    }

    fn resume(gesture: &GestureState<TestMutation>) -> Result<Self, ToolRefusal> {
        let [(_, leaf)] = gesture.entries.as_slice() else { return Err(ToolRefusal::Closed) };
        Ok(Self { verb: gesture.verb.clone(), authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone(), transaction: Some(gesture.transaction.clone()), leaf: Some(leaf.clone()) })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base_revision
    }

    fn abort(&mut self, _reason: ToolAbortReason) {
        self.transaction = None;
        self.leaf = None;
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<TestMutation>) -> Result<ToolStep<TestMutation>, ToolRefusal> {
        if tick.is_some() {
            self.leaf = tick;
        }
        let Some(leaf) = self.leaf.clone() else { return Ok(ToolStep::Idle) };
        let transaction = match self.transaction.clone() {
            Some(transaction) => transaction,
            None => protocol::TransactionRef::mint(&ActorId(self.authoring_seed.clone()), &HybridLogicalTimestamp { actor: 0, physical_ms: 1, logical: 0 }, NUDGE_TOOL),
        };
        if phase == GesturePhase::Stream {
            self.transaction = Some(transaction);
            return Ok(ToolStep::Open);
        }
        self.transaction = None;
        self.leaf = None;
        Ok(ToolStep::Committed(transaction, vec![leaf]))
    }

    fn persist(self) -> Option<GestureState<TestMutation>> {
        let (transaction, leaf) = (self.transaction?, self.leaf?);
        Some(GestureState { states: vec!["streaming".to_string()], verb: self.verb, authoring_seed: self.authoring_seed, base_revision: self.base_revision, transaction, entries: vec![("leaf".to_string(), leaf)], context: DslValue::Null })
    }
}

fn two_panes() -> Vec<ViewWindowInstance> {
    vec![ViewWindowInstance { id: "pane-a".into(), window_kind_id: "main".into() }, ViewWindowInstance { id: "pane-b".into(), window_kind_id: "main".into() }]
}

fn in_window(actor: &str, window: &str) -> ActionMeta {
    let view = ViewModel { window_id: Some(window.to_string()), window_instances: two_panes(), ..ViewModel::new(Locale::En, Terminology::Native) };
    ActionMeta { view_state: Some(view), ..artifact_app_laws::meta(actor) }
}

/// 🚃️ One stream tick of `window` setting the count to `value`, as the dispatch `operation` that publishes: admitted on the
/// live revision, driven against its slot, settled like a publishing completion. Answers what the tick committed.
fn stream(app: &mut ToyApp, actor: &str, operation: u64, window: &str, value: i32) -> Option<(protocol::TransactionRef, Vec<TestMutation>)> {
    let base = app.store.content_revision_now();
    let slot = app.admit_gesture_slot(operation, &base, &in_window(actor, window));
    let committed = slot.drive::<NudgeTool>("nudge", GesturePhase::Stream, Some(SetCount { value }.into()), "seed").expect("a stream tick is never refused");
    assert!(app.settle_gesture_slot(operation, true), "the dispatch drove its slot");
    committed
}

fn open_windows(app: &ToyApp) -> Vec<String> {
    app.tool_machines.gestures().windows().map(str::to_string).collect()
}

/// 🧊️ LAW (design §22.10, the brief's Frozen law): two windows hold a gesture in flight — persisted by one dispatch, resumed
/// by the next, previewed by every render over an untouched committed document. Opening a history edit ends both `frozen`
/// in the runtime: no slot, no overlay, no edit, no store change — although the app answers no host event at all — and a
/// tick that arrives while the document is frozen opens nothing.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_ends_every_open_gesture_with_zero_trace_for_an_app_without_a_frozen_arm() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    assert!(ToyHistoryApp::host_event(&HostEvent::TimeTravelFrozen { window_id: "pane-a".into() }).is_none(), "the app has no Frozen arm of its own");
    let (generation, committed, edits) = (app.store.generation(), head(&app), app.edit_transactions().len());
    assert_eq!(stream(&mut app, &actor, 900, "pane-a", 40), None, "a tick publishes nothing");
    assert_eq!(stream(&mut app, &actor, 901, "pane-a", 41), None);
    assert_eq!(stream(&mut app, &actor, 902, "pane-b", 42), None);
    assert_eq!(open_windows(&app), vec!["pane-a", "pane-b"]);
    let resumed = app.tool_machines.gestures().open("pane-a").expect("pane-a holds its gesture").clone();
    assert_eq!((resumed.verb.as_str(), resumed.entries.as_slice()), ("nudge", [("leaf".to_string(), TestMutation::from(SetCount { value: 41 }))].as_slice()), "the second tick resumed the first tick's gesture");
    assert_eq!(app.rendered_snapshot().count, 42, "every render reads committed ⊕ the open gestures, window by window");
    assert_eq!((app.store.generation(), head(&app), app.edit_transactions().len()), (generation, committed.clone(), edits), "an open gesture never touches the committed document");
    TOY_HOST_EVENTS.with(|events| events.borrow_mut().clear());
    let mutation = seeded_mutation(&app, 0);
    let begun = app.handle_action("historyEditBegin", Some(&DslValue::Object(vec![("mutationId".into(), DslValue::String(mutation))])), &in_window(&actor, "pane-a")).await.expect("begin");
    assert_eq!(rejected(&begun), None);
    assert_eq!(TOY_HOST_EVENTS.with(|events| events.borrow().len()), 2, "the app was told once per window and answered nothing");
    assert!(app.tool_machines.gestures().is_empty(), "the history edit ended every window's gesture");
    assert_eq!(app.rendered_snapshot().count, committed.0, "no overlay is left");
    assert_eq!(stream(&mut app, &actor, 903, "pane-b", 77), None);
    assert!(app.tool_machines.gestures().is_empty(), "a tick on the frozen document opens no gesture");
    assert_eq!((app.store.generation(), head(&app), app.edit_transactions().len()), (generation, committed, edits), "zero trace: no edit, no store change");
    app.handle_action("historyEditExit", None, &in_window(&actor, "pane-a")).await.expect("exit");
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work()).await;
    close(&mut app);
}

/// 🛎️ LAW (design §22.10): a window's host fact ends only that window's gesture, in the runtime — the host-forwarded blur
/// and lost capture through the real `hostEvent` verb, with the app answering nothing — and the render falls back to the
/// committed document once the last gesture is gone.
#[semio_framework_async_macros::async_test]
async fn a_windows_host_fact_ends_only_its_own_gesture() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    let (generation, committed) = (app.store.generation(), head(&app));
    stream(&mut app, &actor, 900, "pane-a", 40);
    stream(&mut app, &actor, 901, "pane-b", 42);
    let host = |window: &str, kind: &str| DslValue::Object(vec![(semio_framework::HOST_EVENT_ARG_WINDOW_ID.to_string(), DslValue::String(window.to_string())), (semio_framework::HOST_EVENT_ARG_KIND.to_string(), DslValue::String(kind.to_string()))]);
    app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&host("pane-a", semio_framework::HOST_EVENT_KIND_BLUR)), &in_window(&actor, "pane-a")).await.expect("blur");
    assert_eq!(open_windows(&app), vec!["pane-b"], "the blur ended pane-a's gesture only");
    assert_eq!(app.rendered_snapshot().count, 42);
    app.handle_action(semio_framework::HOST_EVENT_ACTION_ID, Some(&host("pane-b", semio_framework::HOST_EVENT_KIND_CAPTURE_LOST)), &in_window(&actor, "pane-b")).await.expect("capture lost");
    assert!(app.tool_machines.gestures().is_empty());
    assert_eq!(app.rendered_snapshot().count, committed.0, "the render reads the committed document again");
    assert_eq!((app.store.generation(), head(&app)), (generation, committed), "zero trace");
    close(&mut app);
}

/// 🖋️ LAW (design §22.10): the slot follows a dispatch only when it publishes and only while the window still holds the
/// gesture the dispatch was admitted on — a faulted dispatch decides nothing, and of two dispatches admitted on the same
/// slot the first to publish wins; a release commits the gesture as ONE transaction and clears the slot.
#[semio_framework_async_macros::async_test]
async fn the_slot_follows_only_a_publishing_dispatch_admitted_on_the_gesture_it_holds() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut app = seeded_app(&fixture).await;
    let base = app.store.content_revision_now();
    let faulted = app.admit_gesture_slot(900, &base, &in_window(&actor, "pane-a"));
    faulted.drive::<NudgeTool>("nudge", GesturePhase::Stream, Some(SetCount { value: 7 }.into()), "seed").expect("a tick");
    assert!(app.settle_gesture_slot(900, false));
    assert!(app.tool_machines.gestures().is_empty(), "a dispatch that does not publish decides nothing");
    let (first, second) = (app.admit_gesture_slot(901, &base, &in_window(&actor, "pane-a")), app.admit_gesture_slot(902, &base, &in_window(&actor, "pane-a")));
    first.drive::<NudgeTool>("nudge", GesturePhase::Stream, Some(SetCount { value: 8 }.into()), "seed").expect("a tick");
    second.drive::<NudgeTool>("nudge", GesturePhase::Stream, Some(SetCount { value: 9 }.into()), "seed").expect("a tick");
    assert!(app.settle_gesture_slot(901, true) && app.settle_gesture_slot(902, true));
    assert_eq!(app.rendered_snapshot().count, 8, "the dispatch admitted on a slot that moved since is dropped");
    assert!(!app.settle_gesture_slot(902, true), "a slot is settled once");
    let release = app.admit_gesture_slot(903, &base, &in_window(&actor, "pane-a"));
    let (transaction, mutations) = release.drive::<NudgeTool>("nudge", GesturePhase::Commit, Some(SetCount { value: 10 }.into()), "seed").expect("the release").expect("the release commits");
    assert_eq!((transaction.tool.as_str(), mutations), (NUDGE_TOOL, vec![TestMutation::from(SetCount { value: 10 })]), "ONE transaction of the net leaf");
    assert!(app.settle_gesture_slot(903, true));
    assert!(app.tool_machines.gestures().is_empty(), "the release cleared the slot");
    let emit: Emit<TestMutation, TestConfigMutation, NoDraftMutation> = gesture_emit(Some((transaction.clone(), vec![SetCount { value: 10 }.into()])), "seed");
    assert_eq!((emit.transaction, emit.artifact_mutations.len()), (Some(transaction), 1), "the committed gesture publishes as ONE stamped edit");
    close(&mut app);
}
'''

RUNTIME = {
    f"{PLUGIN}/🛠️tool-machine/🦀️.rs": [
        replace(DRIVER_DOC_OLD, DRIVER_DOC_NEW),
        replace(
            "use semio_framework_tool_machine::{ScrubLedger, ScrubPhase, ToolAbortReason, ToolStep, TypingCommit, TypingInput, TypingLedger, TypingPhase, SCRUB_ABORT_ARG, SCRUB_COMMIT_ARG, SCRUB_GESTURE_ARG, TYPING_BUFFER_ARG, TYPING_COMMIT_ARG};",
            "use semio_framework_tool_machine::{\n    GestureHostEvent, GestureLedger, GesturePhase, GestureState, GestureTool, ScrubLedger, ScrubPhase, ToolAbortReason, ToolStep, TypingCommit, TypingInput, TypingLedger, TypingPhase, SCRUB_ABORT_ARG, SCRUB_COMMIT_ARG, SCRUB_GESTURE_ARG,\n    TYPING_BUFFER_ARG, TYPING_COMMIT_ARG,\n};",
        ),
        replace("//#endregion 🔖️Tag\n", GESTURE_REGION),
        replace(
            "/// 🗂️ The instance's continuous-tool runtime: the per-window press ledger (every lane of a press as [`PressLeaf`]s) and\n/// [`TypingLedger`], the committed ⊕ provisional overlays",
            "/// 🗂️ The instance's continuous-tool runtime: the per-window press ledger (every lane of a press as [`PressLeaf`]s),\n/// [`GestureLedger`] (each window's ONE gesture slot, the slots admitted dispatches hold until they publish, and whether a\n/// host fact ended a gesture since the last repaint) and [`TypingLedger`], the committed ⊕ provisional overlays",
        ),
        replace("    presses: ScrubLedger<PressLeaf<M, CM>>,\n    typing: TypingLedger<M>,\n    overlay: Option<Arc<P>>,", "    presses: ScrubLedger<PressLeaf<M, CM>>,\n    gestures: GestureLedger<M>,\n    gesture_operations: Vec<(u64, Arc<GestureSlot<M>>)>,\n    gesture_ended: bool,\n    typing: TypingLedger<M>,\n    overlay: Option<Arc<P>>,"),
        replace("            presses: ScrubLedger::default(),\n            typing: TypingLedger::default(),", "            presses: ScrubLedger::default(),\n            gestures: GestureLedger::default(),\n            gesture_operations: Vec::new(),\n            gesture_ended: false,\n            typing: TypingLedger::default(),"),
        replace(
            "    /// 🔎️ The open typing runs.\n    pub fn typing(&self) -> &TypingLedger<M> {\n        &self.typing\n    }\n",
            "    /// 🔎️ The open typing runs.\n    pub fn typing(&self) -> &TypingLedger<M> {\n        &self.typing\n    }\n\n    /// 🖐️ Every window's open gesture.\n    pub fn gestures(&self) -> &GestureLedger<M> {\n        &self.gestures\n    }\n",
        ),
        replace(
            "        self.presses.provisional().filter_map(PressLeaf::member).chain(typing).map(semio_framework_value::ToValue::to_value).collect()",
            "        self.presses.provisional().filter_map(PressLeaf::member).chain(self.gestures.provisional()).chain(typing).map(semio_framework_value::ToValue::to_value).collect()",
        ),
        replace(
            "        if self.presses.provisional().all(|leaf| leaf.member().is_none()) && self.typing.is_empty() {\n            let dropped: Vec<Arc<P>> = self.overlay.take().into_iter().collect();",
            "        if self.presses.provisional().all(|leaf| leaf.member().is_none()) && self.gestures.is_empty() && self.typing.is_empty() {\n            let dropped: Vec<Arc<P>> = self.overlay.take().into_iter().collect();",
        ),
        replace(
            "        for leaf in self.presses.provisional().filter_map(PressLeaf::member) {\n            fold_leaf(committed, &mut running, &mut displaced, leaf);\n        }\n        let mut conflicts = Vec::new();",
            "        for leaf in self.presses.provisional().filter_map(PressLeaf::member).chain(self.gestures.provisional()) {\n            fold_leaf(committed, &mut running, &mut displaced, leaf);\n        }\n        let mut conflicts = Vec::new();",
        ),
        replace(
            "    /// 🪞️ The document every render seam reads: `committed` with every open press's and run's provisional leaves, else",
            "    /// 🪞️ The document every render seam reads: `committed` with every open press's, gesture's and run's provisional leaves, else",
        ),
        replace(
            "        if machines.presses.is_empty() && machines.typing.is_empty() && machines.overlay.is_none() && machines.config_overlay.is_none() && machines.window_overlays.is_empty() {",
            "        if machines.presses.is_empty() && machines.gestures.is_empty() && machines.typing.is_empty() && machines.overlay.is_none() && machines.config_overlay.is_none() && machines.window_overlays.is_empty() {",
        ),
        replace("    /// 🪞️ Refolds the tool overlays", DRIVER_METHODS),
        replace(SETTLE_OLD, SETTLE_NEW),
        replace(
            "    /// 🛠️ The ONE point a tagged operation's completion becomes its publication. A press settles every lane of its emit on",
            "    /// 🛠️ The ONE point an operation's completion becomes its tools' publication. The gesture slot its dispatch drove follows\n    /// first ([`Self::settle_gesture_slot`]); a dispatch that only advanced or ended its gesture logs no history row. A tagged\n    /// operation then settles its press or run: a press settles every lane of its emit on",
        ),
    ],
    f"{PLUGIN}/🦀️.rs": [
        replace("    pub use tool_machine::{ScrubTag, ToolMachineRuntime, ToolTag, TypingTag};", "    pub use tool_machine::{gesture_emit, GestureSlot, ScrubTag, ToolMachineRuntime, ToolTag, TypingTag};"),
        replace(
            "        provisional: Vec<DslValue>,\n        provisional_generation: u64,\n    }\n\n    fn artifact_owned_tool_job_context_identity_digest(",
            "        provisional: Vec<DslValue>,\n        provisional_generation: u64,\n        gesture: std::sync::Arc<GestureSlot<A::Mutation>>,\n    }\n\n    fn artifact_owned_tool_job_context_identity_digest(",
        ),
        replace(
            "                provisional: Vec::new(),\n                provisional_generation: 0,\n            }\n        }\n\n        pub fn identity_digest(&self) -> u64 {",
            "                provisional: Vec::new(),\n                provisional_generation: 0,\n                gesture: std::sync::Arc::new(GestureSlot::detached()),\n            }\n        }\n\n        pub fn identity_digest(&self) -> u64 {",
        ),
        replace(
            "        /// 🔢️ The overlay generation as of admission: it moves on every tick, release and zero-trace abort.\n        pub fn provisional_generation(&self) -> u64 {\n            self.provisional_generation\n        }\n    }\n",
            "        /// 🔢️ The overlay generation as of admission: it moves on every tick, release and zero-trace abort.\n        pub fn provisional_generation(&self) -> u64 {\n            self.provisional_generation\n        }\n\n        /// 🎰️ Binds the dispatching window's gesture slot as of command admission (design §22.10). Ephemeral local tool\n        /// state, so it is not part of the identity digest.\n        pub fn with_gesture(mut self, gesture: std::sync::Arc<GestureSlot<A::Mutation>>) -> Self {\n            self.gesture = gesture;\n            self\n        }\n\n        /// 🖐️ The dispatching window's gesture slot: a streamed tool drives its window's ONE gesture through it\n        /// (`GestureSlot::drive`) instead of persisting a gesture of its own; the runtime keeps what the dispatch decided\n        /// when it publishes and ends the gesture on host facts.\n        pub fn gesture(&self) -> &GestureSlot<A::Mutation> {\n            &self.gesture\n        }\n    }\n",
        ),
        replace(
            "                .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation()),\n            );\n            let operation_spec = match admission.proof.clone() {",
            "                .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation())\n                .with_gesture(self.admit_gesture_slot(operation_id.0, &canonical_base_revision, meta)),\n            );\n            let operation_spec = match admission.proof.clone() {",
        ),
        replace(
            "        async fn deliver_host_event(&mut self, event: HostEvent, meta: &ActionMeta) -> Result<(), Fault> {\n            let Some(command) = A::host_event(&event) else { return Ok(()) };",
            "        async fn deliver_host_event(&mut self, event: HostEvent, meta: &ActionMeta) -> Result<(), Fault> {\n            self.end_window_gesture(&event);\n            let Some(command) = A::host_event(&event) else { return Ok(()) };",
        ),
        replace(
            "        /// 📨️ Delivers `event` to the app: its typed answer ([`ArtifactApp::host_event`]) dispatches to the event's window —",
            "        /// 📨️ Ends the event's window's open gesture with the fact's reason (the runtime's ONE mapping, design §22.10), then\n        /// delivers `event` to the app: its typed answer ([`ArtifactApp::host_event`]) dispatches to the event's window —",
        ),
        replace(
            "        pub(crate) async fn deliver_host_event_to_every_window(&mut self, event: impl Fn(String) -> HostEvent, meta: &ActionMeta) -> Result<(), Fault> {\n",
            "        pub(crate) async fn deliver_host_event_to_every_window(&mut self, event: impl Fn(String) -> HostEvent, meta: &ActionMeta) -> Result<(), Fault> {\n            self.end_every_gesture(&event(String::new()));\n",
        ),
        replace(
            "            self.deliver_host_event(event, meta).await?;\n            Ok(Self::empty_result(semio_framework::HOST_EVENT_ACTION_ID, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)",
            "            self.deliver_host_event(event, meta).await?;\n            let ui_scope = if self.take_gesture_ended() { UiDirtyScope::Full } else { UiDirtyScope::None };\n            Ok(Self::empty_result(semio_framework::HOST_EVENT_ACTION_ID, meta, Vec::new(), Vec::new(), ui_scope).await)",
        ),
    ],
    f"{PLUGIN}/🧪️tests/🧪️time-travel/🦀️.rs": [append('\n//#region 🖐️GestureSlot\n#[path = "../🧪️gesture/🦀️.rs"]\nmod gesture_laws;\n//#endregion 🖐️GestureSlot\n')],
    f"{PLUGIN}/🧪️tests/🧪️gesture/🦀️.rs": LAW,
}
#endregion 🖐️Runtime


def runtime():
    staged = []
    for path, edit in RUNTIME.items():
        file = REPO / path
        current = file.read_text(encoding="utf-8") if file.exists() else None
        if isinstance(edit, str):
            if current is not None and current != edit:
                raise SystemExit(f"{path} already exists with other content")
            staged.append((path, current, edit))
            continue
        if current is None:
            raise SystemExit(f"{path} is gone")
        text = current
        for apply in edit:
            text = apply(text)
        staged.append((path, current, text))
    return staged


WAVES = {"slot": slot, "runtime": runtime}


def main():
    wave = sys.argv[1]
    staged = WAVES[wave]()
    if "--land" in sys.argv:
        for path, _, text in staged:
            target = REPO / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(text, encoding="utf-8")
            print(f"landed {path}")
        return
    patch = []
    for path, current, text in staged:
        copy = GENERATED / "staged" / wave / path
        copy.parent.mkdir(parents=True, exist_ok=True)
        copy.write_text(text, encoding="utf-8")
        patch += difflib.unified_diff((current or "").splitlines(keepends=True), text.splitlines(keepends=True), f"a/{path}" if current is not None else "/dev/null", f"b/{path}")
    (GENERATED / "staged" / f"{wave}.patch").write_text("".join(patch), encoding="utf-8")
    changed = sum(1 for line in patch if line.startswith(("+", "-")) and not line.startswith(("+++", "---")))
    print(f"staged {wave}: {len(staged)} files, {changed} changed lines -> 🗑️generated/s5-tools/staged/{wave}.patch")


if __name__ == "__main__":
    main()
