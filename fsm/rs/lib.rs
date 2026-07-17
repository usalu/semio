//! 🎭 Statechart kernel, actor runtime and hosts — XState-parity FSMs as static tables.
//!
//! Machine structure is compile-time (dense static tables addressed by `NodeId`s);
//! machine state and actor instances are runtime. See [`Machine`] and [`statechart!`].

extern crate self as fsm;

mod host {
    //! 🌐 Host abstraction — hosts execute the commands the kernel only describes.

    use crate::{ActorId, InvokeId, Machine, TimerId};

    //#region 🔖Host

    /// 🌐 Executes the side effects a [`crate::Command`] describes. No `async fn` —
    /// hosts own their own tasks/timers and report completion back as ordinary events.
    pub trait Host<M: Machine> {
        /// 🎇 Executes a consumer-defined effect requested by a running actor.
        fn execute_effect(&mut self, actor: ActorId, effect: M::Effect);
        /// ⏱️ Schedules a delayed-transition timer for the given actor.
        fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64);
        /// ⏱️ Cancels a previously scheduled timer (invoked when its owning state exits).
        fn cancel_timer(&mut self, actor: ActorId, timer: TimerId);
        /// 🚀 Starts the task/actor backing an `invoke` declaration.
        fn start_task(&mut self, actor: ActorId, invoke: InvokeId);
        /// 🛑 Stops a previously started task (invoked when its owning state exits).
        fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId);
        /// 🕰️ The host's current clock reading, in milliseconds.
        fn now_ms(&self) -> u64;
    }

    //#endregion 🔖Host

    //#region 🔖NativeHost

    /// 🖥️ A synchronous, wall-clock-backed [`Host`] for native (non-WASM) targets.
    ///
    /// Timers are polled by the caller via [`NativeHost::due_timers`] rather than
    /// firing on their own thread — keeping the whole runtime single-threaded per actor.
    pub struct NativeHost<M: Machine> {
        start: std::time::Instant,
        effects: Vec<(ActorId, M::Effect)>,
        pending_timers: Vec<(ActorId, TimerId, u64)>,
        started_tasks: Vec<(ActorId, InvokeId)>,
        cancelled_tasks: Vec<(ActorId, InvokeId)>,
    }

    impl<M: Machine> NativeHost<M> {
        /// 🖥️ A fresh host whose clock starts at zero.
        pub fn new() -> Self {
            Self {
                start: std::time::Instant::now(),
                effects: Vec::new(),
                pending_timers: Vec::new(),
                started_tasks: Vec::new(),
                cancelled_tasks: Vec::new(),
            }
        }

        /// 🎇 Effects recorded so far, in emission order.
        pub fn effects(&self) -> &[(ActorId, M::Effect)] {
            &self.effects
        }

        /// 🎇 Drains and returns every recorded effect.
        pub fn drain_effects(&mut self) -> Vec<(ActorId, M::Effect)> {
            core::mem::take(&mut self.effects)
        }

        /// 🚀 Tasks started via `invoke`, still pending cancellation.
        pub fn started_tasks(&self) -> &[(ActorId, InvokeId)] {
            &self.started_tasks
        }

        /// ⏱️ Removes and returns every timer whose deadline has passed.
        pub fn due_timers(&mut self) -> Vec<(ActorId, TimerId)> {
            let now = self.now_ms();
            let mut due = Vec::new();
            self.pending_timers.retain(|(actor, timer, at)| {
                if *at <= now {
                    due.push((*actor, *timer));
                    false
                } else {
                    true
                }
            });
            due
        }
    }

    impl<M: Machine> Default for NativeHost<M> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<M: Machine> Host<M> for NativeHost<M> {
        fn execute_effect(&mut self, actor: ActorId, effect: M::Effect) {
            self.effects.push((actor, effect));
        }

        fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64) {
            let at = self.now_ms() + delay_ms;
            self.pending_timers.push((actor, timer, at));
        }

        fn cancel_timer(&mut self, actor: ActorId, timer: TimerId) {
            self.pending_timers.retain(|(a, t, _)| !(*a == actor && *t == timer));
        }

        fn start_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.push((actor, invoke));
        }

        fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.retain(|(a, i)| !(*a == actor && *i == invoke));
            self.cancelled_tasks.push((actor, invoke));
        }

        fn now_ms(&self) -> u64 {
            self.start.elapsed().as_millis() as u64
        }
    }

    //#endregion 🔖NativeHost

    //#region 🔖TestHost

    /// 🧪 A [`Host`] with a caller-driven simulated clock — never sleeps in real time.
    pub struct TestHost<M: Machine> {
        clock_ms: u64,
        effects: Vec<(ActorId, M::Effect)>,
        pending_timers: Vec<(ActorId, TimerId, u64)>,
        started_tasks: Vec<(ActorId, InvokeId)>,
        cancelled_tasks: Vec<(ActorId, InvokeId)>,
    }

    impl<M: Machine> TestHost<M> {
        /// 🧪 A fresh simulated host whose clock starts at zero.
        pub fn new() -> Self {
            Self {
                clock_ms: 0,
                effects: Vec::new(),
                pending_timers: Vec::new(),
                started_tasks: Vec::new(),
                cancelled_tasks: Vec::new(),
            }
        }

        /// 🎇 Effects recorded so far, in emission order.
        pub fn effects(&self) -> &[(ActorId, M::Effect)] {
            &self.effects
        }

        /// 🚀 Tasks currently started (not yet cancelled), for invoke-lifecycle assertions.
        pub fn started_tasks(&self) -> &[(ActorId, InvokeId)] {
            &self.started_tasks
        }

        /// 🛑 Tasks that have been cancelled, for invoke-lifecycle assertions.
        pub fn cancelled_tasks(&self) -> &[(ActorId, InvokeId)] {
            &self.cancelled_tasks
        }

        /// ⏱️ Advances the simulated clock and returns timers that became due, removing them.
        pub fn advance(&mut self, delay_ms: u64) -> Vec<(ActorId, TimerId)> {
            self.clock_ms += delay_ms;
            let now = self.clock_ms;
            let mut due = Vec::new();
            self.pending_timers.retain(|(actor, timer, at)| {
                if *at <= now {
                    due.push((*actor, *timer));
                    false
                } else {
                    true
                }
            });
            due
        }
    }

    impl<M: Machine> Default for TestHost<M> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<M: Machine> Host<M> for TestHost<M> {
        fn execute_effect(&mut self, actor: ActorId, effect: M::Effect) {
            self.effects.push((actor, effect));
        }

        fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64) {
            self.pending_timers.push((actor, timer, self.clock_ms + delay_ms));
        }

        fn cancel_timer(&mut self, actor: ActorId, timer: TimerId) {
            self.pending_timers.retain(|(a, t, _)| !(*a == actor && *t == timer));
        }

        fn start_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.push((actor, invoke));
        }

        fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.retain(|(a, i)| !(*a == actor && *i == invoke));
            self.cancelled_tasks.push((actor, invoke));
        }

        fn now_ms(&self) -> u64 {
            self.clock_ms
        }
    }

    //#endregion 🔖TestHost

    //#region 🧪Tests

    #[cfg(test)]
    mod tests {
        use super::*;

        struct DummyMachine;
        impl Machine for DummyMachine {
            type Context = ();
            type Event = crate::testing::support::UnitEvent;
            type Input = ();
            type Output = ();
            type Effect = &'static str;
            type Config = crate::BitSet<1>;
            fn definition() -> &'static crate::kernel::MachineDefinition<Self> {
                unimplemented!("host tests never step a machine")
            }
        }

        #[test]
        fn test_host_advance_fires_due_timers_only() {
            let mut host = TestHost::<DummyMachine>::new();
            host.schedule(ActorId(0), TimerId(0), 100);
            host.schedule(ActorId(0), TimerId(1), 300);
            let due = host.advance(150);
            assert_eq!(due, vec![(ActorId(0), TimerId(0))]);
            let due = host.advance(200);
            assert_eq!(due, vec![(ActorId(0), TimerId(1))]);
        }

        #[test]
        fn test_host_cancel_timer_removes_pending() {
            let mut host = TestHost::<DummyMachine>::new();
            host.schedule(ActorId(0), TimerId(0), 100);
            host.cancel_timer(ActorId(0), TimerId(0));
            assert_eq!(host.advance(200), Vec::new());
        }

        #[test]
        fn test_host_records_effects_and_task_lifecycle() {
            let mut host = TestHost::<DummyMachine>::new();
            host.execute_effect(ActorId(0), "audit");
            assert_eq!(host.effects(), &[(ActorId(0), "audit")]);
            host.start_task(ActorId(0), InvokeId(0));
            assert_eq!(host.started_tasks(), &[(ActorId(0), InvokeId(0))]);
            host.cancel_task(ActorId(0), InvokeId(0));
            assert!(host.started_tasks().is_empty());
            assert_eq!(host.cancelled_tasks(), &[(ActorId(0), InvokeId(0))]);
        }
    }

    //#endregion 🧪Tests
}
mod inspect {
    //! 🔎 Inspection protocol — structured, microstep-granular observation of a running machine.

    use crate::kernel::Command;
    use crate::{Machine, NodeId};

    //#region 🔖Inspection

    /// 🔎 One structured observation emitted while a macrostep runs to completion.
    pub enum InspectionEvent<'a, M: Machine> {
        /// 🏁 A macrostep began processing an external event or timer.
        MacrostepStart,
        /// 🔬 One microstep exited/entered the given nodes.
        Microstep {
            exited: &'a [NodeId],
            entered: &'a [NodeId],
        },
        /// 🎇 A command was pushed to the outer sink.
        CommandIssued(&'a Command<M>),
        /// 🧊 The macrostep settled after this many microsteps.
        Settled { microsteps: u32 },
    }

    /// 🔎 Observer of [`InspectionEvent`]s — implemented by hosts/tooling that need microstep visibility.
    pub trait Inspector<M: Machine> {
        /// 👀 Called once per [`InspectionEvent`] in emission order.
        fn observe(&mut self, event: InspectionEvent<'_, M>);
    }

    /// 🔎 An [`Inspector`] that discards every event — the default for callers that don't need tracing.
    pub struct NullInspector;

    impl<M: Machine> Inspector<M> for NullInspector {
        fn observe(&mut self, _event: InspectionEvent<'_, M>) {}
    }

    /// 🔎 One recorded microstep — the exited/entered node sets, in kernel-execution order.
    #[derive(Clone, Debug, Default)]
    pub struct MicrostepTrace {
        pub exited: Vec<NodeId>,
        pub entered: Vec<NodeId>,
    }

    /// 🔎 An [`Inspector`] that records every microstep for later assertion/replay.
    pub struct TraceInspector<M: Machine> {
        pub entries: Vec<MicrostepTrace>,
        _marker: core::marker::PhantomData<M>,
    }

    impl<M: Machine> Default for TraceInspector<M> {
        fn default() -> Self {
            Self {
                entries: Vec::new(),
                _marker: core::marker::PhantomData,
            }
        }
    }

    impl<M: Machine> Inspector<M> for TraceInspector<M> {
        fn observe(&mut self, event: InspectionEvent<'_, M>) {
            if let InspectionEvent::Microstep { exited, entered } = event {
                self.entries.push(MicrostepTrace {
                    exited: exited.to_vec(),
                    entered: entered.to_vec(),
                });
            }
        }
    }

    //#endregion 🔖Inspection

    //#region 🧪Tests

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn null_inspector_observes_nothing_observable() {
            // Compile-only smoke test: NullInspector must be constructible and callable
            // without a concrete Machine — exercised indirectly by kernel tests.
            let _inspector = NullInspector;
        }

