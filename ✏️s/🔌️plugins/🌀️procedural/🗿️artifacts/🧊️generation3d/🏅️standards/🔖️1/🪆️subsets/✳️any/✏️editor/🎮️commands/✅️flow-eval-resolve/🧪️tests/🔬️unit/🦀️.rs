use super::*;
use crate::editor::generation3d::unit_tests::context::{empty_history_view, retire_flow_eval_session};
use semio_framework_artifact_flow_flow::neural::{Atom, ColdRetire, Dictionary, Value as NeuralValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect};

/// ⚖️ LAW: an `evaluate` extension result carried back as `flowEvalResolve` seeds the session's shared
/// neural cache under the requested `nodeHash` and re-arms the `flowEvalTick` chain, so the very next
/// tick reads the extension's output out of cache instead of re-dispatching the same node forever.
///
/// 🧹️ Every neural owner this test touches is retired explicitly: both `Dictionary`s through
/// `ColdRetire`, the borrowed `Arc<NeuralCache>` before the session claims its cache root, and the
/// session itself through the granted close loop — each one panics on a live drop by design.
#[test]
fn eval_result_seeds_the_node_cache_and_rearms_the_tick_chain() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let cache = session.neural_cache();
    let output = Dictionary::new().insert("geometry", NeuralValue::Atom(Atom::String("brep:solid-1".into()))).insert("count", NeuralValue::Atom(Atom::Integer(3)));
    let node_hash = 0x5eed_c0de_u64;
    assert!(!cache.contains(node_hash), "the node must be uncached before the extension answers");
    let emit = handle(&FlowEvalResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: dsl::json::to_json_string(&output), extension_id: String::new(), ok: true, fault_code: String::new(), fault_message: String::new() }, &doc, &cfg, &mut session).expect("flowEvalResolve");
    let cached = cache.get(node_hash).expect("the seeded node must be readable from the shared neural cache");
    assert_eq!(cached, output, "the extension output must round-trip through the shared neural cache");
    cached.retire_cold();
    output.retire_cold();
    drop(cache);
    let rearmed: Vec<&Option<dsl::DslValue>> = emit.effects.iter().filter_map(|effect| match effect {
        Effect::DispatchAction { action, args, .. } if action == "flowEvalTick" => Some(args),
        _ => None,
    })
    .collect();
    assert_eq!(rearmed.len(), 1, "resolving one node must re-arm exactly one flowEvalTick continuation");
    assert_eq!(
        rearmed[0].as_ref().and_then(|args| args.get("windowId")).and_then(dsl::DslValue::as_str),
        Some("procedural-preview-test"),
        "the re-armed tick must keep addressing the preview window the resolved evaluation belongs to — an unaddressed re-arm is refused by the window-scoped route (ticket 26/09/09/PROCEDURAL-3D-END-TO-END)"
    );
    retire_flow_eval_session(session);
}
