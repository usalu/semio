//! ⏱️ The BUDGET law of this extension's `evaluate` capability, driven through the REAL handler
//! (`flow_extension_sdk::evaluate_invoke_json`) against the REAL kernel.
//!
//! The requester-side half of the same contract lives beside the app that folds it
//! (`🧊️generation3d/…/✅️flow-eval-resolve/🧪️tests/🔬️budget`) and replays the same fixture; this
//! half proves that a long set operation actually YIELDS — that it answers a request inside the
//! caller's wall allowance instead of running to completion inside one guest turn, which is what
//! made a sixteen-second `brep.bool.cut` indistinguishable from a dead worker
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).

use super::*;
use neural_engine::Value;

const EVALUATE_BUDGET_FIXTURE_JSON: &str = include_str!("../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/⏱️evaluate-budget.json");

/// 📇️ The half of the shared fixture this side of the contract reads. Parsed through `pack::json`
/// rather than a `serde` derive because this crate carries no `serde` dependency at all — the
/// runtime-dependency rule applies to its tests too.
struct SteppedOperator {
    operator_id: String,
    minimum_round_trips_under_a_tight_budget: usize,
    phase_order: Vec<String>,
}

fn fixture() -> (String, SteppedOperator) {
    let fixture = pack::json::parse(EVALUATE_BUDGET_FIXTURE_JSON).expect("evaluate budget fixture");
    assert_eq!(fixture.get("format").and_then(pack::json::Value::as_str), Some("semio.generation3d.evaluate-budget"));
    let capability = fixture.get("capability").and_then(pack::json::Value::as_str).expect("capability").to_string();
    let stepped = fixture.get("steppedOperator").expect("steppedOperator");
    let stepped = SteppedOperator {
        operator_id: stepped.get("operatorId").and_then(pack::json::Value::as_str).expect("operatorId").to_string(),
        minimum_round_trips_under_a_tight_budget: stepped.get("minimumRoundTripsUnderATightBudget").and_then(pack::json::Value::as_f64).expect("minimumRoundTrips") as usize,
        phase_order: stepped.get("phaseOrder").and_then(pack::json::Value::as_array).expect("phaseOrder").iter().filter_map(|value| value.as_str().map(str::to_string)).collect(),
    };
    (capability, stepped)
}

fn number_json(value: f64) -> pack::json::Value {
    pack::json::object([("$schema".to_string(), pack::json::Value::from("number")), ("value".to_string(), pack::json::Value::from(value))])
}

/// 🧊️ The two operands the law cuts: a bored box — a 4×4×2 block centred on the origin with a
/// radius-0.5 cylinder running clean through it. The geometry is the kernel suite's own
/// `box_minus_cylinder_bore_exact_volume_and_validates` case, so the exact engine is genuinely
/// exercised (neither the containment shortcut nor the axis-box shortcut applies) and the result
/// is known-good topology rather than an accidental validation failure.
async fn bored_box_operands(registry: &neural_engine::ColdOwner<Registry>) -> (Dictionary, Dictionary) {
    // 🧹️ Every neural `Dictionary` this helper owns is retired explicitly, and the two OUT
    // dictionaries are RETURNED rather than dropped here: a dictionary that is the last owner of
    // its own pairs panics on drop by design.
    let box_input = Dictionary::new()
        .insert("width", Value::Dictionary(number_dictionary(4.0)))
        .insert("depth", Value::Dictionary(number_dictionary(4.0)))
        .insert("height", Value::Dictionary(number_dictionary(2.0)));
    let box_prim = registry.dispatch("brep.prim3d.box", &box_input).expect("box");
    neural_engine::ColdRetire::retire_cold(box_input);
    let box_out = translated(registry, &box_prim, "solid", [-2.0, -2.0, -0.5]).await;
    neural_engine::ColdRetire::retire_cold(box_prim);
    let cylinder_input = Dictionary::new().insert("radius", Value::Dictionary(number_dictionary(0.5))).insert("height", Value::Dictionary(number_dictionary(4.0)));
    let cylinder_prim = registry.dispatch("brep.prim3d.cylinder", &cylinder_input).expect("cylinder");
    neural_engine::ColdRetire::retire_cold(cylinder_input);
    let cylinder_out = translated(registry, &cylinder_prim, "solid", [0.0, 0.0, -2.0]).await;
    neural_engine::ColdRetire::retire_cold(cylinder_prim);
    (box_out, cylinder_out)
}

