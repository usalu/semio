//! 🧪️ Store fixture — the document survives a real `ArtifactStore` round trip: every committed
//! example encodes, mounts, mutates through the store's own apply path, and comes back identical.

use crate::mutations::{change_seed, pin_pixel, set_input_pixels};
use crate::schema::snapshot::{BitmapSnapshot};
use pack::snapshot::ArtifactPack;

fn examples() -> Vec<BitmapSnapshot> {
    vec![crate::examples::rooms_16::snapshot(), crate::examples::flowers_24::snapshot()]
}

#[test]
fn every_example_survives_the_pack_envelope() {
    for snapshot in examples() {
        let bytes = ArtifactPack::encode_pack(&snapshot);
        assert!(bytes.len() > 64, "a real document is not an empty envelope");
        assert_eq!(<BitmapSnapshot as ArtifactPack>::decode_pack(&bytes).expect("pack decodes"), snapshot);
    }
}

#[test]
fn a_committed_example_takes_a_whole_edit_session_and_inverts_it() {
    let base = crate::examples::rooms_16::snapshot();
    let mut snapshot = base.clone();
    let edits = vec![change_seed(4242), pin_pixel(0, 0, 1), set_input_pixels(0, 0, 2, 2, ([1, 1, 1, 1]).to_vec())];
    let mut inverses = Vec::new();
    for edit in &edits {
        inverses.push(crate::mutations::inverse_bitmap_mutation(&snapshot, edit).expect("valid retained mutation inverse fixture"));
        vcs::apply_mutation(&snapshot, edit).map(|(applied_state, _)| { snapshot = applied_state; }).expect("edit applies");
    }
    assert_ne!(snapshot, base, "the session really moved the document");
    for steps in inverses.iter().rev() {
        for step in steps {
            vcs::apply_mutation(&snapshot, step).map(|(applied_state, _)| { snapshot = applied_state; }).expect("inverse applies");
        }
    }
    assert_eq!(snapshot, base, "the whole session inverts back to the committed example");
}

#[test]
fn the_document_codec_names_this_artifacts_schema() {
    let codec = store::bare_artifact_codec::<BitmapSnapshot, crate::BitmapMutation>(crate::WFC_BITMAP_DOCUMENT_SCHEMA.to_string());
    assert_eq!(codec.schema, crate::WFC_BITMAP_DOCUMENT_SCHEMA);
}


#[semio_framework_async_macros::async_test]
async fn authored_seed_pins_and_geometry_survive_actual_pack_spr_replay() {
    let corpus: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/🀄️wfc/🧪️tests/🗄️store-replay/🔣️.json")).expect("portable replay corpus");
    assert_eq!(corpus["schema"], "semio.wfc.store-replay/v1");
    let case = corpus["cases"].as_array().expect("replay cases").iter().find(|case| case["artifact"] == "bitmap").expect("artifact replay case");
    let document = crate::examples::rooms_16::snapshot();
    assert_eq!(case["example"], "rooms-16");
    assert_ne!(document.seed, 99);
    assert!(document.pinned.iter().all(|pin| pin.x != 0 || pin.y != 0));
    let schema = document.schema.clone();
    let envelope = store::os_store::document_materialize::create_document_envelope(&schema, "wfcbitmap", document, None, &store::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = store::ArtifactStore::<BitmapSnapshot, crate::BitmapMutation>::new::<store::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("replay store opens");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<BitmapSnapshot, crate::BitmapMutation>().expect("funded replay owners")).unwrap_or_else(|(error, _)| panic!("replay owners refused: {error}"));
    let edits: Vec<crate::BitmapMutation> = vec![change_seed(99), crate::mutations::resize_output(5, 4, true), pin_pixel(0, 0, 1)];
    for mutation in edits {
        store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("authored edit applies");
    }
    let observed = store.snapshot().expect("edited replay projection");
    let projection = {
        let pin = observed.pinned.iter().find(|pin| pin.x == 0 && pin.y == 0).expect("authored pin");
        serde_json::json!({ "seed": observed.seed, "pin": { "x": pin.x, "y": pin.y, "color": pin.color }, "geometry": { "width": observed.output.width, "height": observed.output.height, "periodic": observed.output.periodic } })
    };
    assert_eq!(projection, case["expected"]);
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    let mut turns = 0;
    while !store.close_owned_terminal_is_empty() {
        store.close_owned_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("replay store closes through original owners");
        turns += 1;
        assert!(turns < 100_000, "replay store retirement terminates");
    }
}
