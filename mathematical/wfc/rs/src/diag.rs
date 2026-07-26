//! 📊 Diagnostics: run metrics plus a level-gated event stream. `DiagLevel::Off` keeps the sink a
//! no-op (no allocation, no branching cost beyond one comparison) so instrumentation never taxes a
//! production solve that doesn't ask for it.

use crate::ids::NodeId;

// #region 🔖Level
/// 📊 How much event detail a solve records. Only `Off`/`Summary` exist in this phase; `Decisions`
/// and `Full` are added once the search engine grows enough state worth tracing (a later phase).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DiagLevel {
    #[default]
    Off,
    Summary,
}
// #endregion 🔖Level

// #region 🔖Event
/// 📊 One notable occurrence during a solve.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    Solved,
    Contradiction { node: NodeId },
    Restarted,
}

/// 📊 Level-gated event buffer.
#[derive(Clone, Debug, Default)]
pub struct EventSink {
    level: DiagLevel,
    events: Vec<Event>,
}

impl EventSink {
    pub fn new(level: DiagLevel) -> Self {
        Self { level, events: Vec::new() }
    }

    #[inline]
    pub fn emit(&mut self, event: Event) {
        if self.level != DiagLevel::Off {
            self.events.push(event);
        }
    }

    pub fn into_events(self) -> Vec<Event> {
        self.events
    }
}
// #endregion 🔖Event

// #region 🔖Metrics
/// 📊 Aggregate counters for one solve attempt (one restart's worth, unless a caller sums across
/// restarts itself).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Metrics {
    pub observations: u64,
    pub propagations: u64,
    pub removals: u64,
    pub backtracks: u64,
    pub restarts: u64,
    pub elapsed_millis: u64,
}
// #endregion 🔖Metrics

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_sink_records_nothing() {
        let mut sink = EventSink::new(DiagLevel::Off);
        sink.emit(Event::Solved);
        assert!(sink.into_events().is_empty());
    }

    #[test]
    fn summary_sink_records_events() {
        let mut sink = EventSink::new(DiagLevel::Summary);
        sink.emit(Event::Solved);
        sink.emit(Event::Contradiction { node: NodeId(3) });
        let events = sink.into_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], Event::Solved);
    }

    #[test]
    fn metrics_default_to_zero() {
        let m = Metrics::default();
        assert_eq!(m.observations, 0);
        assert_eq!(m.backtracks, 0);
    }
}
// #endregion 🔖Tests