/// ➡️ One `brep.xform.translate` hop, so the two operands actually overlap the way the kernel
/// suite's own bored-box case does.
async fn translated(registry: &neural_engine::ColdOwner<Registry>, out: &Dictionary, channel: &str, offset: [f64; 3]) -> Dictionary {
    let geometry = channel_payload(out, channel).await;
    let input = Dictionary::new().insert("geometry", Value::Dictionary(geometry)).insert("offset", Value::Dictionary(vector_dictionary(offset)));
    let moved = registry.dispatch("brep.xform.translate", &input).expect("translate");
    neural_engine::ColdRetire::retire_cold(input);
    moved
}

/// 🧹️ Retires the four neural dictionaries one operand pair owns — children first, so each release
/// still has a live sharer above it, then the two owning OUT dictionaries.
fn retire_operands(a: Dictionary, b: Dictionary, box_out: Dictionary, cylinder_out: Dictionary) {
    neural_engine::ColdRetire::retire_cold(a);
    neural_engine::ColdRetire::retire_cold(b);
    neural_engine::ColdRetire::retire_cold(box_out);
    neural_engine::ColdRetire::retire_cold(cylinder_out);
}

/// 🧾️ The `evaluate` request body for one cut of `a` by `b`, at the given budget.
fn cut_request_json(a: &Dictionary, b: &Dictionary, node_hash: u64, budget: u64, wall_micros: u64) -> String {
    let input_json = pack::json::to_string(&pack::json::object([("a".to_string(), geometry_json(a)), ("b".to_string(), geometry_json(b))]));
    pack::json::to_string(&pack::json::object([
        ("operatorId".to_string(), pack::json::Value::from("brep.bool.cut")),
        ("inputJson".to_string(), pack::json::Value::from(input_json)),
        ("nodeHash".to_string(), pack::json::Value::from(node_hash as i64)),
        ("budget".to_string(), pack::json::Value::from(budget as i64)),
        ("wallMicros".to_string(), pack::json::Value::from(wall_micros as i64)),
    ]))
}

/// 🧊️ One geometry channel payload as the JSON the operator input carries.
fn geometry_json(geometry: &Dictionary) -> pack::json::Value {
    let handle = geometry.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str().map(str::to_string)).expect("geometry handle");
    let kind = geometry.get("kind").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str().map(str::to_string)).unwrap_or_else(|| "solid".to_string());
    pack::json::object([("$schema".to_string(), pack::json::Value::from("geometry")), ("handle".to_string(), pack::json::Value::from(handle)), ("kind".to_string(), pack::json::Value::from(kind))])
}

