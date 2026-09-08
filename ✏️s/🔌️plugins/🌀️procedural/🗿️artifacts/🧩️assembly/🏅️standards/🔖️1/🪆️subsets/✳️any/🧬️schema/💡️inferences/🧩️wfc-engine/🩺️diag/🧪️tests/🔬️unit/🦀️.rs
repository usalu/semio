
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

#[test]
fn diag_levels_are_ordered() {
    assert!(DiagLevel::Off < DiagLevel::Summary);
    assert!(DiagLevel::Summary < DiagLevel::Decisions);
    assert!(DiagLevel::Decisions < DiagLevel::Full);
}

#[test]
fn emit_detailed_is_suppressed_below_decisions_level() {
    let mut summary_sink = EventSink::new(DiagLevel::Summary);
    summary_sink.emit_detailed(Event::Observed { node: NodeId(0), chosen: PatternId(0) });
    assert!(summary_sink.into_events().is_empty());

    let mut decisions_sink = EventSink::new(DiagLevel::Decisions);
    decisions_sink.emit_detailed(Event::Observed { node: NodeId(0), chosen: PatternId(0) });
    assert_eq!(decisions_sink.into_events().len(), 1);
}

#[test]
fn trace_replay_extracts_only_observed_events_in_order() {
    let report = RunReport {
        metrics: Metrics::default(),
        model_fingerprint: 42,
        seed: 7,
        events: vec![Event::Observed { node: NodeId(0), chosen: PatternId(1) }, Event::Backtracked { node: NodeId(0), candidate: PatternId(1) }, Event::Observed { node: NodeId(0), chosen: PatternId(2) }, Event::Solved],
    };
    let trace = TraceReplay::from_report(&report);
    assert_eq!(trace.model_fingerprint, 42);
    assert_eq!(trace.seed, 7);
    assert_eq!(trace.decisions, vec![(NodeId(0), PatternId(1)), (NodeId(0), PatternId(2))]);
}

#[test]
fn trace_replay_matches_identical_sequences_and_rejects_divergent_ones() {
    let a = TraceReplay { model_fingerprint: 1, seed: 2, decisions: vec![(NodeId(0), PatternId(0))] };
    let b = TraceReplay { model_fingerprint: 1, seed: 2, decisions: vec![(NodeId(0), PatternId(0))] };
    assert!(a.matches(&b));

    let diverged = TraceReplay { model_fingerprint: 1, seed: 2, decisions: vec![(NodeId(0), PatternId(1))] };
    assert!(!a.matches(&diverged));
}
