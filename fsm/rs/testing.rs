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

//#region 🔖Invariants

/// 🧭 A named property that must hold of every [`Snapshot`] visited during exploration.
pub struct Invariant<M: Machine> {
    pub name: &'static str,
    pub check: fn(&Snapshot<M>) -> Result<(), String>,
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
pub fn run_conformance<M: Machine>(input: M::Input, steps: &[ConformanceStep<M>]) -> Result<(), String> {
    let mut sink: Vec<Command<M>> = Vec::new();
    let mut snapshot = init::<M>(input, &mut sink);
    for (index, step) in steps.iter().enumerate() {
        let mut inspector = NullInspector;
        macrostep(&mut snapshot, step.event.clone(), &mut sink, &mut inspector);
        for expected in step.expect_active {
            if !snapshot.matches(expected) {
                return Err(format!(
                    "conformance step {index}: expected active state '{expected}', got {:?}",
                    active_stable_ids(&snapshot)
                ));
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
        let err = run_conformance::<UnitToggleMachine>((), &steps).unwrap_err();
        assert!(err.contains("step 0"));
        assert!(err.contains("off"));
    }

    #[test]
    fn invariant_reports_violation_by_name() {
        let mut sink: Vec<Command<UnitToggleMachine>> = Vec::new();
        let snapshot = init::<UnitToggleMachine>((), &mut sink);
        let invariants = [Invariant {
            name: "never off",
            check: |s: &Snapshot<UnitToggleMachine>| if s.matches("off") { Err("was off".to_string()) } else { Ok(()) },
        }];
        let violations = check_invariants(&snapshot, &invariants);
        assert_eq!(violations, vec!["never off: was off".to_string()]);
    }
}

//#endregion 🧪Tests