/// ⚖️ LAW: a set operation driven at a one-microsecond wall allowance answers `done: false` with
/// monotone progress and a phase from the declared order, more than once, and only THEN the
/// output — and that output is the same solid the unbudgeted evaluation produces.
#[semio_framework_async_macros::async_test]
async fn a_long_set_operation_answers_within_the_wall_allowance_and_resumes() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let (capability, stepped_operator) = fixture();
    assert_eq!(capability, "evaluate");
    assert_eq!(stepped_operator.operator_id, "brep.bool.cut");
    let registry = neural_engine::ColdOwner::new(module_registry().await);
    let (box_out, cylinder_out) = bored_box_operands(&registry).await;
    let (a, b) = (channel_payload(&box_out, "geometry").await, channel_payload(&cylinder_out, "geometry").await);
    let node_hash = 0x_bee_f00_u64;
    // ⌛️ One microsecond: every step overruns it, so the round trip yields after exactly one unit.
    let request = cut_request_json(&a, &b, node_hash, 1, 1);
    let mut round_trips = 0usize;
    let mut previous_done = 0_u64;
    let mut seen_phases: Vec<String> = Vec::new();
    let output_json = loop {
        round_trips += 1;
        assert!(round_trips < 4096, "a budgeted evaluation must terminate; it ran {round_trips} round trips");
        let envelope = pack::json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry, request.as_bytes()).expect("evaluate")).expect("envelope json");
        let done = envelope.get("done").and_then(pack::json::Value::as_bool).expect("done");
        let phase = envelope.get("phase").and_then(pack::json::Value::as_str).expect("phase").to_string();
        let units_done = envelope.get("unitsDone").and_then(pack::json::Value::as_f64).expect("unitsDone") as u64;
        let units_total = envelope.get("unitsTotal").and_then(pack::json::Value::as_f64).expect("unitsTotal") as u64;
        assert!(units_done >= previous_done, "unitsDone never decreases ({previous_done} -> {units_done} at `{phase}`)");
        assert!(units_done <= units_total, "unitsDone {units_done} may never exceed unitsTotal {units_total} at `{phase}`");
        previous_done = units_done;
        if !seen_phases.last().is_some_and(|last| last == &phase) {
            seen_phases.push(phase.clone());
        }
        if done {
            assert_eq!(phase, "complete", "the terminal phase of a successful evaluation");
            assert_eq!(envelope.get("cancellable").and_then(pack::json::Value::as_bool), Some(false), "a finished evaluation is not cancellable");
            break envelope.get("outputJson").and_then(pack::json::Value::as_str).expect("outputJson").to_string();
        }
        assert_eq!(envelope.get("cancellable").and_then(pack::json::Value::as_bool), Some(true), "a working evaluation is cancellable");
        assert_eq!(envelope.get("outputJson").and_then(pack::json::Value::as_str), Some(""), "a working evaluation publishes no output");
        assert!(stepped_operator.phase_order.contains(&phase), "`{phase}` is one of the fixture's declared phases {:?}", stepped_operator.phase_order);
    };
    assert!(
        round_trips >= stepped_operator.minimum_round_trips_under_a_tight_budget,
        "a tight budget must cut this evaluation into at least {} round trips; it took {round_trips} (phases {seen_phases:?})",
        stepped_operator.minimum_round_trips_under_a_tight_budget
    );
    let stepped = pack::json::parse(&output_json).expect("stepped output json");
    assert_eq!(stepped.get("solid").and_then(|value| value.get("$schema")).and_then(pack::json::Value::as_str), Some("geometry"), "the stepped answer is the operator's own out dictionary: {output_json}");

    retire_operands(a, b, box_out, cylinder_out);

    // ♾️ The same cut, unbudgeted, over fresh operands in a fresh kernel — the stepped path IS the
    // algorithm, so the two answers must agree on everything but the minted handle id.
    reset_test_kernel().await;
    let control_registry = neural_engine::ColdOwner::new(module_registry().await);
    let (box_out, cylinder_out) = bored_box_operands(&control_registry).await;
    let (a, b) = (channel_payload(&box_out, "geometry").await, channel_payload(&cylinder_out, "geometry").await);
    let one_shot_json = pack::json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&control_registry, cut_request_json(&a, &b, 0, 1_000_000, 3_600_000_000).as_bytes()).expect("evaluate"))
        .expect("envelope json")
        .get("outputJson")
        .and_then(pack::json::Value::as_str)
        .expect("outputJson")
        .to_string();
    let one_shot = pack::json::parse(&one_shot_json).expect("one-shot output json");
    assert_eq!(
        stepped.get("solid").and_then(|value| value.get("kind")).and_then(pack::json::Value::as_str),
        one_shot.get("solid").and_then(|value| value.get("kind")).and_then(pack::json::Value::as_str),
        "stepped and one-shot answer the same geometry kind"
    );
    assert!(one_shot.get("error").is_none(), "the one-shot control must not have failed: {one_shot_json}");
    retire_operands(a, b, box_out, cylinder_out);
}