        #[test]
        fn trace_inspector_records_one_microstep_per_transition() {
            use crate::kernel::{init, macrostep};
            use crate::testing::support::{UnitToggleEvent, UnitToggleMachine};

            let mut sink = Vec::new();
            let mut snapshot = init::<UnitToggleMachine>((), &mut sink);
            let mut inspector = TraceInspector::<UnitToggleMachine>::default();
            macrostep(&mut snapshot, UnitToggleEvent::Flip, &mut sink, &mut inspector);
            macrostep(&mut snapshot, UnitToggleEvent::Flip, &mut sink, &mut inspector);

            assert_eq!(inspector.entries.len(), 2);
            assert_eq!(inspector.entries[0].exited, vec![NodeId(1)]);
            assert_eq!(inspector.entries[0].entered, vec![NodeId(2)]);
            assert_eq!(inspector.entries[1].exited, vec![NodeId(2)]);
            assert_eq!(inspector.entries[1].entered, vec![NodeId(1)]);
        }
    }

    //#endregion 🧪Tests
}
mod kernel {
    //! 🧠 Pure statechart core — SCXML-style run-to-completion over dense static tables.
    //!
    //! Nothing in this module executes I/O, sleeps, or reaches a host: every effectful
    //! request becomes a [`Command`] pushed to a [`CommandSink`] for the caller to route.

    use crate::inspect::{InspectionEvent, Inspector};
    use crate::{ActionId, ActorId, Configuration, EventId, GuardId, InvokeId, Machine, NodeId, StatechartEvent, TimerId};
    use std::collections::VecDeque;

    //#region 🔖Tables

    /// 🌳 The structural kind of a state node.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NodeKind {
        Atomic,
        Compound,
        Parallel,
        Final,
        HistoryShallow,
        HistoryDeep,
    }

    /// 🌳 One compiled state node. `NodeId(0)` is always the synthetic root wrapping
    /// the whole machine, so [`compute_domain`] always terminates.
    #[derive(Debug)]
    pub struct NodeDef {
        pub stable_id: &'static str,
        pub kind: NodeKind,
        pub parent: Option<NodeId>,
        /// Compound: default child to enter. History: fallback target when no history is recorded.
        pub initial: Option<NodeId>,
        pub children: &'static [NodeId],
        pub entry_actions: &'static [ActionId],
        pub exit_actions: &'static [ActionId],
        pub invokes: &'static [InvokeId],
        /// `after` delayed transitions owned by this state: `(timer, delay_ms)`.
        pub timers: &'static [(TimerId, u64)],
        pub doc_index: u16,
    }

