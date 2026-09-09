//! 📊️ Diagnostics: run metrics plus a level-gated event stream. `DiagLevel::Off` keeps the sink a
//! no-op (no allocation, no branching cost beyond one comparison) so instrumentation never taxes a
//! production solve that doesn't ask for it.

use crate::wfc_engine::ids::{NodeId, PatternId};
use crate::wfc_engine::outcome::RunReport;

// #region 🔖️Level
/// 📊️ How much event detail a solve records, from cheapest to most complete. Ordered
/// (`Off < Summary < Decisions < Full`) so call sites can gate emission with a single comparison
/// (`level >= DiagLevel::Decisions`) instead of matching every variant.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DiagLevel {
    #[default]
    Off,
    /// 📊️ High-level outcomes only: `Solved`/`Contradiction`/`Restarted`/`BudgetExceeded`.
    Summary,
    /// 📊️ Adds one `Observed`/`Backtracked` event per decision — enough to reconstruct the exact
    /// decision sequence via [`TraceReplay`] for a determinism/golden-replay check.
    Decisions,
    /// 📊️ Reserved for finer-grained propagation-level tracing (e.g. per-arc domain reductions);
    /// today behaves identically to `Decisions` — no engine in this crate yet emits anything
    /// beyond decision-level events. Selecting it is forward-compatible: call sites already gate
    /// on `>= Decisions`, so a later phase can add `Full`-only events without revisiting them.
    Full,
}
// #endregion 🔖️Level

// #region 🔖️Event
/// 📊️ One notable occurrence during a solve.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    Solved,
    Contradiction {
        node: NodeId,
    },
    Restarted,
    BudgetExceeded,
    /// 📊️ `DiagLevel::Decisions` and above: a decision was made — `node` was assigned `chosen`.
    Observed {
        node: NodeId,
        chosen: PatternId,
    },
    /// 📊️ `DiagLevel::Decisions` and above: `candidate` was ruled out at `node` and the decision
    /// undone (chronological backtrack or constraint-rejection repair).
    Backtracked {
        node: NodeId,
        candidate: PatternId,
    },
}

/// 📊️ Level-gated event buffer.
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

    /// 📊️ Only records `event` at `DiagLevel::Decisions` or above — for the fine-grained events a
    /// `Summary`-level caller doesn't want paying allocation cost for.
    #[inline]
    pub fn emit_detailed(&mut self, event: Event) {
        if self.level >= DiagLevel::Decisions {
            self.events.push(event);
        }
    }

    pub fn into_events(self) -> Vec<Event> {
        self.events
    }
}
// #endregion 🔖️Event

// #region 🔖️Replay
/// 📊️ The ordered decision sequence recorded by a `DiagLevel::Decisions`-or-above solve, replayable
/// to verify determinism: the same model + same seed must always reach the same
/// (node, chosen-pattern) sequence in the same order. Extracted from a [`RunReport`]'s event
/// stream — empty if that solve ran below `Decisions` level (no `Observed` events to extract).
#[derive(Clone, PartialEq, Debug)]
pub struct TraceReplay {
    pub model_fingerprint: u64,
    pub seed: u64,
    pub decisions: Vec<(NodeId, PatternId)>,
}

impl TraceReplay {
    pub fn from_report(report: &RunReport) -> Self {
        let decisions = report
            .events
            .iter()
            .filter_map(|e| match e {
                Event::Observed { node, chosen } => Some((*node, *chosen)),
                _ => None,
            })
            .collect();
        Self { model_fingerprint: report.model_fingerprint, seed: report.seed, decisions }
    }

    /// 📊️ The golden-replay determinism check: same model, same seed, same decisions in the same
    /// order.
    pub fn matches(&self, other: &TraceReplay) -> bool {
        self.model_fingerprint == other.model_fingerprint && self.seed == other.seed && self.decisions == other.decisions
    }
}
// #endregion 🔖️Replay

// #region 🔖️Metrics
/// 📊️ Aggregate counters for one solve attempt (one restart's worth, unless a caller sums across
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
// #endregion 🔖️Metrics

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
