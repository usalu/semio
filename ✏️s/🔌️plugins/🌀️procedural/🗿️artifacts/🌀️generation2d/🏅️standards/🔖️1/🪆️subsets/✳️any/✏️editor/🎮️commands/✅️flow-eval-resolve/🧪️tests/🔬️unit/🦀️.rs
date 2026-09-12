use super::*;
use crate::editor::generation2d::unit_tests::context::{empty_history_view, retire_flow_eval_session};
use semio_framework_artifact_flow_flow::neural::{Atom, ColdRetire, Dictionary, Value as NeuralValue};
use semio_framework_plugin::{ArtifactView, ConfigView};

/// ⚖️ LAW: an `evaluate` extension result carried back as `flowEvalResolve` seeds the session's shared
/// neural cache under the requested `nodeHash` and re-arms the `flowEvalTick` chain, so the very next
/// tick reads the extension's output out of cache instead of re-dispatching the same node forever.
///
/// 🧹️ Every neural owner this test touches is retired explicitly: both `Dictionary`s through
/// `ColdRetire`, the borrowed `Arc<NeuralCache>` before the session claims its cache root, and the
/// session itself through the granted close loop — each one panics on a live drop by design.
#[test]
fn eval_result_seeds_the_node_cache_and_rearms_the_tick_chain() {
    let snapshot = Generation2dSnapshot::default();
    let history = empty_history_view();
    let config = Generation2dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let cache = session.neural_cache();
    let output = Dictionary::new().insert("drawing", NeuralValue::Atom(Atom::String("2d:polyline-1".into()))).insert("count", NeuralValue::Atom(Atom::Integer(2)));
    let node_hash = 0x2d2d_beef_u64;
    assert!(!cache.contains(node_hash), "the node must be uncached before the extension answers");
    let emit = handle(&FlowEvalResolve { node_hash, output_json: dsl::json::to_json_string(&output) }, &doc, &cfg, &mut session).expect("flowEvalResolve");
    let cached = cache.get(node_hash).expect("the seeded node must be readable from the shared neural cache");
    assert_eq!(cached, output, "the extension output must round-trip through the shared neural cache");
    cached.retire_cold();
    output.retire_cold();
    drop(cache);
    assert_eq!(
        emit.effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")).count(),
        1,
        "resolving one node must re-arm exactly one flowEvalTick continuation"
    );
    retire_flow_eval_session(session);
}
