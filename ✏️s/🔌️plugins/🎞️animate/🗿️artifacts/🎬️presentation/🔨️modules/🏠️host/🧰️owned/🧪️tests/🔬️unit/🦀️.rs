//! 🧪️ Exact native host owners preserve retirement and store publication laws.

use super::*;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::standards::v1::subsets::any::schema::mutations::create_tile;

use store::{os_store::test_support, ArtifactCommand};

#[semio_framework_async_macros::async_test]
async fn presentation_deck_materializes() {
    // 🔐️ Through the owner-installing constructor: a bare `new` installs no catalog and
    // `reserve_edit_history_slot` then refuses every `Apply`
    // (`edit history insertion requires its exact mutation retirement factory`).
    let mut store = new_presentation_store(create_document_envelope(PRESENTATION_DOCUMENT_SCHEMA, "animate-presentation", empty_presentation_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![PresentationMutation::CreateTile(create_tile::CreateTile { index: 0, tile: crate::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: crate::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } } })],
            transaction: None,
        })
        .await
        .expect("apply");
    assert_eq!(crate::presentation_working_scene(&store.snapshot().expect("projection")).1.len(), 1);
}

//#region 🔖️DocumentTextTests
#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_with_operation_applied() {
    // 🔐️ Through the owner-installing constructor: a bare `new` installs no catalog and
    // `reserve_edit_history_slot` then refuses every `Apply`
    // (`edit history insertion requires its exact mutation retirement factory`).
    let mut store = new_presentation_store(create_document_envelope(PRESENTATION_DOCUMENT_SCHEMA, "animate-presentation", crate::default_presentation_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![PresentationMutation::CreateTile(create_tile::CreateTile { index: 0, tile: crate::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: crate::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } } })],
            transaction: None,
        })
        .await
        .expect("apply");
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
}
//#endregion 🔖️DocumentTextTests

/// 🧯️ LAW: a Drop witness must never turn a REPORTED failure into a process abort.
///
/// Every owner in this file asserts in `Drop` that the bounded close protocol ran. That witness
/// exists to catch a leak on a HEALTHY path. If it also fires while the thread is already unwinding
/// from a test's own failed assertion, the second panic is a `panic in a destructor during cleanup`
/// — a NON-unwinding abort that kills the whole test binary, so the first, real failure is never
/// printed and every other test in the binary is lost with it.
///
/// This law can only pass when the guard is there: without it the panic below aborts the process
/// instead of being caught here.
#[test]
fn a_live_owner_dropped_during_a_panic_unwinds_instead_of_aborting() {
    let mut authority = PresentationPackSnapshotAuthority::new(semio_framework_job::OperationId(9_101), semio_framework_job::Generation(1), store::OwnedSchemaPath::ROOT);
    *authority.value = Some(empty_presentation_snapshot());
    assert!(!store::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(&authority), "the fixture must hold a LIVE owner, or this law proves nothing");
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _live = authority;
        panic!("a test failure while a retained owner is still live");
    }));
    std::panic::set_hook(previous);
    assert!(outcome.is_err(), "the fixture panic must reach this caller as an unwind");
}
