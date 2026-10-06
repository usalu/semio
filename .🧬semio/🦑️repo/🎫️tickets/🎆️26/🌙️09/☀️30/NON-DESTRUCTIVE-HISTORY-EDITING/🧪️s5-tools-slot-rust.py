"""🦀️ Wave B Rust edit (design §22.10) of the pure tool-machine crate: host facts, the framework-owned persisted gesture, the
statechart tool on the shared gesture runner and the per-window ledger, plus their fixture laws.
Usage: python3 rust-tm-edit.py <🛠️tool-machine dir>   (the dir must already hold the F21 `drive` group)"""

import pathlib
import sys

root = pathlib.Path(sys.argv[1])


def edit(relative, edits):
    path = root / relative
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            raise SystemExit(f"{relative}: anchor matched {text.count(old)} times: {old[:90]!r}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")
    print(f"edited {relative}")


SLOT = r'''
/// 📡️ A host fact that may end a window's open gesture (law table `hostEvents` of `🧫️fixtures/🧫️gesture-drive-law`): the
/// window lost focus or its pointer capture, its utility switched, it is closing, a history edit froze the document, or a
/// remote edit moved the base.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GestureHostEvent {
    Blur,
    CaptureLost,
    UtilityChanged,
    Retiring,
    TimeTravelFrozen,
    BaseMoved,
}

impl GestureHostEvent {
    pub const ALL: [Self; 6] = [Self::Blur, Self::CaptureLost, Self::UtilityChanged, Self::Retiring, Self::TimeTravelFrozen, Self::BaseMoved];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::CaptureLost => "captureLost",
            Self::UtilityChanged => "utilityChanged",
            Self::Retiring => "retiring",
            Self::TimeTravelFrozen => "timeTravelFrozen",
            Self::BaseMoved => "baseMoved",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|event| event.as_str() == text)
    }

    /// 🧨️ The reason this fact ends an open gesture with; `None` keeps the gesture: a moved base ends only a gesture pinned
    /// to a base revision (`base_bound`), every other fact ends any gesture.
    pub fn abort_reason(self, base_bound: bool) -> Option<ToolAbortReason> {
        match self {
            Self::Blur => Some(ToolAbortReason::Blur),
            Self::CaptureLost => Some(ToolAbortReason::CaptureLost),
            Self::UtilityChanged | Self::Retiring => Some(ToolAbortReason::Retired),
            Self::TimeTravelFrozen => Some(ToolAbortReason::Frozen),
            Self::BaseMoved => base_bound.then_some(ToolAbortReason::BaseMoved),
        }
    }
}

/// 🫧️ One window's open gesture between dispatches — the framework-owned persisted form of a streamed statechart tool
/// (`$defs/GestureState`; window slot of the plugin runtime: ephemeral, local-only, never history): the configuration by
/// stable ids, the verb, the admission's authoring seed and the document revision it opened on (empty: pinned to none), the
/// open transaction with its keyed provisional mutations, and the tool context its entries do not already say (`Null`: none).
#[derive(Clone, Debug, PartialEq)]
pub struct GestureState<M> {
    pub states: Vec<String>,
    pub verb: String,
    pub authoring_seed: String,
    pub base_revision: String,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
    pub context: protocol::DslValue,
}

/// 🧭️ A statechart tool on the shared gesture runner: the chart, its host, the event one dispatch sends and how a resumed
/// gesture gets its chart context back. Starting, resuming, persisting and aborting are [`ChartGesture`]'s, the persisted
/// form is [`GestureState`], the window slot is the runtime's [`GestureLedger`] — a plugin declares none of them.
pub trait GestureChart: ToolMachine
where
    Self::Input: Clone,
{
    type Tick;
    type Host: Host<Self>;

    /// 🧲️ Whether a gesture is pinned to the document revision it opened on, so a moved base ends it `baseMoved`; `false`
    /// for a tool whose leaves fold on any base.
    const BASE_BOUND: bool = true;

    /// 🪛️ The authoring tool id `<appId>#<verb>` every transaction of `verb` is stamped with.
    fn tool(verb: &str) -> String;

    /// 🏠️ The host the chart's timers, invokes and foreign effects go to.
    fn host() -> Self::Host;

    /// 🥚️ The input the chart starts from.
    fn input() -> Self::Input;

    /// 🧶️ The chart context of a resumed gesture, rebuilt from its open transaction's entries and the context it persisted;
    /// `None` refuses the gesture (`Closed`), so the drive drops it with zero trace.
    fn restore(entries: &[(String, Self::Mutation)], context: &protocol::DslValue) -> Option<Self::Context>;

    /// 🎒️ The tool context a gesture persists beyond its entries.
    fn context(_context: &Self::Context) -> protocol::DslValue {
        protocol::DslValue::Null
    }

    /// 📣️ The event one dispatch sends (`at_rest`: no gesture is in flight); `None` sends nothing.
    fn event(phase: GesturePhase, at_rest: bool, tick: Option<Self::Tick>) -> Option<Self::Event>;
}

static GESTURE_CLOCK_TICK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// 🎛️ The ONE [`GestureTool`] of every statechart tool: a [`ToolMachineRunner`] over a [`GestureChart`], started at rest
/// or resumed from its window's [`GestureState`], each event on the host clock with a process-monotone tick (two gestures
/// opened in one millisecond never mint one transaction id).
pub struct ChartGesture<T: GestureChart>
where
    T::Input: Clone,
{
    runner: ToolMachineRunner<T, T::Host>,
    verb: String,
    authoring_seed: String,
    base_revision: String,
}

impl<T: GestureChart> GestureTool for ChartGesture<T>
where
    T::Input: Clone,
    T::Mutation: Clone + PartialEq,
{
    type Gesture = GestureState<T::Mutation>;
    type Tick = T::Tick;
    type Mutation = T::Mutation;

    const BASE_BOUND: bool = T::BASE_BOUND;

    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal> {
        let runner = ToolMachineRunner::start(T::tool(verb), ActorId(authoring_seed.to_string()), T::input(), T::host())?;
        Ok(Self { runner, verb: verb.to_string(), authoring_seed: authoring_seed.to_string(), base_revision: base_revision.to_string() })
    }

    fn resume(gesture: &GestureState<T::Mutation>) -> Result<Self, ToolRefusal> {
        let context = T::restore(&gesture.entries, &gesture.context).ok_or(ToolRefusal::Closed)?;
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: T::definition().fingerprint, states: gesture.states.clone(), history: Vec::new(), done: false };
        let snapshot = machine::restore::<T, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(gesture.transaction.clone(), gesture.entries.clone());
        let runner = ToolMachineRunner::resume(T::tool(&gesture.verb), ActorId(gesture.authoring_seed.clone()), T::input(), snapshot, Some(transaction), T::host())?;
        Ok(Self { runner, verb: gesture.verb.clone(), authoring_seed: gesture.authoring_seed.clone(), base_revision: gesture.base_revision.clone() })
    }

    fn verb(&self) -> &str {
        &self.verb
    }

    fn base_revision(&self) -> &str {
        &self.base_revision
    }

    fn abort(&mut self, reason: ToolAbortReason) {
        self.runner.abort(reason);
    }

    fn send(&mut self, phase: GesturePhase, tick: Option<T::Tick>) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        match T::event(phase, self.runner.at_rest(), tick) {
            Some(event) => self.runner.send(event, authoring_clock(GESTURE_CLOCK_TICK.fetch_add(1, std::sync::atomic::Ordering::Relaxed))),
            None if self.runner.transaction().is_some() => Ok(ToolStep::Open),
            None => Ok(ToolStep::Idle),
        }
    }

    fn persist(self) -> Option<GestureState<T::Mutation>> {
        let context = T::context(&self.runner.snapshot().context);
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        Some(GestureState {
            states: machine::persist(&snapshot).states,
            verb: self.verb,
            authoring_seed: self.authoring_seed,
            base_revision: self.base_revision,
            transaction: transaction.reference().clone(),
            entries: transaction.entries().to_vec(),
            context,
        })
    }
}

/// 🎬️ Drives the statechart tool `T` through ONE dispatch from its window's persisted gesture ([`drive_gesture`] over
/// [`ChartGesture`]); a tool that is not [`GestureChart::BASE_BOUND`] pins its gesture to no revision.
pub fn drive_chart_gesture<T: GestureChart>(
    persisted: Option<&GestureState<T::Mutation>>,
    verb: &str,
    phase: GesturePhase,
    tick: Option<T::Tick>,
    authoring_seed: &str,
    base_revision: &str,
) -> Result<GestureDrive<GestureState<T::Mutation>, T::Mutation>, ToolRefusal>
where
    T::Input: Clone,
    T::Mutation: Clone + PartialEq,
{
    drive_gesture::<ChartGesture<T>>(persisted, verb, phase, tick, authoring_seed, if T::BASE_BOUND { base_revision } else { "" })
}

/// 🗄️ Every window's open gesture, at most one per window — the ONE framework-owned window slot of a persisted
/// [`GestureTool`] (design §22.10, law `slots`). Pure: the runtime keeps one per app instance, overlays
/// [`Self::provisional`] on the committed document for every render, and ends a window's gesture on its host facts
/// ([`Self::host_event`]) — no editor maps a host fact to an abort of its own. A persisted gesture holds no host timer.
#[derive(Clone, Debug, PartialEq)]
pub struct GestureLedger<M> {
    windows: std::collections::BTreeMap<String, GestureState<M>>,
}

impl<M> Default for GestureLedger<M> {
    fn default() -> Self {
        Self { windows: std::collections::BTreeMap::new() }
    }
}

impl<M> GestureLedger<M> {
    /// 🪹️ Whether no window holds an open gesture.
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    /// 🔦️ The open gesture of `window`.
    pub fn open(&self, window: &str) -> Option<&GestureState<M>> {
        self.windows.get(window)
    }

    /// 🏘️ The windows holding an open gesture, in window id order.
    pub fn windows(&self) -> impl Iterator<Item = &str> {
        self.windows.keys().map(String::as_str)
    }

    /// 🪞️ Every open gesture's provisional mutations, window by window in window id order — the overlay a render applies
    /// on the committed document; never history.
    pub fn provisional(&self) -> impl Iterator<Item = &M> {
        self.windows.values().flat_map(|gesture| gesture.entries.iter().map(|(_, mutation)| mutation))
    }

    /// 🖋️ Keeps the gesture a dispatch decided for `window` (`None` clears the slot).
    pub fn settle(&mut self, window: &str, next: Option<GestureState<M>>) {
        match next {
            Some(gesture) => {
                self.windows.insert(window.to_string(), gesture);
            }
            None => {
                self.windows.remove(window);
            }
        }
    }

    /// 🚃️ Drives `window`'s tool through ONE dispatch against its slot ([`drive_gesture`]): the slot follows the drive and
    /// the committed transaction is answered; a refused dispatch leaves the ledger exactly as it was.
    pub fn drive<T: GestureTool<Gesture = GestureState<M>, Mutation = M>>(
        &mut self,
        window: &str,
        verb: &str,
        phase: GesturePhase,
        tick: Option<T::Tick>,
        authoring_seed: &str,
        base_revision: &str,
    ) -> Result<Option<(TransactionRef, Vec<M>)>, ToolRefusal> {
        let drive = drive_gesture::<T>(self.windows.get(window), verb, phase, tick, authoring_seed, base_revision)?;
        if let Some(next) = drive.next {
            self.settle(window, next);
        }
        Ok(drive.committed)
    }

    /// 🧹️ Host cancel of `window`'s open gesture: zero trace. `Aborted(ref, reason)`, or `Idle` when none was open.
    pub fn abort(&mut self, window: &str, reason: ToolAbortReason) -> ToolStep<M> {
        self.windows.remove(window).map_or(ToolStep::Idle, |gesture| ToolStep::Aborted(gesture.transaction, reason))
    }

    /// 🛎️ A host fact of `window`: its open gesture ends with the fact's reason ([`GestureHostEvent::abort_reason`]; a
    /// gesture pinned to no revision survives a moved base), `Idle` when it stays or none was open.
    pub fn host_event(&mut self, window: &str, event: GestureHostEvent) -> ToolStep<M> {
        match self.windows.get(window).and_then(|gesture| event.abort_reason(!gesture.base_revision.is_empty())) {
            Some(reason) => self.abort(window, reason),
            None => ToolStep::Idle,
        }
    }

    /// 🌐️ A host fact of every window (a history edit freezing the document): one `(window, Aborted)` per gesture it ended,
    /// in window id order.
    pub fn host_event_all(&mut self, event: GestureHostEvent) -> Vec<(String, ToolStep<M>)> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.into_iter().map(|window| (self.host_event(&window, event), window)).filter(|(step, _)| !matches!(step, ToolStep::Idle)).map(|(step, window)| (window, step)).collect()
    }

    /// ⚰️ Host cancel (`retired`) of the open gesture of every window `keep` refuses.
    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool) -> Vec<(String, ToolStep<M>)> {
        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).cloned().collect();
        retired.into_iter().map(|window| (self.abort(&window, ToolAbortReason::Retired), window)).map(|(step, window)| (window, step)).collect()
    }
}
//#endregion 🌊️Gesture'''

