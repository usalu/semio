//! 🧪️ The Process3d document genesis is born through an original caller, installs one shared semantic issuer and closes every owner; its authored state is checked against the shared fixture with a `serde_json` oracle.

use super::*;
use semio_framework_job::{default_now_us, Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneStep, RetainedWorkOutcome};
use semio_plugin_store_lifecycle::original_store_genesis::{OriginalStoreBirth, OriginalStoreGenesis, OriginalStoreSlot};

type Birth = OriginalStoreGenesis<Process3dDocumentGenesis>;

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("🔣️.json")).unwrap()
}

fn policy(law: &serde_json::Value) -> RetainedCloneGrant {
    let turn = &law["turn"];
    RetainedCloneGrant { maximum_items: turn["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: turn["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: turn["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: turn["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: 4096 }
}

fn caller() -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(OperationId(9821), Generation(3), RetainedCloneGrant { maximum_items: 1 << 28, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 29, maximum_release_bytes: 1 << 29, maximum_depth: 4096 }, default_now_us, u64::MAX, 1 << 30)
}

fn call<T>(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner, grant: RetainedCloneGrant, run: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    caller.turn_with_decoder(grant, decoder, run).unwrap()
}

#[test]
fn the_authored_initial_document_matches_the_shared_fixture() {
    let law = law();
    let snapshot = empty_process3d_snapshot();
    assert_eq!(snapshot.step_payloads.len() as u64, law["stepCount"].as_u64().unwrap());
    assert_eq!(snapshot.tool_solids.len() as u64, law["toolSolidCount"].as_u64().unwrap());
    assert_eq!(snapshot.workshop.machines.len() as u64, law["machineCount"].as_u64().unwrap());
    assert_eq!(<Process3dDocumentGenesis as OriginalStoreGenesisOwner>::SCHEMA, law["schema"].as_str().unwrap());
    assert_eq!(<Process3dDocumentGenesis as OriginalStoreGenesisOwner>::MAXIMUM_INITIAL_PACK_BYTES as u64, law["maximumInitialPackBytes"].as_u64().unwrap());
    assert_eq!(<Process3dDocumentGenesis as OriginalStoreGenesisOwner>::artifact_dialect().unwrap().artifact_kind, law["artifactKind"].as_str().unwrap());
    let demand = <Process3dDocumentGenesis as OriginalFixedStoreInitial>::initial_state_demands().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: 0, maximum_depth: demand.depth };
    let (state, progress) = <Process3dDocumentGenesis as OriginalFixedStoreInitial>::initial_state(grant).unwrap();
    assert_eq!(state, snapshot);
    assert_eq!(progress.retained_capacity_bytes, demand.capacity_bytes);
    let starved = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant };
    assert!(<Process3dDocumentGenesis as OriginalFixedStoreInitial>::initial_state(starved).is_err());
}

#[test]
fn original_process_document_genesis_installs_the_bounded_shared_issuer_and_closes_every_owner() {
    let law = law();
    let grant = policy(&law);
    let mut caller = caller();
    let mut decoder = OriginalJobDecodeOwner::empty();
    let actor = store::ActorId("native-process-reader".into());
    let address = law["address"].as_str().unwrap();
    let mut frame = OriginalStoreBirth::default();
    let mut birth = None;
    for index in 0..1_048_576 {
        let _ = call(&mut caller, &mut decoder, grant, |cx| Birth::admit(address, &actor, cx, &mut frame, &mut birth));
        if birth.is_some() {
            break;
        }
        assert!(index < 1_048_575, "original Process3d genesis birth did not complete");
    }
    let mut output = None;
    for index in 0..1_048_576 {
        output = call(&mut caller, &mut decoder, grant, |cx| birth.as_mut().unwrap().step(cx));
        if output.is_some() {
            break;
        }
        assert!(index < 1_048_575, "original Process3d document genesis retained stage {:?}", birth.as_ref().unwrap().progress());
    }
    let (born, store) = output.unwrap();
    assert_eq!(born, address);
    assert_eq!(format!("{:?}", store.envelope().history_shape).to_lowercase(), law["historyShape"].as_str().unwrap());
    let owners = &store.parts_ref().owners;
    assert_eq!(std::sync::Arc::as_ptr(owners.one_item_preparation_factory.as_ref().unwrap()) as *const (), std::sync::Arc::as_ptr(owners.replay_preparation_factory.as_ref().unwrap()) as *const ());
    for index in 0..1_048_576 {
        if birth.as_ref().unwrap().terminal_is_empty() {
            break;
        }
        call(&mut caller, &mut decoder, grant, |cx| cx.with_original_retirement(|cx| birth.as_mut().unwrap().close_step(cx)));
        assert!(index < 1_048_575);
    }
    call(&mut caller, &mut decoder, grant, |cx| {
        cx.with_original_retirement(|cx| {
            let mut demand = Birth::boxed_release_demands();
            demand.copy_bytes += size_of::<String>();
            demand.release_bytes += born.capacity();
            semio_framework_job::close_original_retained_step(cx, demand, |_| {
                birth.take();
                drop(born);
                RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() })), work: 1 }
            })
        })
    });
    let mut store = OriginalStoreSlot::from_original(store);
    for index in 0..1_048_576 {
        if store.terminal_is_empty() {
            break;
        }
        call(&mut caller, &mut decoder, grant, |cx| cx.with_original_retirement(|cx| store.close_step(cx)));
        assert!(index < 1_048_575, "original Process3d document Store closure retained its native cursor");
    }
    for index in 0..1_048_576 {
        if frame.terminal_is_empty() {
            break;
        }
        let _ = call(&mut caller, &mut decoder, grant, |cx| cx.with_original_retirement(|cx| frame.close_step(cx)));
        assert!(index < 1_048_575, "original Process3d genesis birth frame retained its construction");
    }
    while !decoder.terminal_is_empty() {
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
            decoder.retirement_demands(32768)?;
            decoder.close_on_source(cx)?;
            Ok(())
        })).unwrap();
    }
    caller.retire(grant);
}
