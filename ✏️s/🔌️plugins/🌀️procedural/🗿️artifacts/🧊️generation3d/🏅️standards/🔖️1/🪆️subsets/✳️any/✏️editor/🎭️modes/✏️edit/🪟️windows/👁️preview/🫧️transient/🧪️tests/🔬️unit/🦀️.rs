use super::*;
use semio_framework_plugin::WindowTransientOwner;

/// ♻️ Walks a `RetirementCursor` tree under the smallest positive grant there is (one byte per
/// turn), pushing each `Child` cursor the owner hands back — the exact shape
/// `store::retirement::RetirementStep` declares, so a nested owner is retired explicitly instead of
/// being dropped recursively.
fn retire(cursor: Box<dyn store::retirement::RetirementCursor>, maximum_turns: usize) {
    let mut stack = vec![cursor];
    for _ in 0..maximum_turns {
        let Some(top) = stack.last_mut() else { return };
        match top.close_step(1) {
            store::retirement::RetirementStep::Child(child) => stack.push(child),
            store::retirement::RetirementStep::Bytes(released_bytes) => assert!(released_bytes <= 1),
            store::retirement::RetirementStep::Complete => {
                assert!(top.terminal_is_empty());
                stack.pop();
            }
            store::retirement::RetirementStep::BudgetExhausted => panic!("positive tiny retirement grant was reported exhausted"),
        }
    }
    panic!("Generation3d preview retirement exceeded its bounded turns");
}

#[test]
fn preview_eval_retained_capacity_is_admitted_and_retired_with_tiny_grants() {
    let eval_text = "e".repeat(65_536);
    let mutation = Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text: Some(eval_text) };
    let footprint = preflight(&mutation).expect("large logical preview fits the Store envelope");
    assert!(footprint.is_admissible() && footprint.retained_bytes >= 65_536);
    retire(store::retirement::RetireOwned::retirement(mutation), 70_000);
}

#[test]
fn preview_eval_oversized_reserved_capacity_rejects_returns_and_retires_the_exact_owner() {
    let owners = Generation3dPreviewWindowTransientOwner::build_owners();
    let mut eval_text = String::with_capacity(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
    eval_text.push('e');
    let pointer = eval_text.as_ptr();
    let capacity = eval_text.capacity();
    let request = store::ArtifactEphemeralOneItemPreparationRequest {
        operation: semio_framework_job::OperationId(41),
        generation: semio_framework_job::Generation(3),
        base: store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(std::sync::Arc::new(Generation3dPreviewWindowTransient::default()))),
        mutation: Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text: Some(eval_text) },
    };
    let returned = match owners.preparation.begin(request) { Err(request) => request, Ok(_) => panic!("reserved capacity over the Store maximum must reject") };
    let Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text } = returned.mutation;
    let eval_text = eval_text.expect("returned evaluation owner");
    assert_eq!(eval_text, "e");
    assert_eq!(eval_text.as_ptr(), pointer);
    assert_eq!(eval_text.capacity(), capacity);
    retire(store::retirement::RetireOwned::retirement(Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text: Some(eval_text) }), store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 16);
}

#[test]
fn preview_eval_transient_codec_matches_the_independent_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️unit/🔣️.json")).expect("neutral fixture");
    for expected in fixture["accepted"].as_array().expect("accepted cases") {
        let typed: Generation3dPreviewWindowTransient = serde_json::from_value(expected.clone()).expect("serde_json oracle accepts");
        assert_eq!(serde_json::to_value(&typed).expect("typed JSON"), *expected);
        let packed = store::ArtifactPack::encode_pack_with(&typed, &Default::default()).expect("pack");
        assert_eq!(<Generation3dPreviewWindowTransient as store::ArtifactPack>::decode_pack_with(&packed, &Default::default()).expect("unpack"), typed);
    }
}