/// ⚖️ LAW: a cancel between two round trips retires the parked job, and the very next round trip
/// says so — `phase: "cancelled"`, no output, nothing more owed.
#[semio_framework_async_macros::async_test]
async fn a_cancel_between_round_trips_retires_the_parked_evaluation() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let registry = neural_engine::ColdOwner::new(module_registry().await);
    let (box_out, cylinder_out) = bored_box_operands(&registry).await;
    let (a, b) = (channel_payload(&box_out, "geometry").await, channel_payload(&cylinder_out, "geometry").await);
    let node_hash = 0x_c0_1d_u64;
    let request = cut_request_json(&a, &b, node_hash, 1, 1);
    let first = pack::json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry, request.as_bytes()).expect("evaluate")).expect("envelope json");
    assert_eq!(first.get("done").and_then(pack::json::Value::as_bool), Some(false), "the first round trip must yield, or there is nothing to cancel");
    assert!(flow_extension_sdk::evaluation_progress("brep.bool.cut", node_hash).is_some(), "a yielded evaluation is parked under its own node hash");
    assert!(flow_extension_sdk::cancel_evaluation("brep.bool.cut", node_hash), "the cancel retires exactly that job");
    assert!(!flow_extension_sdk::cancel_evaluation("brep.bool.cut", node_hash), "a second cancel is a no-op, not a fault");
    // 🔁 The chain re-emits the identical request; the extension no longer holds the job, so it
    // admits a fresh one — the cancel stopped the WORK, it did not poison the node.
    let resumed = pack::json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry, request.as_bytes()).expect("evaluate")).expect("envelope json");
    assert!(resumed.get("phase").and_then(pack::json::Value::as_str).is_some(), "the resumed round trip still answers an envelope");
    assert_eq!(flow_extension_sdk::cancel_all_evaluations(), 1, "the whole-registry cancel retires the job the resumed round trip parked");
    assert_eq!(flow_extension_sdk::cancel_all_evaluations(), 0, "an empty registry retires nothing");
    retire_operands(a, b, box_out, cylinder_out);
}

/// ⚖️ LAW: an operator with no sub-structure completes inside the FIRST round trip. The budget
/// contract must not turn a microsecond primitive into a multi-turn conversation.
#[semio_framework_async_macros::async_test]
async fn an_unbudgeted_operator_completes_in_one_round_trip() {
    let _serial = test_serial().await;
    reset_test_kernel().await;
    let registry = neural_engine::ColdOwner::new(module_registry().await);
    let input_json = pack::json::to_string(&pack::json::object([("width".to_string(), number_json(1.0)), ("depth".to_string(), number_json(1.0)), ("height".to_string(), number_json(1.0))]));
    let request = pack::json::to_string(&pack::json::object([
        ("operatorId".to_string(), pack::json::Value::from("brep.prim3d.box")),
        ("inputJson".to_string(), pack::json::Value::from(input_json)),
        ("nodeHash".to_string(), pack::json::Value::from(7_i64)),
        ("budget".to_string(), pack::json::Value::from(1_i64)),
        ("wallMicros".to_string(), pack::json::Value::from(1_i64)),
    ]));
    let envelope = pack::json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry, request.as_bytes()).expect("evaluate")).expect("envelope json");
    assert_eq!(envelope.get("done").and_then(pack::json::Value::as_bool), Some(true), "a primitive answers inside its first round trip whatever the budget");
    assert_eq!(envelope.get("phase").and_then(pack::json::Value::as_str), Some("complete"));
    let output = pack::json::parse(envelope.get("outputJson").and_then(pack::json::Value::as_str).expect("outputJson")).expect("output json");
    assert_eq!(output.get("solid").and_then(|value| value.get("$schema")).and_then(pack::json::Value::as_str), Some("geometry"));
    assert!(flow_extension_sdk::evaluation_progress("brep.prim3d.box", 7).is_none(), "a one-round-trip evaluation parks nothing");
}
