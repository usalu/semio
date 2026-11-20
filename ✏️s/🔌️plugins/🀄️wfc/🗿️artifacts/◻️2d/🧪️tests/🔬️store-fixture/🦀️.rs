//! 🧪️ Store fixture — the document really mounts: an envelope over a real example, edited through
//! the mutation machinery and closed the way a host closes it.

use crate::mutations::{change_seed, pin_slot, Wfc2dMutation};
use crate::Wfc2dSnapshot;

/// 🏪️ A snapshot survives the native pack envelope AND the DSL envelope, in both directions.
#[test]
fn a_mounted_document_round_trips_through_both_envelopes() {
    let document = crate::examples::wall_roof_facade_strip::document();
    let text = pack::snapshot::ArtifactDsl::print_dsl(&document);
    let parsed = <Wfc2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).expect("the printed document parses");
    assert_eq!(parsed, document);
    let bytes = <Wfc2dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&document);
    let decoded = <Wfc2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("the packed document decodes");
    assert_eq!(decoded, document);
}

/// 🧬️ A short edit ladder applies and inverts against a mounted projection.
#[test]
fn an_edit_ladder_applies_and_inverts() {
    let base = crate::examples::two_room_corridor::document();
    let ladder: Vec<Wfc2dMutation> = vec![change_seed(4242), pin_slot("room-a".into(), "room".into())];
    let mut projection = base.clone();
    let mut inverses: Vec<Vec<Wfc2dMutation>> = Vec::new();
    for mutation in &ladder {
        inverses.push(crate::mutations::inverse_wfc2d_mutation(&projection, mutation).expect("valid retained mutation inverse fixture"));
        vcs::apply_mutation(&projection, mutation).map(|(applied_state, _)| { projection = applied_state; }).expect("ladder step applies");
    }
    assert_ne!(projection, base);
    for steps in inverses.iter().rev() {
        for step in steps {
            vcs::apply_mutation(&projection, step).map(|(applied_state, _)| { projection = applied_state; }).expect("inverse step applies");
        }
    }
    assert_eq!(projection, base, "the edit ladder did not invert back to the mounted document");
}


#[semio_framework_async_macros::async_test]
async fn authored_seed_pins_and_geometry_survive_actual_pack_spr_replay() {
    let corpus: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/🀄️wfc/🧪️tests/🗄️store-replay/🔣️.json")).expect("portable replay corpus");
    assert_eq!(corpus["schema"], "semio.wfc.store-replay/v1");
    let case = corpus["cases"].as_array().expect("replay cases").iter().find(|case| case["artifact"] == "wfc2d").expect("artifact replay case");
    let document = crate::examples::two_room_corridor::document();
    assert_eq!(case["example"], "two-room-corridor");
    assert_ne!(document.seed, 99);
    assert!(document.slots.iter().find(|slot| slot.id == "room-a").expect("free example slot").pinned_tile_id.is_none());
    let schema = document.schema.clone();
    let envelope = store::os_store::document_materialize::create_document_envelope(&schema, "wfc2d", document, None, &store::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = store::ArtifactStore::<Wfc2dSnapshot, Wfc2dMutation>::new::<store::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("replay store opens");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Wfc2dSnapshot, Wfc2dMutation>().expect("funded replay owners")).unwrap_or_else(|(error, _)| panic!("replay owners refused: {error}"));
    let edits: Vec<Wfc2dMutation> = vec![change_seed(99), crate::mutations::move_slot("room-a".into(), 7.0, 9.0), pin_slot("room-a".into(), "room".into())];
    for mutation in edits {
        store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("authored edit applies");
    }
    let observed = store.snapshot().expect("edited replay projection");
    let projection = {
        let slot = observed.slots.iter().find(|slot| slot.id == "room-a").expect("authored slot");
        serde_json::json!({ "seed": observed.seed, "pin": { "slotId": slot.id, "tileId": slot.pinned_tile_id }, "geometry": { "slotId": slot.id, "x": slot.x, "y": slot.y, "width": slot.width, "height": slot.height } })
    };
    assert_eq!(projection, case["expected"]);
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    let mut turns = 0;
    while !store.close_owned_terminal_is_empty() {
        store.close_owned_step_cold(semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 256 }).expect("replay store closes through original owners");
        turns += 1;
        assert!(turns < 100_000, "replay store retirement terminates");
    }
}
