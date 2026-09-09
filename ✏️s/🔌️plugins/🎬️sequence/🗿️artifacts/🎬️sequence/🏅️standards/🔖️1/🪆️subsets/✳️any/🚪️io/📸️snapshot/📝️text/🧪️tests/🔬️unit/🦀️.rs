use super::*;
use crate::{default_snapshot, SequenceStep, SlotRef, StepParams};

#[semio_framework_async_macros::async_test]
async fn dsl_round_trips_default_snapshot() {
    store::os_store::test_support::assert_dsl_round_trip(&default_snapshot());
}

#[semio_framework_async_macros::async_test]
async fn default_sequence_example_dsl_round_trips() {
    let snapshot = parse_dsl(SEQUENCE_EXAMPLE_TEXT).expect("🎬️default.sequence must parse");
    store::os_store::test_support::assert_dsl_round_trip(&snapshot);
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trips_snapshot_with_slots_and_nested_params() {
    use neural_engine::{Atom, Dictionary, Value};
    let mut fixture = default_snapshot().to_fixture();
    fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("flag", Value::Atom(Atom::Boolean(true))), x: 560.0, y: 0.0, slot: None, collapsed: true });
    fixture.steps.push(SequenceStep {
        id: "step-4".into(),
        kind: "log.print".into(),
        params: StepParams::new()
            .insert("message", Value::Atom(Atom::String("nested \"quote\" and \\ backslash".into())))
            .insert("meta", Value::Dictionary(Dictionary::new().insert("count", Value::Atom(Atom::Integer(-3))).insert("ratio", Value::Atom(Atom::Decimal(2.5))))),
        x: 560.0,
        y: 160.0,
        slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
        collapsed: false,
    });
    let snapshot = SequenceSnapshot::from_fixture(fixture);
    store::os_store::test_support::assert_dsl_round_trip(&snapshot);
}