    /// 🔔 What causes a [`TransitionDef`] to become a candidate during a microstep.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Trigger {
        Event(EventId),
        Eventless,
        /// Fires once every descendant of `NodeId` reaches a final state (`on_done`).
        Done(NodeId),
        /// Fires when the named `after` timer elapses (delivered via [`timer_elapsed`]).
        Timer(TimerId),
    }

    /// 🔀 External transitions exit+re-enter their source; internal transitions to a
    /// descendant of a compound source leave the source itself active.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TransitionKind {
        External,
        Internal,
    }

    /// 🔀 One compiled transition. Guard/action indices are dense — the fn-pointer
    /// tables they index into live on [`MachineDefinition`], keyed by the consumer's `M`.
    #[derive(Debug)]
    pub struct TransitionDef {
        pub source: NodeId,
        pub trigger: Trigger,
        pub guard: Option<GuardId>,
        pub targets: &'static [NodeId],
        pub kind: TransitionKind,
        pub actions: &'static [ActionId],
        pub doc_index: u16,
    }

    /// 🔢 `NodeId(0)` is always the synthetic root — see [`NodeDef`].
    pub const ROOT: NodeId = NodeId(0);

    /// 🧵 Reads context+event, decides whether a guarded transition may fire. Pure — no I/O, no mutation.
    pub type GuardFn<M> = fn(&<M as Machine>::Context, Option<&<M as Machine>::Event>) -> bool;

    /// 🧵 Mutates context and/or pushes commands; used for entry/exit/transition actions alike.
    pub type ActionFn<M> = fn(&mut <M as Machine>::Context, Option<&<M as Machine>::Event>, &mut dyn CommandSink<M>);

    /// 🧵 Builds the initial context from consumer-supplied input.
    pub type InputFn<M> = fn(<M as Machine>::Input) -> <M as Machine>::Context;

    /// 🧵 Builds the machine's output once the root reaches a fully-final configuration.
    pub type OutputFn<M> = fn(&<M as Machine>::Context) -> <M as Machine>::Output;

    /// 📐 The compiled, `'static` definition of a machine — dense tables, no HashMap, no string dispatch.
    pub struct MachineDefinition<M: Machine> {
        pub id: &'static str,
        pub nodes: &'static [NodeDef],
        pub transitions: &'static [TransitionDef],
        pub context_from_input: InputFn<M>,
        pub make_output: Option<OutputFn<M>>,
        pub guards: &'static [GuardFn<M>],
        pub actions: &'static [ActionFn<M>],
        /// Stable hash of the compiled structure — used to gate [`crate::persist::restore`].
        pub fingerprint: u64,
        pub manifest_json: &'static str,
    }

    //#endregion 🔖Tables

    //#region 🔖Configuration

    fn is_descendant(nodes: &[NodeDef], a: NodeId, ancestor: NodeId) -> bool {
        if a == ancestor {
            return false;
        }
        let mut cur = nodes[a.0 as usize].parent;
        while let Some(p) = cur {
            if p == ancestor {
                return true;
            }
            cur = nodes[p.0 as usize].parent;
        }
        false
    }

    fn is_descendant_or_self(nodes: &[NodeDef], a: NodeId, ancestor: NodeId) -> bool {
        a == ancestor || is_descendant(nodes, a, ancestor)
    }

    fn depth_of(nodes: &[NodeDef], id: NodeId) -> u32 {
        let mut depth = 0;
        let mut cur = nodes[id.0 as usize].parent;
        while let Some(p) = cur {
            depth += 1;
            cur = nodes[p.0 as usize].parent;
        }
        depth
    }

    fn is_compound_or_parallel(nodes: &[NodeDef], id: NodeId) -> bool {
        matches!(nodes[id.0 as usize].kind, NodeKind::Compound | NodeKind::Parallel)
    }

    fn is_leafish(nodes: &[NodeDef], id: NodeId) -> bool {
        matches!(nodes[id.0 as usize].kind, NodeKind::Atomic | NodeKind::Final)
    }

    /// 🌳 The transition domain per SCXML `getTransitionDomain` — the innermost compound/parallel
    /// ancestor whose descendants fully cover source+targets. Always terminates at [`ROOT`].
    fn compute_domain(nodes: &[NodeDef], source: NodeId, targets: &[NodeId], kind: TransitionKind) -> NodeId {
        if targets.is_empty() {
            return source;
        }
        if kind == TransitionKind::Internal
            && is_compound_or_parallel(nodes, source)
            && targets.iter().all(|t| is_descendant(nodes, *t, source))
        {
            return source;
        }
        let mut anc = nodes[source.0 as usize].parent;
        while let Some(candidate) = anc {
            if is_compound_or_parallel(nodes, candidate) && targets.iter().all(|t| is_descendant_or_self(nodes, *t, candidate)) {
                return candidate;
            }
            anc = nodes[candidate.0 as usize].parent;
        }
        ROOT
    }

    fn set_history(history: &mut Vec<(NodeId, Vec<NodeId>)>, key: NodeId, value: Vec<NodeId>) {
        if let Some(entry) = history.iter_mut().find(|(k, _)| *k == key) {
            entry.1 = value;
        } else {
            history.push((key, value));
        }
    }

    fn resolve_effective_targets(nodes: &[NodeDef], targets: &[NodeId], history: &[(NodeId, Vec<NodeId>)]) -> Vec<NodeId> {
        let mut out = Vec::new();
        for &t in targets {
            match nodes[t.0 as usize].kind {
                NodeKind::HistoryShallow | NodeKind::HistoryDeep => {
                    if let Some((_, recorded)) = history.iter().find(|(id, _)| *id == t) {
                        for &r in recorded {
                            if !out.contains(&r) {
                                out.push(r);
                            }
                        }
                    } else if let Some(default) = nodes[t.0 as usize].initial {
                        if !out.contains(&default) {
                            out.push(default);
                        }
                    }
                }
                _ => {
                    if !out.contains(&t) {
                        out.push(t);
                    }
                }
            }
        }
        out
    }

    fn add_descendant_states_to_enter(nodes: &[NodeDef], state: NodeId, history: &[(NodeId, Vec<NodeId>)], out: &mut Vec<NodeId>) {
        if matches!(nodes[state.0 as usize].kind, NodeKind::HistoryShallow | NodeKind::HistoryDeep) {
            for r in resolve_effective_targets(nodes, core::slice::from_ref(&state), history) {
                add_descendant_states_to_enter(nodes, r, history, out);
            }
            return;
        }
        if !out.contains(&state) {
            out.push(state);
        }
        match nodes[state.0 as usize].kind {
            NodeKind::Compound => {
                if let Some(initial) = nodes[state.0 as usize].initial {
                    add_descendant_states_to_enter(nodes, initial, history, out);
                    add_ancestor_states_to_enter(nodes, initial, state, history, out);
                }
            }
            NodeKind::Parallel => {
                for &child in nodes[state.0 as usize].children {
                    if !out.iter().any(|e| is_descendant_or_self(nodes, *e, child)) {
                        add_descendant_states_to_enter(nodes, child, history, out);
                    }
                }
            }
            _ => {}
        }
    }

    fn add_ancestor_states_to_enter(nodes: &[NodeDef], state: NodeId, stop_at: NodeId, history: &[(NodeId, Vec<NodeId>)], out: &mut Vec<NodeId>) {
        let mut anc = nodes[state.0 as usize].parent;
        while let Some(a) = anc {
            if a == stop_at {
                break;
            }
            if !out.contains(&a) {
                out.push(a);
            }
            if nodes[a.0 as usize].kind == NodeKind::Parallel {
                for &child in nodes[a.0 as usize].children {
                    if !out.iter().any(|e| is_descendant_or_self(nodes, *e, child)) {
                        add_descendant_states_to_enter(nodes, child, history, out);
                    }
                }
            }
            anc = nodes[a.0 as usize].parent;
        }
    }

    fn state_done(nodes: &[NodeDef], config: &impl Configuration, node: NodeId) -> bool {
        match nodes[node.0 as usize].kind {
            NodeKind::Final => true,
            NodeKind::Compound => {
                for &child in nodes[node.0 as usize].children {
                    if config.contains(child) {
                        return state_done(nodes, config, child);
                    }
                }
                false
            }
            NodeKind::Parallel => nodes[node.0 as usize].children.iter().all(|&c| state_done(nodes, config, c)),
            _ => false,
        }
    }

    fn compute_done_nodes<M: Machine>(def: &MachineDefinition<M>, config: &M::Config) -> Vec<NodeId> {
        let mut out = Vec::new();
        for id in config.iter_ones() {
            if is_compound_or_parallel(def.nodes, id) && state_done(def.nodes, config, id) {
                out.push(id);
            }
        }
        out
    }

    //#endregion 🔖Configuration

    //#region 🔖Commands

    /// 🎇 A declarative request the kernel produces but never executes — the [`crate::Host`] does.
    pub enum Command<M: Machine> {
        Effect(M::Effect),
        Raise(M::Event),
        Send { to: ActorId, event: M::Event },
        Emit(M::Output),
        StartInvoke(InvokeId),
        StopInvoke(InvokeId),
        Schedule { timer: TimerId, delay_ms: u64 },
        CancelTimer(TimerId),
    }

    /// 🎇 Where a running machine pushes the [`Command`]s it produces.
    pub trait CommandSink<M: Machine> {
        fn push(&mut self, command: Command<M>);
    }

    impl<M: Machine> CommandSink<M> for Vec<Command<M>> {
        fn push(&mut self, command: Command<M>) {
            Vec::push(self, command);
        }
    }

    //#endregion 🔖Commands

    //#region 🔖Snapshot

    /// 🏳️ Whether a machine is still running, has produced an output, or was stopped by its host.
    #[derive(Debug)]
    pub enum Status<O> {
        Running,
        Done(O),
        Stopped,
    }

    /// 📸 A machine's runtime state: active configuration (never a single enum — see [`Configuration`]),
    /// consumer context, status, and private history slots for `history(...)` targets.
    pub struct Snapshot<M: Machine> {
        pub configuration: M::Config,
        pub context: M::Context,
        pub status: Status<M::Output>,
        pub(crate) history: Vec<(NodeId, Vec<NodeId>)>,
    }

    impl<M: Machine> Snapshot<M> {
        pub(crate) fn from_parts(configuration: M::Config, context: M::Context, status: Status<M::Output>, history: Vec<(NodeId, Vec<NodeId>)>) -> Self {
            Self { configuration, context, status, history }
        }

        pub(crate) fn history_entries(&self) -> &[(NodeId, Vec<NodeId>)] {
            &self.history
        }

        /// 🔎 Whether the state with this stable id is part of the active configuration.
        pub fn matches(&self, stable_id: &str) -> bool {
            let def = M::definition();
            self.configuration.iter_ones().any(|id| def.nodes[id.0 as usize].stable_id == stable_id)
        }
    }

    /// 📋 How many microsteps a [`macrostep`] took before settling.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct StepReport {
        pub microsteps: u32,
    }

    //#endregion 🔖Snapshot

    //#region 🔖Microstep

    /// 🧯 Safety cap against unguarded eventless transition cycles — a malformed machine
    /// hits this instead of looping forever.
    pub const MICROSTEP_LIMIT: u32 = 1000;

    #[derive(Clone, Copy)]
    enum Selector {
        Event(EventId),
        Spontaneous,
        Timer(TimerId),
    }

    struct ActiveTrigger<M: Machine> {
        selector: RaisedSelector,
        event: Option<M::Event>,
    }

    #[derive(Clone, Copy)]
    enum RaisedSelector {
        Event(EventId),
        Timer(TimerId),
    }

    fn candidates_for<M: Machine>(
        def: &MachineDefinition<M>,
        config: &M::Config,
        context: &M::Context,
        event: Option<&M::Event>,
        selector: Selector,
        done: &[NodeId],
    ) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, t) in def.transitions.iter().enumerate() {
            if !config.contains(t.source) {
                continue;
            }
            let matches_trigger = match (selector, t.trigger) {
                (Selector::Event(id), Trigger::Event(tid)) => id == tid,
                (Selector::Spontaneous, Trigger::Eventless) => true,
                (Selector::Spontaneous, Trigger::Done(node)) => done.contains(&node),
                (Selector::Timer(id), Trigger::Timer(tid)) => id == tid,
                _ => false,
            };
            if !matches_trigger {
                continue;
            }
            if let Some(guard_id) = t.guard {
                let guard = def.guards[guard_id.0 as usize];
                if !guard(context, event) {
                    continue;
                }
            }
            out.push(i);
        }
        out
    }

    /// 🥊 Keeps the deepest-source transition when two candidates' exit domains overlap
    /// (child preemption); ties keep document order.
    fn resolve_conflicts(nodes: &[NodeDef], transitions: &[TransitionDef], mut candidates: Vec<usize>) -> Vec<usize> {
        candidates.sort_by_key(|&i| transitions[i].doc_index);
        let mut selected: Vec<usize> = Vec::new();
        'outer: for cand in candidates {
            let cand_domain = compute_domain(nodes, transitions[cand].source, transitions[cand].targets, transitions[cand].kind);
            let mut to_remove = Vec::new();
            for (i, &sel) in selected.iter().enumerate() {
                let sel_domain = compute_domain(nodes, transitions[sel].source, transitions[sel].targets, transitions[sel].kind);
                let overlap = is_descendant_or_self(nodes, cand_domain, sel_domain) || is_descendant_or_self(nodes, sel_domain, cand_domain);
                if overlap {
                    let cand_depth = depth_of(nodes, transitions[cand].source);
                    let sel_depth = depth_of(nodes, transitions[sel].source);
                    if cand_depth > sel_depth {
                        to_remove.push(i);
                    } else {
                        continue 'outer;
                    }
                }
            }
            for i in to_remove.into_iter().rev() {
                selected.remove(i);
            }
            selected.push(cand);
        }
        selected
    }

    fn apply_transitions<M: Machine>(
        def: &MachineDefinition<M>,
        snapshot: &mut Snapshot<M>,
        transitions_idx: &[usize],
        event: Option<&M::Event>,
        sink: &mut impl CommandSink<M>,
        inspector: &mut impl Inspector<M>,
    ) {
        let nodes = def.nodes;

        let mut exit_ids: Vec<NodeId> = Vec::new();
        for &ti in transitions_idx {
            let t = &def.transitions[ti];
            let domain = compute_domain(nodes, t.source, t.targets, t.kind);
            for id in snapshot.configuration.iter_ones() {
                if is_descendant(nodes, id, domain) && !exit_ids.contains(&id) {
                    exit_ids.push(id);
                }
            }
        }
        exit_ids.sort_by(|a, b| depth_of(nodes, *b).cmp(&depth_of(nodes, *a)));

        for &owner in &exit_ids {
            for &child in nodes[owner.0 as usize].children {
                match nodes[child.0 as usize].kind {
                    NodeKind::HistoryShallow => {
                        if let Some(&active_child) = nodes[owner.0 as usize]
                            .children
                            .iter()
                            .find(|c| snapshot.configuration.contains(**c) && !matches!(nodes[c.0 as usize].kind, NodeKind::HistoryShallow | NodeKind::HistoryDeep))
                        {
                            set_history(&mut snapshot.history, child, vec![active_child]);
                        }
                    }
                    NodeKind::HistoryDeep => {
                        let leaves: Vec<NodeId> = snapshot
                            .configuration
                            .iter_ones()
                            .filter(|id| is_descendant(nodes, *id, owner) && is_leafish(nodes, *id))
                            .collect();
                        set_history(&mut snapshot.history, child, leaves);
                    }
                    _ => {}
                }
            }
        }

        for &id in &exit_ids {
            for &action_id in nodes[id.0 as usize].exit_actions {
                (def.actions[action_id.0 as usize])(&mut snapshot.context, event, sink);
            }
            for &(timer_id, _delay) in nodes[id.0 as usize].timers {
                sink.push(Command::CancelTimer(timer_id));
            }
            for &invoke_id in nodes[id.0 as usize].invokes {
                sink.push(Command::StopInvoke(invoke_id));
            }
            snapshot.configuration.clear(id);
        }

        for &ti in transitions_idx {
            let t = &def.transitions[ti];
            for &action_id in t.actions {
                (def.actions[action_id.0 as usize])(&mut snapshot.context, event, sink);
            }
        }

        let mut entry_ids: Vec<NodeId> = Vec::new();
        for &ti in transitions_idx {
            let t = &def.transitions[ti];
            let domain = compute_domain(nodes, t.source, t.targets, t.kind);
            let effective_targets = resolve_effective_targets(nodes, t.targets, &snapshot.history);
            for &target in &effective_targets {
                add_descendant_states_to_enter(nodes, target, &snapshot.history, &mut entry_ids);
            }
            for &target in &effective_targets {
                add_ancestor_states_to_enter(nodes, target, domain, &snapshot.history, &mut entry_ids);
            }
        }
        entry_ids.sort_by_key(|id| depth_of(nodes, *id));

        for &id in &entry_ids {
            snapshot.configuration.set(id);
            for &action_id in nodes[id.0 as usize].entry_actions {
                (def.actions[action_id.0 as usize])(&mut snapshot.context, event, sink);
            }
            for &(timer_id, delay_ms) in nodes[id.0 as usize].timers {
                sink.push(Command::Schedule { timer: timer_id, delay_ms });
            }
            for &invoke_id in nodes[id.0 as usize].invokes {
                sink.push(Command::StartInvoke(invoke_id));
            }
        }

        inspector.observe(InspectionEvent::Microstep { exited: &exit_ids, entered: &entry_ids });
    }

    //#endregion 🔖Microstep

    //#region 🔖Macrostep

    fn finalize_status<M: Machine>(def: &MachineDefinition<M>, snapshot: &mut Snapshot<M>) {
        if matches!(snapshot.status, Status::Done(_)) {
            return;
        }
        if state_done(def.nodes, &snapshot.configuration, ROOT) {
            if let Some(make_output) = def.make_output {
                snapshot.status = Status::Done(make_output(&snapshot.context));
            }
        }
    }

    fn run_to_completion<M: Machine>(
        snapshot: &mut Snapshot<M>,
        seed: Option<ActiveTrigger<M>>,
        sink: &mut impl CommandSink<M>,
        inspector: &mut impl Inspector<M>,
    ) -> StepReport {
        let def = M::definition();
        inspector.observe(InspectionEvent::MacrostepStart);
        let mut queue: VecDeque<ActiveTrigger<M>> = VecDeque::new();
        if let Some(seed) = seed {
            queue.push_back(seed);
        }
        let mut microsteps = 0u32;
        loop {
            if microsteps >= MICROSTEP_LIMIT {
                break;
            }
            let (selected, event_owned) = if let Some(trigger) = queue.pop_front() {
                let done = compute_done_nodes(def, &snapshot.configuration);
                let selector = match trigger.selector {
                    RaisedSelector::Event(id) => Selector::Event(id),
                    RaisedSelector::Timer(id) => Selector::Timer(id),
                };
                let cands = candidates_for(def, &snapshot.configuration, &snapshot.context, trigger.event.as_ref(), selector, &done);
                (cands, trigger.event)
            } else {
                let done = compute_done_nodes(def, &snapshot.configuration);
                let cands = candidates_for(def, &snapshot.configuration, &snapshot.context, None, Selector::Spontaneous, &done);
                if cands.is_empty() {
                    break;
                }
                (cands, None)
            };
            if selected.is_empty() {
                continue;
            }
            let selected = resolve_conflicts(def.nodes, def.transitions, selected);
            microsteps += 1;
            let mut local: Vec<Command<M>> = Vec::new();
            apply_transitions(def, snapshot, &selected, event_owned.as_ref(), &mut local, inspector);
            for command in local {
                if let Command::Raise(ref ev) = command {
                    queue.push_back(ActiveTrigger {
                        selector: RaisedSelector::Event(ev.event_id()),
                        event: Some(ev.clone()),
                    });
                }
                inspector.observe(InspectionEvent::CommandIssued(&command));
                sink.push(command);
            }
        }
        finalize_status(def, snapshot);
        inspector.observe(InspectionEvent::Settled { microsteps });
        StepReport { microsteps }
    }

    /// 🚀 Builds a fresh [`Snapshot`] from `input`, entering the root's default descendant chain
    /// and settling any eventless/done transitions enabled immediately on init.
    pub fn init<M: Machine>(input: M::Input, sink: &mut impl CommandSink<M>) -> Snapshot<M> {
        let def = M::definition();
        let mut snapshot = Snapshot {
            configuration: <M::Config as Default>::default(),
            context: (def.context_from_input)(input),
            status: Status::Running,
            history: Vec::new(),
        };
        let mut entry_ids: Vec<NodeId> = Vec::new();
        add_descendant_states_to_enter(def.nodes, ROOT, &snapshot.history, &mut entry_ids);
        entry_ids.sort_by_key(|id| depth_of(def.nodes, *id));
        for &id in &entry_ids {
            Configuration::set(&mut snapshot.configuration, id);
            for &action_id in def.nodes[id.0 as usize].entry_actions {
                (def.actions[action_id.0 as usize])(&mut snapshot.context, None, sink);
            }
            for &(timer_id, delay_ms) in def.nodes[id.0 as usize].timers {
                sink.push(Command::Schedule { timer: timer_id, delay_ms });
            }
            for &invoke_id in def.nodes[id.0 as usize].invokes {
                sink.push(Command::StartInvoke(invoke_id));
            }
        }
        let mut inspector = crate::inspect::NullInspector;
        run_to_completion(&mut snapshot, None, sink, &mut inspector);
        snapshot
    }

    /// 🏃 Runs one external event to completion (a "macrostep"): the triggered microstep,
    /// then every enabled eventless/`on_done` microstep, until the configuration settles.
    pub fn macrostep<M: Machine>(snapshot: &mut Snapshot<M>, event: M::Event, sink: &mut impl CommandSink<M>, inspector: &mut impl Inspector<M>) -> StepReport {
        let seed = ActiveTrigger {
            selector: RaisedSelector::Event(event.event_id()),
            event: Some(event),
        };
        run_to_completion(snapshot, Some(seed), sink, inspector)
    }

    /// ⏱️ Runs an `after`-timer firing to completion — the runtime's entry point when a
    /// [`crate::Host`] reports a scheduled [`TimerId`] elapsed.
    pub fn timer_elapsed<M: Machine>(snapshot: &mut Snapshot<M>, timer: TimerId, sink: &mut impl CommandSink<M>, inspector: &mut impl Inspector<M>) -> StepReport {
        let seed = ActiveTrigger {
            selector: RaisedSelector::Timer(timer),
            event: None,
        };
        run_to_completion(snapshot, Some(seed), sink, inspector)
    }

    //#endregion 🔖Macrostep

    //#region 🧪Tests

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::inspect::NullInspector;
        use crate::{ActionId, BitSet, EventId, GuardId, NodeId};

        //#region 🔖ToggleMachine

        #[derive(Clone, Debug, PartialEq)]
        enum ToggleEvent {
            Flip,
        }

        impl StatechartEvent for ToggleEvent {
            const EVENT_COUNT: u16 = 1;
            fn event_id(&self) -> EventId {
                EventId(0)
            }
            fn event_name(id: EventId) -> &'static str {
                match id.0 {
                    0 => "Flip",
                    _ => "?",
                }
            }
        }

        #[derive(Clone, Debug, Default, PartialEq)]
        struct ToggleContext {
            count: u32,
            allow: bool,
        }

        fn toggle_inc(ctx: &mut ToggleContext, _event: Option<&ToggleEvent>, _sink: &mut dyn CommandSink<ToggleMachine>) {
            ctx.count += 1;
        }

        fn toggle_allowed(ctx: &ToggleContext, _event: Option<&ToggleEvent>) -> bool {
            ctx.allow
        }

        const TOGGLE_NODES: &[NodeDef] = &[
            NodeDef {
                stable_id: "root",
                kind: NodeKind::Compound,
                parent: None,
                initial: Some(NodeId(1)),
                children: &[NodeId(1), NodeId(2)],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 0,
            },
            NodeDef {
                stable_id: "off",
                kind: NodeKind::Atomic,
                parent: Some(ROOT),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 1,
            },
            NodeDef {
                stable_id: "on",
                kind: NodeKind::Atomic,
                parent: Some(ROOT),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 2,
            },
        ];

        const TOGGLE_TRANSITIONS: &[TransitionDef] = &[
            TransitionDef {
                source: NodeId(1),
                trigger: Trigger::Event(EventId(0)),
                guard: None,
                targets: &[NodeId(2)],
                kind: TransitionKind::External,
                actions: &[ActionId(0)],
                doc_index: 0,
            },
            TransitionDef {
                source: NodeId(2),
                trigger: Trigger::Event(EventId(0)),
                guard: Some(GuardId(0)),
                targets: &[NodeId(1)],
                kind: TransitionKind::External,
                actions: &[ActionId(0)],
                doc_index: 1,
            },
        ];

        struct ToggleMachine;
        impl Machine for ToggleMachine {
            type Context = ToggleContext;
            type Event = ToggleEvent;
            type Input = bool;
            type Output = ();
            type Effect = ();
            type Config = BitSet<1>;
            fn definition() -> &'static MachineDefinition<Self> {
                static DEF: MachineDefinition<ToggleMachine> = MachineDefinition {
                    id: "toggle",
                    nodes: TOGGLE_NODES,
                    transitions: TOGGLE_TRANSITIONS,
                    context_from_input: |allow| ToggleContext { count: 0, allow },
                    make_output: None,
                    guards: &[toggle_allowed],
                    actions: &[toggle_inc],
                    fingerprint: 1,
                    manifest_json: "{}",
                };
                &DEF
            }
        }

        #[test]
        fn flat_machine_toggles_and_counts() {
            let mut sink: Vec<Command<ToggleMachine>> = Vec::new();
            let mut snapshot = init::<ToggleMachine>(true, &mut sink);
            assert!(snapshot.matches("off"));
            let mut inspector = NullInspector;
            macrostep(&mut snapshot, ToggleEvent::Flip, &mut sink, &mut inspector);
            assert!(snapshot.matches("on"));
            assert_eq!(snapshot.context.count, 1);
            macrostep(&mut snapshot, ToggleEvent::Flip, &mut sink, &mut inspector);
            assert!(snapshot.matches("off"));
            assert_eq!(snapshot.context.count, 2);
        }

        #[test]
        fn guard_blocks_transition_when_false() {
            let mut sink: Vec<Command<ToggleMachine>> = Vec::new();
            let mut snapshot = init::<ToggleMachine>(false, &mut sink);
            let mut inspector = NullInspector;
            macrostep(&mut snapshot, ToggleEvent::Flip, &mut sink, &mut inspector);
            assert!(snapshot.matches("on"));
            // guard on the On->Off transition requires ctx.allow, which is false.
            macrostep(&mut snapshot, ToggleEvent::Flip, &mut sink, &mut inspector);
            assert!(snapshot.matches("on"), "guard should have blocked the transition back to off");
            assert_eq!(snapshot.context.count, 1);
        }

        //#endregion 🔖ToggleMachine

        //#region 🔖PlayerMachine

        #[derive(Clone, Debug, PartialEq)]
        enum PlayerEvent {
            Open,
            Pause,
            Play,
            Stop,
            Resume,
        }

        impl StatechartEvent for PlayerEvent {
            const EVENT_COUNT: u16 = 5;
            fn event_id(&self) -> EventId {
                match self {
                    PlayerEvent::Open => EventId(0),
                    PlayerEvent::Pause => EventId(1),
                    PlayerEvent::Play => EventId(2),
                    PlayerEvent::Stop => EventId(3),
                    PlayerEvent::Resume => EventId(4),
                }
            }
            fn event_name(id: EventId) -> &'static str {
                match id.0 {
                    0 => "Open",
                    1 => "Pause",
                    2 => "Play",
                    3 => "Stop",
                    4 => "Resume",
                    _ => "?",
                }
            }
        }

        #[derive(Clone, Debug, Default)]
        struct PlayerContext;

        const PLAYER_NODES: &[NodeDef] = &[
            NodeDef {
                stable_id: "root",
                kind: NodeKind::Compound,
                parent: None,
                initial: Some(NodeId(1)),
                children: &[NodeId(1), NodeId(3)],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 0,
            },
            NodeDef {
                stable_id: "closed",
                kind: NodeKind::Atomic,
                parent: Some(ROOT),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 1,
            },
            NodeDef {
                stable_id: "playing",
                kind: NodeKind::Atomic,
                parent: Some(NodeId(3)),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 3,
            },
            NodeDef {
                stable_id: "open",
                kind: NodeKind::Compound,
                parent: Some(ROOT),
                initial: Some(NodeId(2)),
                children: &[NodeId(2), NodeId(4), NodeId(5)],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 2,
            },
            NodeDef {
                stable_id: "paused",
                kind: NodeKind::Atomic,
                parent: Some(NodeId(3)),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 4,
            },
            NodeDef {
                stable_id: "open.history",
                kind: NodeKind::HistoryShallow,
                parent: Some(NodeId(3)),
                initial: Some(NodeId(2)),
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 5,
            },
        ];

        const PLAYER_TRANSITIONS: &[TransitionDef] = &[
            TransitionDef {
                source: NodeId(1),
                trigger: Trigger::Event(EventId(0)),
                guard: None,
                targets: &[NodeId(3)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 0,
            },
            TransitionDef {
                source: NodeId(2),
                trigger: Trigger::Event(EventId(1)),
                guard: None,
                targets: &[NodeId(4)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 1,
            },
            TransitionDef {
                source: NodeId(4),
                trigger: Trigger::Event(EventId(2)),
                guard: None,
                targets: &[NodeId(2)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 2,
            },
            TransitionDef {
                source: NodeId(3),
                trigger: Trigger::Event(EventId(3)),
                guard: None,
                targets: &[NodeId(1)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 3,
            },
            TransitionDef {
                source: NodeId(1),
                trigger: Trigger::Event(EventId(4)),
                guard: None,
                targets: &[NodeId(5)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 4,
            },
        ];

        struct PlayerMachine;
        impl Machine for PlayerMachine {
            type Context = PlayerContext;
            type Event = PlayerEvent;
            type Input = ();
            type Output = ();
            type Effect = ();
            type Config = BitSet<1>;
            fn definition() -> &'static MachineDefinition<Self> {
                static DEF: MachineDefinition<PlayerMachine> = MachineDefinition {
                    id: "player",
                    nodes: PLAYER_NODES,
                    transitions: PLAYER_TRANSITIONS,
                    context_from_input: |_| PlayerContext,
                    make_output: None,
                    guards: &[],
                    actions: &[],
                    fingerprint: 2,
                    manifest_json: "{}",
                };
                &DEF
            }
        }

        #[test]
        fn hierarchical_machine_enters_default_descendant() {
            let mut sink: Vec<Command<PlayerMachine>> = Vec::new();
            let snapshot = init::<PlayerMachine>((), &mut sink);
            assert!(snapshot.matches("closed"));
            assert!(!snapshot.matches("open"));
        }

        #[test]
        fn hierarchical_machine_transitions_into_compound_default() {
            let mut sink: Vec<Command<PlayerMachine>> = Vec::new();
            let mut snapshot = init::<PlayerMachine>((), &mut sink);
            let mut inspector = NullInspector;
            macrostep(&mut snapshot, PlayerEvent::Open, &mut sink, &mut inspector);
            assert!(snapshot.matches("open"));
            assert!(snapshot.matches("playing"));
        }

        #[test]
        fn shallow_history_restores_last_active_child() {
            let mut sink: Vec<Command<PlayerMachine>> = Vec::new();
            let mut snapshot = init::<PlayerMachine>((), &mut sink);
            let mut inspector = NullInspector;
            macrostep(&mut snapshot, PlayerEvent::Open, &mut sink, &mut inspector);
            macrostep(&mut snapshot, PlayerEvent::Pause, &mut sink, &mut inspector);
            assert!(snapshot.matches("paused"));
            macrostep(&mut snapshot, PlayerEvent::Stop, &mut sink, &mut inspector);
            assert!(snapshot.matches("closed"));
            assert!(!snapshot.matches("open"));
            macrostep(&mut snapshot, PlayerEvent::Resume, &mut sink, &mut inspector);
            assert!(snapshot.matches("open"));
            assert!(snapshot.matches("paused"), "shallow history should restore Paused, not the default Playing");
            assert!(!snapshot.matches("playing"));
            macrostep(&mut snapshot, PlayerEvent::Play, &mut sink, &mut inspector);
            assert!(snapshot.matches("playing"));
            assert!(!snapshot.matches("paused"));
        }

        //#endregion 🔖PlayerMachine

        //#region 🔖RecorderMachine

        #[derive(Clone, Debug, PartialEq)]
        enum RecorderEvent {
            Start,
            AudioStop,
            VideoStop,
        }

        impl StatechartEvent for RecorderEvent {
            const EVENT_COUNT: u16 = 3;
            fn event_id(&self) -> EventId {
                match self {
                    RecorderEvent::Start => EventId(0),
                    RecorderEvent::AudioStop => EventId(1),
                    RecorderEvent::VideoStop => EventId(2),
                }
            }
            fn event_name(id: EventId) -> &'static str {
                match id.0 {
                    0 => "Start",
                    1 => "AudioStop",
                    2 => "VideoStop",
                    _ => "?",
                }
            }
        }

        #[derive(Clone, Debug, Default)]
        struct RecorderContext;

        const RECORDER_NODES: &[NodeDef] = &[
            NodeDef {
                stable_id: "root",
                kind: NodeKind::Compound,
                parent: None,
                initial: Some(NodeId(1)),
                children: &[NodeId(1), NodeId(2)],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 0,
            },
            NodeDef {
                stable_id: "idle",
                kind: NodeKind::Atomic,
                parent: Some(ROOT),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 1,
            },
            NodeDef {
                stable_id: "recording",
                kind: NodeKind::Parallel,
                parent: Some(ROOT),
                initial: None,
                children: &[NodeId(3), NodeId(6)],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 2,
            },
            NodeDef {
                stable_id: "audio",
                kind: NodeKind::Compound,
                parent: Some(NodeId(2)),
                initial: Some(NodeId(4)),
                children: &[NodeId(4), NodeId(5)],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 3,
            },
            NodeDef {
                stable_id: "audio.capturing",
                kind: NodeKind::Atomic,
                parent: Some(NodeId(3)),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 4,
            },
            NodeDef {
                stable_id: "audio.done",
                kind: NodeKind::Final,
                parent: Some(NodeId(3)),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 5,
            },
            NodeDef {
                stable_id: "video",
                kind: NodeKind::Compound,
                parent: Some(NodeId(2)),
                initial: Some(NodeId(7)),
                children: &[NodeId(7), NodeId(8)],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 6,
            },
            NodeDef {
                stable_id: "video.capturing",
                kind: NodeKind::Atomic,
                parent: Some(NodeId(6)),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 7,
            },
            NodeDef {
                stable_id: "video.done",
                kind: NodeKind::Final,
                parent: Some(NodeId(6)),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 8,
            },
        ];

        const RECORDER_TRANSITIONS: &[TransitionDef] = &[
            TransitionDef {
                source: NodeId(1),
                trigger: Trigger::Event(EventId(0)),
                guard: None,
                targets: &[NodeId(2)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 0,
            },
            TransitionDef {
                source: NodeId(4),
                trigger: Trigger::Event(EventId(1)),
                guard: None,
                targets: &[NodeId(5)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 1,
            },
            TransitionDef {
                source: NodeId(7),
                trigger: Trigger::Event(EventId(2)),
                guard: None,
                targets: &[NodeId(8)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 2,
            },
            TransitionDef {
                source: NodeId(2),
                trigger: Trigger::Done(NodeId(2)),
                guard: None,
                targets: &[NodeId(1)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 3,
            },
        ];

        struct RecorderMachine;
        impl Machine for RecorderMachine {
            type Context = RecorderContext;
            type Event = RecorderEvent;
            type Input = ();
            type Output = ();
            type Effect = ();
            type Config = BitSet<1>;
            fn definition() -> &'static MachineDefinition<Self> {
                static DEF: MachineDefinition<RecorderMachine> = MachineDefinition {
                    id: "recorder",
                    nodes: RECORDER_NODES,
                    transitions: RECORDER_TRANSITIONS,
                    context_from_input: |_| RecorderContext,
                    make_output: None,
                    guards: &[],
                    actions: &[],
                    fingerprint: 3,
                    manifest_json: "{}",
                };
                &DEF
            }
        }

        #[test]
        fn parallel_regions_enter_together() {
            let mut sink: Vec<Command<RecorderMachine>> = Vec::new();
            let mut snapshot = init::<RecorderMachine>((), &mut sink);
            let mut inspector = NullInspector;
            macrostep(&mut snapshot, RecorderEvent::Start, &mut sink, &mut inspector);
            assert!(snapshot.matches("recording"));
            assert!(snapshot.matches("audio.capturing"));
            assert!(snapshot.matches("video.capturing"));
        }

        #[test]
        fn parallel_done_bubbles_only_once_every_region_finishes() {
            let mut sink: Vec<Command<RecorderMachine>> = Vec::new();
            let mut snapshot = init::<RecorderMachine>((), &mut sink);
            let mut inspector = NullInspector;
            macrostep(&mut snapshot, RecorderEvent::Start, &mut sink, &mut inspector);
            macrostep(&mut snapshot, RecorderEvent::AudioStop, &mut sink, &mut inspector);
            assert!(snapshot.matches("audio.done"));
            assert!(snapshot.matches("recording"), "video region still capturing — on_done must not fire yet");
            macrostep(&mut snapshot, RecorderEvent::VideoStop, &mut sink, &mut inspector);
            assert!(snapshot.matches("idle"), "on_done should bubble once both regions reach final");
            assert!(!snapshot.matches("recording"));
        }

        //#endregion 🔖RecorderMachine
    }

    //#endregion 🧪Tests
}
mod persist {
    //! 💾 Versioned persistence — stable string state ids, fingerprint-checked restore.
    //!
    //! Never serializes JS promises, futures, callbacks, timer handles, or actor
    //! references — only the logical configuration, history and context survive.

    use crate::kernel::{Snapshot, Status};
    use crate::{Configuration, Machine, NodeId};

    //#region 🔖Persist

    /// 💾 A machine's logical state, addressed by stable string ids so adding/renumbering
    /// states never invalidates a previously persisted snapshot.
    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct PersistedSnapshot {
        pub version: u32,
        pub fingerprint: u64,
        pub states: Vec<String>,
        pub history: Vec<(String, Vec<String>)>,
        pub done: bool,
    }

    /// 💾 Why [`restore`] could not rebuild a [`Snapshot`].
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum RestoreError {
        /// No [`Migration`] bridges the persisted fingerprint to the current machine.
        FingerprintMismatch,
        /// A persisted stable id no longer exists in the current machine.
        UnknownStableId(String),
    }

    /// 💾 Migrates a [`PersistedSnapshot`] captured under an older machine fingerprint.
    pub trait Migration {
        /// The fingerprint this migration accepts as input.
        fn from_version(&self) -> u64;
        /// Produces a [`PersistedSnapshot`] valid under a newer fingerprint.
        fn migrate(&self, snapshot: PersistedSnapshot) -> PersistedSnapshot;
    }

    /// 💾 Captures a running [`Snapshot`] as a portable, stable-id-addressed value.
    pub fn persist<M: Machine>(snapshot: &Snapshot<M>) -> PersistedSnapshot {
        let def = M::definition();
        let states = snapshot.configuration.iter_ones().map(|id| def.nodes[id.0 as usize].stable_id.to_string()).collect();
        let history = snapshot
            .history_entries()
            .iter()
            .map(|(owner, ids)| {
                (
                    def.nodes[owner.0 as usize].stable_id.to_string(),
                    ids.iter().map(|id| def.nodes[id.0 as usize].stable_id.to_string()).collect(),
                )
            })
            .collect();
        PersistedSnapshot {
            version: 1,
            fingerprint: def.fingerprint,
            states,
            history,
            done: matches!(snapshot.status, Status::Done(_)),
        }
    }

    fn stable_id_to_node(def_nodes: &[crate::kernel::NodeDef], stable_id: &str) -> Result<NodeId, RestoreError> {
        def_nodes
            .iter()
            .position(|n| n.stable_id == stable_id)
            .map(|idx| NodeId(idx as u16))
            .ok_or_else(|| RestoreError::UnknownStableId(stable_id.to_string()))
    }

    /// 💾 Rebuilds a [`Snapshot`] from a [`PersistedSnapshot`], applying `migrations` in
    /// sequence until the fingerprint matches the current machine, then re-resolving
    /// stable ids back to dense [`NodeId`]s. `context` is supplied by the caller since
    /// the consumer's `Context` may itself need domain-specific deserialization.
    pub fn restore<M: Machine>(persisted: &PersistedSnapshot, context: M::Context, migrations: &[&dyn Migration]) -> Result<Snapshot<M>, RestoreError> {
        let def = M::definition();
        let mut current = persisted.clone();
        while current.fingerprint != def.fingerprint {
            let next = migrations.iter().find(|m| m.from_version() == current.fingerprint);
            match next {
                Some(m) => current = m.migrate(current),
                None => return Err(RestoreError::FingerprintMismatch),
            }
        }
        let mut configuration = M::Config::default();
        for stable_id in &current.states {
            configuration.set(stable_id_to_node(def.nodes, stable_id)?);
        }
        let mut history = Vec::new();
        for (owner_id, ids) in &current.history {
            let owner = stable_id_to_node(def.nodes, owner_id)?;
            let mut resolved = Vec::new();
            for id in ids {
                resolved.push(stable_id_to_node(def.nodes, id)?);
            }
            history.push((owner, resolved));
        }
        let status = Status::Running;
        Ok(Snapshot::from_parts(configuration, context, status, history))
    }

    //#endregion 🔖Persist

    //#region 🧪Tests

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::kernel::{init, macrostep};
        use crate::testing::support::{unit_toggle_definition, UnitToggleContext, UnitToggleEvent, UnitToggleMachine};

        #[test]
        fn persist_then_restore_round_trips_active_state() {
            let _ = unit_toggle_definition();
            let mut sink = Vec::new();
            let mut snapshot = init::<UnitToggleMachine>((), &mut sink);
            let mut inspector = crate::inspect::NullInspector;
            macrostep(&mut snapshot, UnitToggleEvent::Flip, &mut sink, &mut inspector);
            assert!(snapshot.matches("on"));

            let persisted = persist(&snapshot);
            assert_eq!(persisted.fingerprint, UnitToggleMachine::definition().fingerprint);
            assert!(persisted.states.iter().any(|s| s == "on"));

            let restored = restore::<UnitToggleMachine>(&persisted, UnitToggleContext::default(), &[]).expect("restore should succeed");
            assert!(restored.matches("on"));
        }

        #[test]
        fn restore_rejects_fingerprint_mismatch_without_migration() {
            let mut sink = Vec::new();
            let snapshot = init::<UnitToggleMachine>((), &mut sink);
            let mut persisted = persist(&snapshot);
            persisted.fingerprint = 9999;
            let result = restore::<UnitToggleMachine>(&persisted, UnitToggleContext::default(), &[]);
            assert!(matches!(result, Err(RestoreError::FingerprintMismatch)));
        }

        struct BumpFingerprint;
        impl Migration for BumpFingerprint {
            fn from_version(&self) -> u64 {
                9999
            }
            fn migrate(&self, mut snapshot: PersistedSnapshot) -> PersistedSnapshot {
                snapshot.fingerprint = UnitToggleMachine::definition().fingerprint;
                snapshot
            }
        }

        #[test]
        fn restore_applies_migration_chain_until_fingerprint_matches() {
            let mut sink = Vec::new();
            let snapshot = init::<UnitToggleMachine>((), &mut sink);
            let mut persisted = persist(&snapshot);
            persisted.fingerprint = 9999;
            let migration = BumpFingerprint;
            let migrations: &[&dyn Migration] = &[&migration];
            let restored = restore::<UnitToggleMachine>(&persisted, UnitToggleContext::default(), migrations).expect("migration should bridge fingerprint");
            assert!(restored.matches("off"));
        }
    }

    //#endregion 🧪Tests
}
mod runtime {
    //! 🎬 Actor runtime — mailboxes, spawn, and command routing atop the pure kernel.
    //!
    //! Every actor processes its mailbox serially; nothing here mutates a snapshot
    //! concurrently, even on multithreaded native targets.

    use crate::host::Host;
    use crate::kernel::{init, macrostep, Command, Status};
    use crate::{ActorId, Machine, NullInspector, Snapshot, StepReport};
    use std::collections::VecDeque;

    //#region 🔖ActorLogic

    /// 🎭 The shape of runnable actor logic — implemented for any [`Machine`] via [`MachineLogic`].
    pub trait ActorLogic {
        type Event;
        type Input;
        type Output;
        type Snapshot;
    }

    /// 🎭 Blanket [`ActorLogic`] for any compiled [`Machine`].
    pub struct MachineLogic<M: Machine>(core::marker::PhantomData<M>);

    impl<M: Machine> ActorLogic for MachineLogic<M> {
        type Event = M::Event;
        type Input = M::Input;
        type Output = M::Output;
        type Snapshot = Snapshot<M>;
    }

    //#endregion 🔖ActorLogic

    //#region 🔖Actor

    /// 🎬 One running machine instance: its snapshot plus a serial mailbox.
    struct Actor<M: Machine> {
        id: ActorId,
        snapshot: Snapshot<M>,
        mailbox: VecDeque<M::Event>,
    }

    //#endregion 🔖Actor

    //#region 🔖System

    /// 🎬 Owns every spawned [`Actor`] for one machine type and routes their [`Command`]s
    /// to a [`Host`]. Mailboxes drain in round-robin order until quiescent.
    pub struct ActorSystem<M: Machine, H: Host<M>> {
        pub host: H,
        actors: Vec<Actor<M>>,
        next_id: u32,
    }

    impl<M: Machine, H: Host<M>> ActorSystem<M, H> {
        /// 🎬 A fresh system with no actors yet, owning `host`.
        pub fn new(host: H) -> Self {
            Self { host, actors: Vec::new(), next_id: 0 }
        }

        /// 🎬 Initializes and registers a root actor, routing its initial commands immediately.
        pub fn spawn_root(&mut self, input: M::Input) -> ActorId {
            let id = ActorId(self.next_id);
            self.next_id += 1;
            let mut buffer: Vec<Command<M>> = Vec::new();
            let snapshot = init::<M>(input, &mut buffer);
            self.actors.push(Actor { id, snapshot, mailbox: VecDeque::new() });
            self.route_commands(id, buffer);
            id
        }

        /// 🎬 The current [`Snapshot`] of an actor, if it exists.
        pub fn snapshot(&self, id: ActorId) -> Option<&Snapshot<M>> {
            self.actors.iter().find(|a| a.id == id).map(|a| &a.snapshot)
        }

        /// 🎬 Enqueues an event for delivery on the next [`ActorSystem::drain`].
        pub fn send(&mut self, to: ActorId, event: M::Event) {
            if let Some(actor) = self.actors.iter_mut().find(|a| a.id == to) {
                actor.mailbox.push_back(event);
            }
        }

        /// 🎬 Delivers a [`TimerId`](crate::TimerId) elapsed notification straight to `macrostep`'s
        /// timer entry point for `to`.
        pub fn timer_elapsed(&mut self, to: ActorId, timer: crate::TimerId) -> Option<StepReport> {
            let idx = self.actors.iter().position(|a| a.id == to)?;
            let mut buffer: Vec<Command<M>> = Vec::new();
            let mut inspector = NullInspector;
            let report = crate::kernel::timer_elapsed(&mut self.actors[idx].snapshot, timer, &mut buffer, &mut inspector);
            self.route_commands(to, buffer);
            Some(report)
        }

        /// 🎬 Drains every actor's mailbox to quiescence, running one macrostep per delivered event.
        pub fn drain(&mut self) -> Vec<StepReport> {
            let mut reports = Vec::new();
            loop {
                let mut progressed = false;
                for idx in 0..self.actors.len() {
                    let Some(event) = self.actors[idx].mailbox.pop_front() else {
                        continue;
                    };
                    progressed = true;
                    let id = self.actors[idx].id;
                    let mut buffer: Vec<Command<M>> = Vec::new();
                    let mut inspector = NullInspector;
                    let report = macrostep(&mut self.actors[idx].snapshot, event, &mut buffer, &mut inspector);
                    self.route_commands(id, buffer);
                    reports.push(report);
                }
                if !progressed {
                    break;
                }
            }
            reports
        }

        fn route_commands(&mut self, actor: ActorId, commands: Vec<Command<M>>) {
            let mut sends = Vec::new();
            if let Some(idx) = self.actors.iter().position(|a| a.id == actor) {
                for command in commands {
                    if let Some(pair) = route_command(&mut self.host, &mut self.actors[idx].snapshot, actor, command) {
                        sends.push(pair);
                    }
                }
            }
            for (to, event) in sends {
                self.send(to, event);
            }
        }
    }

    /// 🎬 Applies one [`Command`] to `host`/`snapshot`; returns a `Send` command's
    /// `(to, event)` pair for the caller to route on, since a lone [`Host`]+[`Snapshot`]
    /// pair (e.g. a single `export_wasm_machine!` instance) has no other actor to deliver it to.
    pub fn route_command<M: Machine>(host: &mut impl Host<M>, snapshot: &mut Snapshot<M>, actor: ActorId, command: Command<M>) -> Option<(ActorId, M::Event)> {
        match command {
            Command::Effect(effect) => {
                host.execute_effect(actor, effect);
                None
            }
            Command::Raise(_) => {
                // The kernel's run-to-completion loop already re-processed this
                // internally; forwarded here only for host-side observability.
                None
            }
            Command::Send { to, event } => Some((to, event)),
            Command::Emit(output) => {
                snapshot.status = Status::Done(output);
                None
            }
            Command::StartInvoke(invoke) => {
                host.start_task(actor, invoke);
                None
            }
            Command::StopInvoke(invoke) => {
                host.cancel_task(actor, invoke);
                None
            }
            Command::Schedule { timer, delay_ms } => {
                host.schedule(actor, timer, delay_ms);
                None
            }
            Command::CancelTimer(timer) => {
                host.cancel_timer(actor, timer);
                None
            }
        }
    }

    //#endregion 🔖System

    //#region 🧪Tests

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::host::TestHost;
        use crate::testing::support::{UnitToggleContext, UnitToggleEvent, UnitToggleMachine};

        #[test]
        fn actor_system_drains_sent_events_through_one_macrostep_each() {
            let mut system: ActorSystem<UnitToggleMachine, TestHost<UnitToggleMachine>> = ActorSystem::new(TestHost::new());
            let root = system.spawn_root(());
            assert!(system.snapshot(root).unwrap().matches("off"));

            system.send(root, UnitToggleEvent::Flip);
            let reports = system.drain();
            assert_eq!(reports.len(), 1);
            assert!(system.snapshot(root).unwrap().matches("on"));

            system.send(root, UnitToggleEvent::Flip);
            system.drain();
            assert!(system.snapshot(root).unwrap().matches("off"));
            assert_eq!(system.snapshot(root).unwrap().context, UnitToggleContext::default());
        }
    }

    //#endregion 🧪Tests
}
#[cfg(any(test, feature = "testing"))]
mod testing {
    //! 🧭 Model-based testing — reachability exploration, invariants, and inline conformance fixtures.
    //!
    //! Fixtures are plain Rust structs/consts rather than a separate JSON format or
    //! separate test files, per this workspace's "extend existing test files" rule.

    use crate::kernel::{init, macrostep, Command, Status};
    use crate::{Configuration, Machine, NullInspector, Snapshot};

    //#region 🔖Model

    /// 🧭 A set of representative events tried from every reachable configuration.
    pub struct Model<M: Machine> {
        events: Vec<M::Event>,
    }

    impl<M: Machine> Model<M> {
        /// 🧭 A model that explores with exactly these representative events.
        pub fn new(events: Vec<M::Event>) -> Self {
            Self { events }
        }
    }

    //#endregion 🔖Model

    //#region 🔖Paths

    fn active_stable_ids<M: Machine>(snapshot: &Snapshot<M>) -> Vec<&'static str> {
        let def = M::definition();
        snapshot.configuration.iter_ones().map(|id| def.nodes[id.0 as usize].stable_id).collect()
    }

    //#endregion 🔖Paths

    //#region 🔖Coverage

    /// 🧭 What a BFS [`explore`] found: distinct configurations visited and every stable
    /// state id reached across them.
    #[derive(Debug, Default)]
    pub struct Coverage {
        pub visited_configurations: usize,
        pub reached_stable_ids: Vec<&'static str>,
    }

    /// 🧭 Breadth-first walk over reachable configurations, trying every event in
    /// `model` from each newly-discovered configuration. Approximates reachability by
    /// configuration only — guard outcomes that depend on context may under-approximate.
    pub fn explore<M: Machine>(model: &Model<M>, input: M::Input) -> Coverage
    where
        M::Context: Clone,
    {
        let mut sink: Vec<Command<M>> = Vec::new();
        let root = init::<M>(input, &mut sink);
        let mut visited: Vec<M::Config> = Vec::new();
        let mut frontier: Vec<Snapshot<M>> = vec![root];
        let mut reached_ids: Vec<&'static str> = Vec::new();

        while let Some(snapshot) = frontier.pop() {
            if visited.iter().any(|c| *c == snapshot.configuration) {
                continue;
            }
            for stable in active_stable_ids(&snapshot) {
                if !reached_ids.contains(&stable) {
                    reached_ids.push(stable);
                }
            }
            visited.push(snapshot.configuration.clone());

            for event in &model.events {
                let mut next = Snapshot::from_parts(snapshot.configuration.clone(), snapshot.context.clone(), Status::Running, snapshot.history_entries().to_vec());
                let mut local_sink: Vec<Command<M>> = Vec::new();
                let mut inspector = NullInspector;
                macrostep(&mut next, event.clone(), &mut local_sink, &mut inspector);
                frontier.push(next);
            }
        }

        Coverage {
            visited_configurations: visited.len(),
            reached_stable_ids: reached_ids,
        }
    }

    //#endregion 🔖Coverage

    //#region ⚠️ Errors

    /// ⚠️ Why an [`Invariant`] check or [`run_conformance`] fixture failed.
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    pub enum FsmError {
        /// 🧭 A model-checked invariant or conformance step reported a violation.
        #[error("{0}")]
        Violation(String),
    }

    //#endregion ⚠️ Errors

    //#region 🔖Invariants

    /// 🧭 A named property that must hold of every [`Snapshot`] visited during exploration.
    pub struct Invariant<M: Machine> {
        pub name: &'static str,
        pub check: fn(&Snapshot<M>) -> Result<(), FsmError>,
    }

    /// 🧭 Runs every invariant against `snapshot`, returning one formatted message per violation.
    pub fn check_invariants<M: Machine>(snapshot: &Snapshot<M>, invariants: &[Invariant<M>]) -> Vec<String> {
        invariants.iter().filter_map(|inv| (inv.check)(snapshot).err().map(|reason| format!("{}: {}", inv.name, reason))).collect()
    }

    //#endregion 🔖Invariants

    //#region 🔖Conformance

    /// 🧭 One step of an inline conformance fixture: send `event`, then assert every
    /// stable id in `expect_active` is part of the settled configuration.
    pub struct ConformanceStep<M: Machine> {
        pub event: M::Event,
        pub expect_active: &'static [&'static str],
    }

    /// 🧭 Runs `steps` against a freshly-initialized machine, failing fast with a
    /// descriptive message naming the offending step and the actual active configuration.
    pub fn run_conformance<M: Machine>(input: M::Input, steps: &[ConformanceStep<M>]) -> Result<(), FsmError> {
        let mut sink: Vec<Command<M>> = Vec::new();
        let mut snapshot = init::<M>(input, &mut sink);
        for (index, step) in steps.iter().enumerate() {
            let mut inspector = NullInspector;
            macrostep(&mut snapshot, step.event.clone(), &mut sink, &mut inspector);
            for expected in step.expect_active {
                if !snapshot.matches(expected) {
                    return Err(FsmError::Violation(format!(
                        "conformance step {index}: expected active state '{expected}', got {:?}",
                        active_stable_ids(&snapshot)
                    )));
                }
            }
        }
        Ok(())
    }

    //#endregion 🔖Conformance

    //#region 🔖Support

    #[cfg(test)]
    pub(crate) mod support {
        use crate::kernel::{MachineDefinition, NodeDef, NodeKind, TransitionDef, TransitionKind, Trigger, ROOT};
        use crate::{BitSet, EventId, Machine, StatechartEvent};

        #[derive(Clone, Debug, PartialEq)]
        pub struct UnitEvent;

        impl StatechartEvent for UnitEvent {
            const EVENT_COUNT: u16 = 1;
            fn event_id(&self) -> EventId {
                EventId(0)
            }
            fn event_name(_id: EventId) -> &'static str {
                "Unit"
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub enum UnitToggleEvent {
            Flip,
        }

        impl StatechartEvent for UnitToggleEvent {
            const EVENT_COUNT: u16 = 1;
            fn event_id(&self) -> EventId {
                EventId(0)
            }
            fn event_name(_id: EventId) -> &'static str {
                "Flip"
            }
        }

        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct UnitToggleContext {
            pub count: u32,
        }

        const NODES: &[NodeDef] = &[
            NodeDef {
                stable_id: "root",
                kind: NodeKind::Compound,
                parent: None,
                initial: Some(crate::NodeId(1)),
                children: &[crate::NodeId(1), crate::NodeId(2)],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 0,
            },
            NodeDef {
                stable_id: "off",
                kind: NodeKind::Atomic,
                parent: Some(ROOT),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 1,
            },
            NodeDef {
                stable_id: "on",
                kind: NodeKind::Atomic,
                parent: Some(ROOT),
                initial: None,
                children: &[],
                entry_actions: &[],
                exit_actions: &[],
                invokes: &[],
                timers: &[],
                doc_index: 2,
            },
        ];

        const TRANSITIONS: &[TransitionDef] = &[
            TransitionDef {
                source: crate::NodeId(1),
                trigger: Trigger::Event(EventId(0)),
                guard: None,
                targets: &[crate::NodeId(2)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 0,
            },
            TransitionDef {
                source: crate::NodeId(2),
                trigger: Trigger::Event(EventId(0)),
                guard: None,
                targets: &[crate::NodeId(1)],
                kind: TransitionKind::External,
                actions: &[],
                doc_index: 1,
            },
        ];

        pub struct UnitToggleMachine;

        impl Machine for UnitToggleMachine {
            type Context = UnitToggleContext;
            type Event = UnitToggleEvent;
            type Input = ();
            type Output = ();
            type Effect = ();
            type Config = BitSet<1>;
            fn definition() -> &'static MachineDefinition<Self> {
                static DEF: MachineDefinition<UnitToggleMachine> = MachineDefinition {
                    id: "unit_toggle",
                    nodes: NODES,
                    transitions: TRANSITIONS,
                    context_from_input: |_| UnitToggleContext::default(),
                    make_output: None,
                    guards: &[],
                    actions: &[],
                    fingerprint: 42,
                    manifest_json: "{}",
                };
                &DEF
            }
        }

        pub fn unit_toggle_definition() -> &'static MachineDefinition<UnitToggleMachine> {
            UnitToggleMachine::definition()
        }
    }

    //#endregion 🔖Support

    //#region 🧪Tests

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::testing::support::{UnitToggleEvent, UnitToggleMachine};

        #[test]
        fn explore_reaches_both_toggle_states() {
            let model = Model::<UnitToggleMachine>::new(vec![UnitToggleEvent::Flip]);
            let coverage = explore(&model, ());
            assert!(coverage.reached_stable_ids.contains(&"off"));
            assert!(coverage.reached_stable_ids.contains(&"on"));
            assert_eq!(coverage.visited_configurations, 2);
        }

        #[test]
        fn conformance_fixture_passes_for_matching_sequence() {
            let steps = [
                ConformanceStep {
                    event: UnitToggleEvent::Flip,
                    expect_active: &["on"],
                },
                ConformanceStep {
                    event: UnitToggleEvent::Flip,
                    expect_active: &["off"],
                },
            ];
            assert!(run_conformance::<UnitToggleMachine>((), &steps).is_ok());
        }

        #[test]
        fn conformance_fixture_fails_with_descriptive_message() {
            let steps = [ConformanceStep {
                event: UnitToggleEvent::Flip,
                expect_active: &["off"],
            }];
            let err = run_conformance::<UnitToggleMachine>((), &steps).unwrap_err().to_string();
            assert!(err.contains("step 0"));
            assert!(err.contains("off"));
        }

        #[test]
        fn invariant_reports_violation_by_name() {
            let mut sink: Vec<Command<UnitToggleMachine>> = Vec::new();
            let snapshot = init::<UnitToggleMachine>((), &mut sink);
            let invariants = [Invariant {
                name: "never off",
                check: |s: &Snapshot<UnitToggleMachine>| if s.matches("off") { Err(FsmError::Violation("was off".to_string())) } else { Ok(()) },
            }];
            let violations = check_invariants(&snapshot, &invariants);
            assert_eq!(violations, vec!["never off: was off".to_string()]);
        }
    }

    //#endregion 🧪Tests
}

