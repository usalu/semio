use super::*;
use semio_plugin::WindowTransientOwner;

/// ♻️ Retires one payload byte per turn with separately admitted constructor and whole release grants.
fn retire<T: semio_framework_value::retirement::RetireOwned>(value: T, maximum_turns: usize) {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, close_original_retained_effect_step};
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneStep, RetainedWorkPoll, retirement::controlled::ControlledRetirement};
    let mut owner = ControlledRetirement::new(value).unwrap_or_else(|(error, _)| panic!("preview retirement owner refused: {error}"));
    let stock = RetainedCloneGrant {
        maximum_items: maximum_turns, maximum_copy_bytes: maximum_turns * 64,
        maximum_capacity_bytes: maximum_turns * 64, maximum_release_bytes: maximum_turns * 64, maximum_depth: 64,
    };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(83), Generation(1), stock, || Some(1000), 1500, 32_768);
    for _ in 0..maximum_turns {
        if owner.terminal_is_empty() {
            caller.retire(stock);
            return;
        }
        let demand = owner.next_demand(1).unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth,
        };
        let step = caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
            close_original_retained_effect_step(cx, demand, |effect| match owner.step(1, effect) {
                RetainedWorkPoll::Pending { .. } => Ok(RetainedCloneStep::Progress(Default::default())),
                RetainedWorkPoll::Complete(turn) => turn.result.map_err(|failure| failure.into_value_error()),
            })
        })).unwrap();
        assert!(step.progress().fits(grant));
    }
    panic!("Generation3d preview retirement exceeded its bounded turns");
}

#[test]
fn preview_eval_retained_capacity_is_admitted_and_retired_with_tiny_grants() {
    let eval_text = "e".repeat(65_536);
    let mutation = Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text: Some(eval_text), generation_id: Some("generation".into()) };
    let footprint = preflight(&mutation).expect("large logical preview fits the Store envelope");
    assert!(footprint.is_admissible() && footprint.retained_bytes >= 65_536);
    retire(mutation, 70_000);
}

#[test]
fn preview_eval_oversized_reserved_capacity_rejects_returns_and_retires_the_exact_owner() {
    let owners = Generation3dPreviewWindowTransientOwner::build_owners();
    let mut eval_text = String::with_capacity(store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
    eval_text.push('e');
    let pointer = eval_text.as_ptr();
    let capacity = eval_text.capacity();
    let request = store::ArtifactEphemeralOneItemPreparationRequest {
        operation: semio_framework_job::OperationId(41),
        generation: semio_framework_job::Generation(3),
        base: store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(std::sync::Arc::new(Generation3dPreviewWindowTransient::default()))),
        mutation: Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text: Some(eval_text), generation_id: Some("generation".into()) },
    };
    let returned = match owners.preparation.begin(request) { Err(request) => request, Ok(_) => panic!("reserved capacity over the Store maximum must reject") };
    let Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text, generation_id } = returned.mutation;
    let eval_text = eval_text.expect("returned evaluation owner");
    assert_eq!(eval_text, "e");
    assert_eq!(eval_text.as_ptr(), pointer);
    assert_eq!(eval_text.capacity(), capacity);
    assert_eq!(generation_id.as_deref(), Some("generation"));
    retire(Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text: Some(eval_text), generation_id }, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 16);
}

#[test]
fn preview_eval_transient_codec_matches_the_independent_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔬️unit/🔣️.json")).expect("neutral fixture");
    for expected in fixture["accepted"].as_array().expect("accepted cases") {
        let typed: Generation3dPreviewWindowTransient = serde_json::from_value(expected.clone()).expect("serde_json oracle accepts");
        assert_eq!(serde_json::to_value(&typed).expect("typed JSON"), *expected);
        let packed = pack::snapshot::ArtifactPack::encode_pack_with(&typed, &Default::default()).expect("pack");
        assert_eq!(<Generation3dPreviewWindowTransient as pack::snapshot::ArtifactPack>::decode_pack_with(&packed, &Default::default()).expect("unpack"), typed);
    }
}

#[test]
fn preview_visibility_uses_the_published_generation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/generation-preview-identity.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let state: Generation3dPreviewWindowTransient = serde_json::from_value(row["stored"].clone()).unwrap();
        let selected = row["selected"].as_str();
        let view = if row["mode"] == "generate" {
            selected.and_then(|id| state.evaluation_for(Some(id)))
        } else {
            state.evaluation_for(None)
        };
        assert_eq!(view, row["visible"].as_str(), "{}", row["name"]);
        let mutation = Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text: view.map(str::to_owned), generation_id: selected.map(str::to_owned) };
        let binary = protocol::OpBinary::encode_op(&mutation).unwrap();
        assert_eq!(<Generation3dPreviewWindowTransientMutation as protocol::OpBinary>::decode_op(&binary).unwrap(), mutation);
        let text = protocol::OpText::print_op(&mutation);
        assert_eq!(<Generation3dPreviewWindowTransientMutation as protocol::OpText>::parse_op(&text).unwrap(), mutation);
    }
}
