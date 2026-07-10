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
}

//#endregion 🧪Tests