//#region 🔖Ids

/// 🔢 Dense index of a state node within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u16);

/// 🔢 Dense index of an event kind within a [`StatechartEvent`] enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(pub u16);

/// 🔢 Dense index of a transition within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransitionId(pub u16);

/// 🔢 Dense index of a guard function within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuardId(pub u16);

/// 🔢 Dense index of a reducer/emitter action within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionId(pub u16);

/// 🔢 Dense index of an `invoke` declaration within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvokeId(pub u16);

/// 🔢 Dense index of an `after` (delayed transition) timer within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerId(pub u16);

/// 🔢 Runtime-assigned identity of a spawned actor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorId(pub u32);

/// 🧮 Hand-rolled fixed-size bitset over `W` `u64` words — one bit per [`NodeId`].
///
/// The `statechart!` macro emits a concrete `BitSet<W>` sized to the machine's
/// state count; the kernel only ever operates on it through [`Configuration`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitSet<const W: usize> {
    words: [u64; W],
}

impl<const W: usize> BitSet<W> {
    /// 🧮 An empty bitset with no bits set.
    pub const fn empty() -> Self {
        Self { words: [0u64; W] }
    }
}

impl<const W: usize> Default for BitSet<W> {
    fn default() -> Self {
        Self::empty()
    }
}