edit(
    "🦀️.rs",
    [
        (
            "/// its window persisted between dispatches (window or artifact transient, never history).\npub trait GestureTool: Sized {",
            "/// its window persisted between dispatches (the runtime's [`GestureLedger`] slot, never history). A statechart tool is\n/// [`ChartGesture`] over its [`GestureChart`].\npub trait GestureTool: Sized {",
        ),
        (
            "    type Gesture: PartialEq;\n    type Tick;\n    type Mutation;\n    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal>;",
            "    type Gesture: PartialEq;\n    type Tick;\n    type Mutation;\n    /// 📌️ Whether a gesture is pinned to the document revision it opened on, so a moved base ends it `baseMoved`; a tool\n    /// whose leaves fold on any base says `false` and is driven with an empty base revision.\n    const BASE_BOUND: bool = true;\n    fn start(verb: &str, authoring_seed: &str, base_revision: &str) -> Result<Self, ToolRefusal>;",
        ),
        ("    Ok(GestureDrive { committed, next })\n}\n//#endregion 🌊️Gesture", "    Ok(GestureDrive { committed, next })\n}\n" + SLOT),
    ],
)

LAW = r'''

//#region 🗄️Ledger
/// 🔢️ The counting tool as a real statechart on the shared runner: a stream tick upserts `tick:<n>` (the gesture is open
/// while it has ticks), a one-shot or commit folds its tick in and commits, a negative tick leaves the chart at rest with
/// its transaction open — the runner's `Unclosed` refusal (the fixture's `sendRefusal`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CounterContext {
    ticks: usize,
}

/// 🎟️ What one counting dispatch carries: its tick, if any.
type CounterTick = Option<i64>;

fn counter_context(input: CounterContext) -> CounterContext {
    input
}

fn counter_tick(event: Option<&counter::Event>) -> Option<i64> {
    match event {
        Some(counter::Event::Once(tick) | counter::Event::Stream(tick) | counter::Event::Finish(tick)) => *tick,
        _ => None,
    }
}

fn counter_ticks(_context: &CounterContext, event: Option<&counter::Event>) -> bool {
    counter_tick(event).is_some()
}

fn counter_refuses(_context: &CounterContext, event: Option<&counter::Event>) -> bool {
    counter_tick(event).is_some_and(|tick| tick < 0)
}

fn counter_upsert(context: &mut CounterContext, event: Option<&counter::Event>, sink: &mut Vec<Command<counter::Counter>>) {
    if let Some(tick) = counter_tick(event) {
        sink.push(Command::Effect(ToolYield::upsert(format!("tick:{}", context.ticks), tick)));
        context.ticks += 1;
    }
}

fn counter_commit(context: &mut CounterContext, event: Option<&counter::Event>, sink: &mut Vec<Command<counter::Counter>>) {
    counter_upsert(context, event, sink);
    sink.push(Command::Effect(ToolYield::Commit));
    context.ticks = 0;
}

machine::statechart! {
    machine counter {
        context: CounterContext;
        event Event { Once(CounterTick), Stream(CounterTick), Finish(CounterTick) }
        input: CounterContext;
        output: ();
        effect: ToolYield<i64>;
        context_from_input: counter_context;
        initial: idle;
        state idle {
            on Once if counter_refuses => idle do counter_upsert;
            on Stream if counter_refuses => idle do counter_upsert;
            on Once if counter_ticks => idle do counter_commit;
            on Stream if counter_ticks => streaming do counter_upsert;
        }
        state streaming {
            on Stream if counter_refuses => idle do counter_upsert;
            on Finish if counter_refuses => idle do counter_upsert;
            on Stream => streaming do counter_upsert;
            on Finish => idle do counter_commit;
        }
    }
}

/// 🏚️ The counting chart's host: no timer, no invoke, no foreign effect.
pub struct CounterHost;

impl Host<counter::Counter> for CounterHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<i64>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        0
    }
}

impl GestureChart for counter::Counter {
    type Tick = i64;
    type Host = CounterHost;

    fn tool(verb: &str) -> String {
        format!("law#{verb}")
    }

    fn host() -> CounterHost {
        CounterHost
    }

    fn input() -> CounterContext {
        CounterContext::default()
    }

    fn restore(entries: &[(String, i64)], context: &protocol::DslValue) -> Option<CounterContext> {
        matches!(context, protocol::DslValue::Null).then_some(CounterContext { ticks: entries.len() })
    }

    fn event(phase: GesturePhase, at_rest: bool, tick: Option<i64>) -> Option<counter::Event> {
        match phase {
            GesturePhase::Stream => Some(counter::Event::Stream(tick)),
            GesturePhase::Commit if !at_rest => Some(counter::Event::Finish(tick)),
            GesturePhase::Once | GesturePhase::Commit => Some(counter::Event::Once(tick)),
            GesturePhase::Abort(_) => None,
        }
    }
}

type Counting = ChartGesture<counter::Counter>;

/// 🪟️ A ledger slot in the fixture's gesture shape; `corrupt` marks one whose configuration the chart cannot restore.
fn slot_json(gesture: &GestureState<i64>) -> Value {
    let mut slot = json!({ "verb": gesture.verb, "base": gesture.base_revision, "ticks": gesture.entries.iter().map(|(_, tick)| *tick).collect::<Vec<_>>() });
    if gesture.states.iter().any(|state| state == "tampered") {
        slot["corrupt"] = json!(true);
    }
    slot
}

fn ended(aborted: Vec<(String, ToolStep<i64>)>) -> Value {
    Value::Array(
        aborted
            .into_iter()
            .map(|(window, step)| match step {
                ToolStep::Aborted(_, reason) => json!({ "window": window, "reason": reason.as_str() }),
                other => panic!("a host fact reported {}", other.kind()),
            })
            .collect(),
    )
}

/// 👣️ One slot step through the real ledger, reported in the fixture's outcome shape.
fn ledger_step(ledger: &mut GestureLedger<i64>, step: &Value) -> Value {
    let (mut committed, mut refused, mut aborted) = (Value::Null, Value::Null, json!([]));
    if let Some(drive) = step.get("drive") {
        let phase = GesturePhase::parse(drive["phase"].as_str(), drive["reason"].as_str()).expect("a known phase");
        match ledger.drive::<Counting>(text(&drive["window"]), text(&drive["verb"]), phase, drive["tick"].as_i64(), "seed", text(&drive["base"])) {
            Ok(Some((reference, ticks))) => {
                assert_eq!(reference.tool, format!("law#{}", text(&drive["verb"])), "a commit is stamped `<appId>#<verb>`");
                committed = json!(ticks);
            }
            Ok(None) => {}
            Err(refusal) => refused = json!(refusal.code()),
        }
    } else if let Some(host) = step.get("host") {
        let window = text(&host["window"]);
        let event = GestureHostEvent::parse(text(&host["event"])).expect("a known host fact");
        aborted = ended(Some(ledger.host_event(window, event)).filter(|step| !matches!(step, ToolStep::Idle)).map(|step| (window.to_string(), step)).into_iter().collect());
    } else if let Some(host) = step.get("hostAll") {
        aborted = ended(ledger.host_event_all(GestureHostEvent::parse(text(&host["event"])).expect("a known host fact")));
    } else if let Some(keep) = step.get("retain") {
        let keep: Vec<&str> = keep.as_array().expect("windows").iter().map(text).collect();
        aborted = ended(ledger.retain_windows(|window| keep.contains(&window)));
    } else {
        let window = text(&step["corrupt"]);
        let mut gesture = ledger.open(window).expect("a slot to tamper").clone();
        gesture.states = vec!["tampered".to_string()];
        ledger.settle(window, Some(gesture));
    }
    let open: serde_json::Map<String, Value> = ledger.windows().map(|window| (window.to_string(), slot_json(ledger.open(window).expect("a listed window is open")))).collect();
    json!({ "committed": committed, "refused": refused, "aborted": aborted, "open": open })
}

/// 📡️ LAW (design §22.10): every host fact ends an open gesture with the fixture's reason — a history edit `frozen`, a
/// blur `blur`, a lost capture `captureLost`, a utility switch or a closing window `retired` — and a moved base ends only
/// a gesture pinned to a base revision; the table covers every fact for both kinds of gesture.
#[test]
fn every_host_fact_ends_a_gesture_with_the_fixtures_reason() {
    let law = law();
    let rows = law["hostEvents"].as_array().expect("hostEvents");
    assert_eq!(rows.len(), GestureHostEvent::ALL.len() * 2);
    for event in GestureHostEvent::ALL {
        assert_eq!(GestureHostEvent::parse(event.as_str()), Some(event));
        for base_bound in [true, false] {
            let row = rows.iter().find(|row| text(&row["event"]) == event.as_str() && row["baseBound"] == json!(base_bound)).unwrap_or_else(|| panic!("no row for {} / {base_bound}", event.as_str()));
            assert_eq!(event.abort_reason(base_bound).map_or(Value::Null, |reason| json!(reason.as_str())), row["reason"], "{} / {base_bound}", event.as_str());
        }
    }
    assert_eq!(GestureHostEvent::TimeTravelFrozen.abort_reason(false), Some(ToolAbortReason::Frozen));
}

/// 🗄️ LAW (design §22.10): every slot scenario steps the real ledger — the statechart counting tool persisted by one
/// dispatch and resumed by the next — to the fixture's outcome: a gesture persists between dispatches and commits once, a
/// history edit ends every window's gesture with zero trace, windows never share a slot, a refused tick keeps the slot, an
/// unrestorable slot is dropped by the next dispatch.
#[test]
fn every_slot_scenario_steps_the_ledger_to_its_expected_outcome() {
    for scenario in law()["slots"].as_array().expect("slots") {
        let mut ledger = GestureLedger::<i64>::default();
        for (index, step) in scenario["steps"].as_array().expect("steps").iter().enumerate() {
            assert_eq!(ledger_step(&mut ledger, step), step["expected"], "{} step {index}", scenario["name"]);
        }
    }
}

/// 💾️ LAW (design §22.10): the persisted gesture is the framework's own form — one stream tick persists the chart's
/// configuration by stable id, the verb, the admission and revision it opened on and its open transaction; resuming and
/// persisting it again is the identity; the ledger's provisional overlay is exactly its entries; a gesture persisted with a
/// context its chart cannot restore, or resting with an open transaction, is refused and dropped with zero trace.
#[test]
fn a_persisted_gesture_resumes_to_itself_and_refuses_what_its_chart_cannot_restore() {
    let mut ledger = GestureLedger::<i64>::default();
    assert_eq!(ledger.drive::<Counting>("w1", "drag", GesturePhase::Stream, Some(4), "seed", "r1"), Ok(None));
    let gesture = ledger.open("w1").expect("the tick opened the gesture").clone();
    assert!(gesture.states.iter().any(|state| state == "streaming") && !gesture.states.iter().any(|state| state == "idle"), "the configuration is persisted by stable id: {:?}", gesture.states);
    assert_eq!((gesture.verb.as_str(), gesture.authoring_seed.as_str(), gesture.base_revision.as_str()), ("drag", "seed", "r1"));
    assert_eq!((gesture.transaction.tool.as_str(), gesture.entries.as_slice(), &gesture.context), ("law#drag", [("tick:0".to_string(), 4)].as_slice(), &protocol::DslValue::Null));
    assert_eq!(Counting::resume(&gesture).expect("the gesture resumes").persist(), Some(gesture.clone()));
    assert_eq!(ledger.provisional().copied().collect::<Vec<_>>(), vec![4]);
    assert_eq!(ledger.drive::<Counting>("w1", "drag", GesturePhase::Stream, None, "other", "r1"), Ok(None));
    assert_eq!(ledger.open("w1"), Some(&gesture), "a tickless stream resumes and keeps the gesture");
    let unknown = GestureState { context: protocol::DslValue::Bool(true), ..gesture.clone() };
    assert_eq!(Counting::resume(&unknown).err(), Some(ToolRefusal::Closed));
    let resting = GestureState { states: vec!["idle".to_string()], ..gesture.clone() };
    assert_eq!(Counting::resume(&resting).err(), Some(ToolRefusal::Unclosed));
    ledger.settle("w1", Some(resting));
    assert_eq!(ledger.drive::<Counting>("w1", "drag", GesturePhase::Commit, None, "seed", "r1"), Ok(None));
    assert!(ledger.is_empty(), "an unrestorable gesture is dropped with zero trace");
    assert_eq!(drive_chart_gesture::<counter::Counter>(None, "drag", GesturePhase::Once, Some(7), "seed", "r1").expect("a one-shot at rest").committed.map(|(_, ticks)| ticks), Some(vec![7]));
}
//#endregion 🗄️Ledger
'''

path = root / "🧪️tests/🧪️gesture-drive-law/🦀️.rs"
text = path.read_text(encoding="utf-8")
if "🗄️Ledger" in text:
    raise SystemExit("law already applied")
old_head = "//! table, and one dispatch per row from the window's persisted gesture with the abort reasons the drive called.\n"
if text.count(old_head) != 1:
    raise SystemExit("law header moved")
text = text.replace(old_head, "//! table, one dispatch per row from the window's persisted gesture with the abort reasons the drive called, the reason\n//! each host fact ends an open gesture with, and step sequences over the per-window ledger (design §22.10) driven by the\n//! counting tool as a real statechart on the shared runner.\n")
path.write_text(text.rstrip("\n") + LAW, encoding="utf-8")
print("edited 🧪️tests/🧪️gesture-drive-law/🦀️.rs")
