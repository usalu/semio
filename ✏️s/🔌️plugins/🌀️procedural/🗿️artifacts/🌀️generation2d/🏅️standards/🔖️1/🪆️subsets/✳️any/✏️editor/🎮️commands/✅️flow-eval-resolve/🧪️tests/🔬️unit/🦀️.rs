use super::*;
use crate::editor::generation2d::unit_tests::context::{empty_history_view, retire_flow_eval_session};
use semio_framework_artifact_flow_flow::neural::{Atom, ColdRetire, Dictionary, Value as NeuralValue};
use semio_framework_plugin::{ArtifactView, ConfigView};

/// ⚖️ LAW: an `evaluate` extension answer carried back as `flowEvalResolve` seeds the addressed session's
/// shared neural cache under the requested `nodeHash`, settles the window's outstanding answer and arms
/// nothing itself — the `previewEval` run job schedules the next hop off the latch.
///
/// 🧹️ Every neural owner this test touches is retired explicitly: both `Dictionary`s through
/// `ColdRetire`, the borrowed `Arc<NeuralCache>` before the session claims its cache root, and the
/// session itself through the granted close loop — each one panics on a live drop by design.
#[test]
fn eval_result_seeds_the_node_cache_settles_the_window_and_arms_nothing() {
    let snapshot = Generation2dSnapshot::default();
    let history = empty_history_view();
    let config = Generation2dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let window = "generation2d-preview-1";
    assert!(session.arm_window_tick(window));
    session.begin_window_tick(window);
    session.note_window_tick_outcome(window, true);
    session.note_window_extensions_in_flight(window, 1);
    let cache = session.neural_cache();
    let output = Dictionary::new().insert("drawing", NeuralValue::Atom(Atom::String("2d:polyline-1".into()))).insert("count", NeuralValue::Atom(Atom::Integer(2)));
    let node_hash = 0x2d2d_beef_u64;
    assert!(!cache.contains(node_hash), "the node must be uncached before the extension answers");
    let payload = FlowEvalResolve { window_id: window.into(), window_kind_id: "generation2d-preview".into(), node_hash, output_json: dsl::json::to_json_string(&output), ok: true, ..Default::default() };
    let emit = handle(&payload, &doc, &cfg, &mut session).expect("flowEvalResolve");
    let cached = cache.get(node_hash).expect("the seeded node must be readable from the shared neural cache");
    assert_eq!(cached, output, "the extension output must round-trip through the shared neural cache");
    cached.retire_cold();
    output.retire_cold();
    drop(cache);
    assert!(emit.effects.is_empty(), "an answer arms no continuation of its own: {:?}", emit.effects);
    assert_eq!(session.window_extensions_in_flight(window), 0, "the answer settles the window's outstanding request");
    assert!(session.window_tick_owed(window), "a settled, unfinished window owes the run its next hop");
    retire_flow_eval_session(session);
}