/// 🧩 Active-state configuration operations, implemented by the macro-generated `BitSet<W>`.
///
/// The active configuration of a running machine is a set of atomic `NodeId`s —
/// never a single enum variant — so that parallel regions can hold multiple
/// simultaneously-active states.
pub trait Configuration: Clone + PartialEq + Default {
    /// ➕ Marks `id` as part of the active configuration.
    fn set(&mut self, id: NodeId);
    /// ➖ Removes `id` from the active configuration.
    fn clear(&mut self, id: NodeId);
    /// ❓ Whether `id` is part of the active configuration.
    fn contains(&self, id: NodeId) -> bool;
    /// 🔁 Iterates active `NodeId`s in ascending order.
    fn iter_ones(&self) -> ConfigurationIter<'_, Self>;
    /// 🧹 Clears every bit.
    fn clear_all(&mut self);
    /// ❓ Whether no bit is set.
    fn is_empty(&self) -> bool;
}

/// 🔁 Iterator over the active `NodeId`s of a [`Configuration`].
pub struct ConfigurationIter<'a, C: Configuration + ?Sized> {
    words: &'a [u64],
    word_index: usize,
    current: u64,
    _marker: core::marker::PhantomData<C>,
}

impl<'a, C: Configuration + ?Sized> Iterator for ConfigurationIter<'a, C> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        loop {
            if self.current != 0 {
                let bit = self.current.trailing_zeros();
                self.current &= self.current - 1;
                return Some(NodeId((self.word_index as u16 - 1) * 64 + bit as u16));
            }
            self.word_index += 1;
            self.current = *self.words.get(self.word_index - 1)?;
        }
    }
}

