#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs"]
mod replacement;

use semio_framework_value::{
    ArtifactOwnedValueRetirementFactory, RetainedCloneGrant, RetainedCloneProgress,
    RetainedWorkPoll, ValueError, factory::OwnedValueRetirementFactory,
};
use semio_os_spr::InputReplacement;
use semio_os_store_operation_wire::{ArtifactOperationWire, ArtifactOriginalOperationReceiving};
use semio_plugin_testing_laws::artifact_app_laws::{OriginalFixtureCaller, fixture_renew_turn};
use std::sync::Arc;

fn close_turn(
    receiving: &mut replacement::OriginalReplacementReceiving<Vec<u8>>,
    row: &mut InputReplacement,
    cx: &mut semio_framework_job::StepContext<'_>,
    grant: RetainedCloneGrant,
) -> RetainedCloneProgress {
    let (ceiling, _) = cx.original_turn_ceiling();
    fixture_renew_turn(cx, OriginalFixtureCaller::turn_fuel(), ceiling).unwrap();
    semio_plugin_time_travel::receive_step(cx, |native, _| {
        native.with_effect(grant, 128, |native, effect| {
            native.with_retirement(grant, |control| {
                Ok::<_, ValueError>(receiving.close_native(row, control, effect))
            })
        }).map(|result| match result.transpose().unwrap() {
            Some(RetainedWorkPoll::Complete(turn)) => turn.result.unwrap().progress(),
            _ => RetainedCloneProgress::default(),
        })
    }, |result| result.as_ref().copied().unwrap_or_default()).unwrap().unwrap().unwrap()
}

#[test]
fn replacement_source_publishes_same_payload_and_returns_it() {
    let mut caller = OriginalFixtureCaller::admit().unwrap();
    let mut cx = caller.context();
    let (ceiling, _) = cx.original_turn_ceiling();
    let grant = RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: 32_768,
        maximum_capacity_bytes: 262_144,
        maximum_release_bytes: 1_048_576,
        maximum_depth: 4_096,
    };
    let codec = ArtifactOperationWire::<Vec<u8>>::canonical();
    let factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Vec<u8>>> =
        Arc::new(OwnedValueRetirementFactory::default());
    let payload = vec![1, 35, 2, 3, 4];
    let original = payload.as_ptr();
    let mut row = InputReplacement::Input { schema: "puzzle.puzzle3d".into(), payload };
    let mut receiving = replacement::OriginalReplacementReceiving::<Vec<u8>>::default();
    let mut copy_quotes = Vec::new();
    for _ in 0..128 {
        if let Ok(payload) = receiving.borrow_payload() {
            assert_eq!(payload.get().as_ptr(), original);
            assert_eq!(payload.get(), &[1, 35, 2, 3, 4]);
            break;
        }
        fixture_renew_turn(&mut cx, OriginalFixtureCaller::turn_fuel(), ceiling).unwrap();
        let demand = receiving.demands(&codec, grant.maximum_copy_bytes).unwrap();
        let result = semio_plugin_time_travel::receive_step(&mut cx, |native, work| {
            receiving.advance_native(&mut row, &codec, &factory, native, grant, work)
        }, |result| result.as_ref().copied().unwrap_or_default()).unwrap();
        if let Some(progress) = result {
            let progress = progress.unwrap();
            assert_eq!(progress.copied_items, 1);
            assert!(progress.fits(grant));
            assert_eq!(progress.copied_bytes, demand.copy_bytes);
            copy_quotes.push(demand.copy_bytes);
        }
    }
    assert_eq!(receiving.borrow_payload().unwrap().get().as_ptr(), original);
    let publication = copy_quotes[copy_quotes.len() - 2];
    let transfer = copy_quotes[copy_quotes.len() - 1];
    assert_eq!(publication, std::mem::size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<Vec<u8>>>>() + std::mem::size_of::<Option<u64>>());
    assert!(transfer > 0 && transfer < publication);
    println!("[DEBUG] replacement Source publication copy {publication}, transfer copy {transfer}");
    let handback = std::mem::size_of::<Option<semio_framework_value::retained_clone::RetainedCloneSource<Vec<u8>>>>()
        + std::mem::size_of::<Option<Vec<u8>>>() + std::mem::size_of::<Vec<u8>>();
    let mut checked_denial = false;
    for _ in 0..256 {
        if receiving.terminal_is_empty() {
            break;
        }
        let demand = receiving.close_demands(grant.maximum_copy_bytes).unwrap();
        if demand.copy_bytes == handback {
            let deficient = RetainedCloneGrant { maximum_copy_bytes: handback - 1, ..grant };
            assert_eq!(close_turn(&mut receiving, &mut row, &mut cx, deficient), RetainedCloneProgress::default());
            assert_eq!(receiving.close_demands(grant.maximum_copy_bytes).unwrap(), demand);
            assert!(matches!(&row, InputReplacement::Input { payload, .. } if payload.is_empty() && payload.capacity() == 0));
            let exact = RetainedCloneGrant { maximum_copy_bytes: handback, ..grant };
            assert_eq!(close_turn(&mut receiving, &mut row, &mut cx, exact).copied_bytes, handback);
            assert!(matches!(&row, InputReplacement::Input { payload, .. } if payload.as_ptr() == original));
            checked_denial = true;
        } else {
            close_turn(&mut receiving, &mut row, &mut cx, grant);
        }
    }
    assert!(checked_denial);
    assert!(receiving.terminal_is_empty());
    let InputReplacement::Input { payload, .. } = row else { unreachable!() };
    assert_eq!(payload.as_ptr(), original);
    let fuel = cx.fuel_remaining();
    drop(cx);
    caller.retire(fuel).unwrap();
}
