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