impl<const W: usize> Configuration for BitSet<W> {
    fn set(&mut self, id: NodeId) {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] |= 1u64 << bit;
    }

    fn clear(&mut self, id: NodeId) {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] &= !(1u64 << bit);
    }

    fn contains(&self, id: NodeId) -> bool {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] & (1u64 << bit) != 0
    }

    fn iter_ones(&self) -> ConfigurationIter<'_, Self> {
        ConfigurationIter {
            words: &self.words,
            word_index: 0,
            current: 0,
            _marker: core::marker::PhantomData,
        }
    }

    fn clear_all(&mut self) {
        self.words = [0u64; W];
    }

    fn is_empty(&self) -> bool {
        self.words.iter().all(|w| *w == 0)
    }
}

//#endregion 🔖Ids

//#region 🔖Schema

/// 🏷️ A consumer-defined event enum, reflected into a dense [`EventId`] space.
///
/// Implemented via `#[derive(StatechartEvent)]` — see `fsm_macros`.
pub trait StatechartEvent: Clone {
    /// 🔢 Number of distinct event variants.
    const EVENT_COUNT: u16;
    /// 🔢 The dense id of this event's variant.
    fn event_id(&self) -> EventId;
    /// 🏷️ The variant's declared name, for diagnostics/inspection/manifest.
    fn event_name(id: EventId) -> &'static str;
}

/// 🎭 A compiled statechart: consumer-owned types bound to a static [`MachineDefinition`].
///
/// Implemented by the marker type generated by `statechart! { machine name { … } }`.
pub trait Machine: Sized + 'static {
    /// 📦 Consumer-owned, mutable data carried alongside the active configuration.
    type Context;
    /// 📨 Consumer-owned event enum, reflected via [`StatechartEvent`].
    type Event: StatechartEvent;
    /// 📥 Consumer-owned input to [`kernel::init`].
    type Input;
    /// 📤 Consumer-owned output produced when the machine reaches a final state.
    type Output: Clone;
    /// 🎇 Consumer-owned effect payloads requested via [`kernel::Command::Effect`].
    type Effect;
    /// 🧮 The macro-sized `BitSet<W>` for this machine's state count.
    type Config: Configuration;

    /// 📐 The compiled static definition backing this machine.
    fn definition() -> &'static kernel::MachineDefinition<Self>;
}

/// 🏷️ Struct-level field name/type metadata, embedded as JSON — feeds TypeScript
/// generation tooling. Implemented via `#[derive(StatechartSchema)]`.
pub trait StatechartSchema {
    /// 🏷️ `{"fields":[{"name":"..","type":".."}, ..]}` for this struct's named fields.
    const SCHEMA_JSON: &'static str;
}

//#endregion 🔖Schema

//#region 🔖Reexports

pub use host::{Host, NativeHost, TestHost};
pub use inspect::{InspectionEvent, Inspector, MicrostepTrace, NullInspector, TraceInspector};
pub use kernel::{
    init, macrostep, timer_elapsed, ActionFn, Command, CommandSink, GuardFn, InputFn, MachineDefinition,
    NodeDef, NodeKind, OutputFn, Snapshot, Status, StepReport, TransitionDef, TransitionKind, Trigger,
    MICROSTEP_LIMIT, ROOT,
};
pub use persist::{persist, restore, Migration, PersistedSnapshot, RestoreError};
pub use runtime::{route_command, ActorLogic, ActorSystem, MachineLogic};

#[cfg(any(test, feature = "testing"))]
pub use testing::{check_invariants, explore, run_conformance, ConformanceStep, Coverage, FsmError, Invariant, Model};

// 🪄 `StatechartEvent` here re-exports the derive macro — it shares its name with the
// `StatechartEvent` trait above without conflict since macros and traits live in
// separate namespaces (the same pattern `serde`/`serde_derive` use).
#[cfg(feature = "macros")]
pub use fsm_macros::{statechart, StatechartEvent, StatechartSchema};

#[cfg(all(feature = "macros", target_arch = "wasm32"))]
pub use fsm_macros::export_wasm_machine;

//#endregion 🔖Reexports

//#region 🔖WasmBridge

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use crate::{ActorId, Host, InvokeId, Machine, TimerId};

    /// 🌐 A [`Host`] backed by browser timers and a JS effect callback. Timers are
    /// polled by the caller via [`WasmHost::due_timers`] (driven by the generated
    /// `export_wasm_machine!` wrapper's `tick()`), matching [`crate::NativeHost`] and
    /// [`crate::TestHost`]'s caller-driven clock — no `async fn` anywhere in the host.
    pub struct WasmHost<M: Machine> {
        effect_callback: Option<js_sys::Function>,
        pending_timers: Vec<(ActorId, TimerId, f64)>,
        started_tasks: Vec<(ActorId, InvokeId)>,
        _marker: core::marker::PhantomData<M>,
    }

    impl<M: Machine> WasmHost<M> {
        /// 🌐 A fresh host with no effect callback registered yet.
        pub fn new() -> Self {
            Self {
                effect_callback: None,
                pending_timers: Vec::new(),
                started_tasks: Vec::new(),
                _marker: core::marker::PhantomData,
            }
        }

        /// 🌐 Registers the JS function invoked as `(actorId: number, effectJson: string) => void`.
        pub fn set_effect_callback(&mut self, callback: js_sys::Function) {
            self.effect_callback = Some(callback);
        }

        /// 🚀 Tasks started via `invoke`, still pending cancellation.
        pub fn started_tasks(&self) -> &[(ActorId, InvokeId)] {
            &self.started_tasks
        }

        /// ⏱️ Removes and returns every timer whose deadline has passed, per `js_sys::Date::now()`.
        pub fn due_timers(&mut self) -> Vec<(ActorId, TimerId)> {
            let now = js_sys::Date::now();
            let mut due = Vec::new();
            self.pending_timers.retain(|(actor, timer, at)| {
                if *at <= now {
                    due.push((*actor, *timer));
                    false
                } else {
                    true
                }
            });
            due
        }
    }

    impl<M: Machine> Default for WasmHost<M> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<M: Machine> Host<M> for WasmHost<M>
    where
        M::Effect: serde::Serialize,
    {
        fn execute_effect(&mut self, actor: ActorId, effect: M::Effect) {
            if let Some(callback) = &self.effect_callback {
                if let Ok(json) = serde_json::to_string(&effect) {
                    let _ = callback.call2(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_f64(actor.0 as f64), &wasm_bindgen::JsValue::from_str(&json));
                }
            }
        }

        fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64) {
            self.pending_timers.push((actor, timer, js_sys::Date::now() + delay_ms as f64));
        }

        fn cancel_timer(&mut self, actor: ActorId, timer: TimerId) {
            self.pending_timers.retain(|(a, t, _)| !(*a == actor && *t == timer));
        }

        fn start_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.push((actor, invoke));
        }

        fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.retain(|(a, i)| !(*a == actor && *i == invoke));
        }

        fn now_ms(&self) -> u64 {
            js_sys::Date::now() as u64
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bridge::WasmHost;

/// 🎫 A minimal always-compiled `export_wasm_machine!` consumer — proves the wasm
/// target and the whole DSL → kernel → `WasmHost` → JSON-boundary path continuously,
/// independent of any downstream consumer crate.
#[cfg(target_arch = "wasm32")]
mod wasm_smoke {
    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct ToggleContext;

    fn build_context(_input: ()) -> ToggleContext {
        ToggleContext
    }

    crate::statechart! {
        machine toggle {
            context: ToggleContext;
            event Event { Flip }
            input: ();
            output: ();
            effect: ();
            context_from_input: build_context;
            initial: off;
            state off {
                on Flip => on;
            }
            state on {
                on Flip => off;
            }
        }
    }

    crate::export_wasm_machine!(toggle::Toggle, "ToggleMachine");
}

//#endregion 🔖WasmBridge

//#region 🧪Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitset_set_clear_contains() {
        let mut bits = BitSet::<1>::empty();
        assert!(!bits.contains(NodeId(3)));
        bits.set(NodeId(3));
        assert!(bits.contains(NodeId(3)));
        bits.clear(NodeId(3));
        assert!(!bits.contains(NodeId(3)));
    }

    #[test]
    fn bitset_iter_ones_spans_words() {
        let mut bits = BitSet::<2>::empty();
        bits.set(NodeId(0));
        bits.set(NodeId(63));
        bits.set(NodeId(64));
        bits.set(NodeId(100));
        let ids: Vec<u16> = bits.iter_ones().map(|n| n.0).collect();
        assert_eq!(ids, vec![0, 63, 64, 100]);
    }

    #[test]
    fn bitset_clear_all_and_is_empty() {
        let mut bits = BitSet::<1>::empty();
        assert!(bits.is_empty());
        bits.set(NodeId(5));
        assert!(!bits.is_empty());
        bits.clear_all();
        assert!(bits.is_empty());
    }
}

/// 🎫 End-to-end proof that `statechart!` DSL → kernel → runtime → [`TestHost`] timers/invoke
/// → persist/restore → inspection trace → model coverage all compose over one real machine.
#[cfg(feature = "macros")]
#[cfg(test)]
mod checkout_integration {
    use crate::{ActorSystem, CommandSink, InvokeId, Machine, TestHost, TimerId, TraceInspector};

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CheckoutContext {
        pub attempts: u32,
        pub method_set: bool,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Receipt {
        pub attempts: u32,
    }

    fn build_context(_input: ()) -> CheckoutContext {
        CheckoutContext::default()
    }

    fn allow_select(ctx: &CheckoutContext, _event: Option<&checkout::Event>) -> bool {
        ctx.attempts < 3
    }

    fn set_method(ctx: &mut CheckoutContext, _event: Option<&checkout::Event>, _sink: &mut dyn CommandSink<checkout::Checkout>) {
        ctx.method_set = true;
    }

    fn note_timeout(ctx: &mut CheckoutContext, _event: Option<&checkout::Event>, _sink: &mut dyn CommandSink<checkout::Checkout>) {
        ctx.attempts += 1;
    }

    fn make_receipt(ctx: &CheckoutContext) -> Receipt {
        Receipt { attempts: ctx.attempts }
    }

    crate::statechart! {
        machine checkout {
            context: CheckoutContext;
            event Event { Confirm, SelectMethod, PaymentSucceeded, PaymentFailed, Retry, Cancel, Resume, ShipDone, InvoiceDone }
            input: ();
            output: Receipt;
            effect: ();
            context_from_input: build_context;
            output_from_context: make_receipt;
            initial: cart;

            state cart {
                on Confirm => payment;
                on Resume => payment_history;
            }
            state payment {
                initial: selecting;
                history payment_history shallow;
                state selecting {
                    on SelectMethod if allow_select => processing do set_method;
                }
                state processing {
                    invoke charge;
                    after 5000 => failed do note_timeout;
                    on PaymentSucceeded => fulfilment;
                    on PaymentFailed => failed;
                    on Cancel => cart;
                }
                state failed {
                    on Retry => processing;
                }
            }
            parallel fulfilment {
                state shipping {
                    initial: ship_pending;
                    state ship_pending { on ShipDone => ship_done; }
                    final ship_done;
                }
                state invoicing {
                    initial: invoice_pending;
                    state invoice_pending { on InvoiceDone => invoice_done; }
                    final invoice_done;
                }
                on_done => done;
            }
            final done;
        }
    }

    #[test]
    fn dsl_machine_walks_cart_to_receipt() {
        let mut system: ActorSystem<checkout::Checkout, TestHost<checkout::Checkout>> = ActorSystem::new(TestHost::new());
        let root = system.spawn_root(());
        assert!(system.snapshot(root).unwrap().matches("cart"));

        system.send(root, checkout::Event::Confirm);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("selecting"));

        system.send(root, checkout::Event::SelectMethod);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("processing"));
        assert!(system.snapshot(root).unwrap().context.method_set);
        assert_eq!(system.host.started_tasks(), &[(root, InvokeId(0))]);

        system.send(root, checkout::Event::PaymentSucceeded);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("ship_pending"));
        assert!(system.snapshot(root).unwrap().matches("invoice_pending"));
        assert_eq!(system.host.cancelled_tasks(), &[(root, InvokeId(0))], "leaving processing must stop its invoke");

        system.send(root, checkout::Event::ShipDone);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("ship_done"));
        assert!(system.snapshot(root).unwrap().matches("invoice_pending"), "invoicing region must still be pending");
        system.send(root, checkout::Event::InvoiceDone);
        system.drain();

        assert!(matches!(system.snapshot(root).unwrap().status, crate::Status::Done(_)));
        if let crate::Status::Done(receipt) = &system.snapshot(root).unwrap().status {
            assert_eq!(receipt.attempts, 0);
        }
    }

    #[test]
    fn dsl_machine_cancel_resume_round_trips_via_shallow_history() {
        let mut sink: Vec<crate::Command<checkout::Checkout>> = Vec::new();
        let mut snapshot = crate::init::<checkout::Checkout>((), &mut sink);
        let mut inspector = TraceInspector::<checkout::Checkout>::default();

        crate::macrostep(&mut snapshot, checkout::Event::Confirm, &mut sink, &mut inspector);
        crate::macrostep(&mut snapshot, checkout::Event::SelectMethod, &mut sink, &mut inspector);
        assert!(snapshot.matches("processing"));

        // Cancelling from `processing` exits `payment` entirely (recording shallow
        // history), landing back in `cart`.
        crate::macrostep(&mut snapshot, checkout::Event::Cancel, &mut sink, &mut inspector);
        assert!(snapshot.matches("cart"));
        assert!(!snapshot.matches("payment"));

        // Resuming must restore `processing`, not `payment`'s default `selecting`.
        crate::macrostep(&mut snapshot, checkout::Event::Resume, &mut sink, &mut inspector);
        assert!(snapshot.matches("processing"), "shallow history must restore into processing, not the default selecting");
        assert!(!snapshot.matches("selecting"));
        assert!(!inspector.entries.is_empty());

        let fired = crate::timer_elapsed(&mut snapshot, TimerId(0), &mut sink, &mut inspector);
        assert_eq!(fired.microsteps, 1);
        assert!(snapshot.matches("failed"));
        assert_eq!(snapshot.context.attempts, 1);

        crate::macrostep(&mut snapshot, checkout::Event::Retry, &mut sink, &mut inspector);
        assert!(snapshot.matches("processing"));

        let persisted = crate::persist(&snapshot);
        assert_eq!(persisted.fingerprint, checkout::Checkout::definition().fingerprint);
        let restored = crate::restore::<checkout::Checkout>(&persisted, snapshot.context.clone(), &[]).expect("restore should succeed");
        assert!(restored.matches("processing"));
    }

    #[test]
    fn dsl_machine_coverage_reaches_every_declared_state() {
        let model = crate::Model::<checkout::Checkout>::new(vec![
            checkout::Event::Confirm,
            checkout::Event::SelectMethod,
            checkout::Event::PaymentSucceeded,
            checkout::Event::PaymentFailed,
            checkout::Event::Retry,
            checkout::Event::Cancel,
            checkout::Event::Resume,
            checkout::Event::ShipDone,
            checkout::Event::InvoiceDone,
        ]);
        let coverage = crate::explore(&model, ());
        for expected in ["cart", "selecting", "processing", "failed", "ship_pending", "ship_done", "invoice_pending", "invoice_done", "done"] {
            assert!(coverage.reached_stable_ids.contains(&expected), "expected model exploration to reach `{expected}`, got {:?}", coverage.reached_stable_ids);
        }
    }
}

//#endregion 🧪Tests
