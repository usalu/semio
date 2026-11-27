//! 🧪️ Store-fixture law — every bundled example survives the exact envelope round trip a mounted
//! artifact store performs.

use crate::standards::v1::subsets::any::io::binary::snapshot::{decode, encode};
use crate::schema::snapshot::Grid2dSnapshot;

fn examples() -> Vec<Grid2dSnapshot> {
    crate::examples::grid2d::sources().iter().map(|source| <Grid2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&source.document()).expect("example parses")).collect()
}

#[test]
fn every_example_survives_the_pack_envelope() {
    for document in examples() {
        let bytes = encode(&document);
        assert!(bytes.len() > 64);
        assert_eq!(decode(&bytes).expect("pack decodes"), document);
    }
}

#[semio_framework_async_macros::async_test]
async fn authored_seed_and_pins_survive_actual_pack_spr_replay() {
    let corpus: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/🀄️wfc/🧪️tests/🗄️store-replay/🔣️.json")).expect("portable replay corpus");
    assert_eq!(corpus["schema"], "semio.wfc.store-replay/v1");
    let case = corpus["cases"].as_array().expect("replay cases").iter().find(|case| case["artifact"] == "grid2d").expect("artifact replay case");
    let document = crate::examples::grid2d::pipes::document();
    assert_eq!(case["example"], "pipes");
    assert_ne!(document.seed, 99);
    assert!(!document.pinned.iter().any(|pin| pin.x == 1 && pin.y == 0));
    let schema = document.schema.clone();
    let envelope = store::os_store::document_materialize::create_document_envelope(&schema, "grid2d", document, None, &store::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = store::ArtifactStore::<Grid2dSnapshot, crate::mutations::Grid2dMutation>::new::<store::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("replay store opens");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Grid2dSnapshot, crate::mutations::Grid2dMutation>().expect("funded replay owners")).unwrap_or_else(|(error, _)| panic!("replay owners refused: {error}"));
    for mutation in [crate::mutations::change_seed(99), crate::mutations::pin_cell(1, 0, "pipe-h".into())] {
        store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("authored edit applies");
    }
    let observed = store.snapshot().expect("edited replay projection");
    let pin = observed.pinned.iter().find(|pin| pin.x == 1 && pin.y == 0).expect("authored pin");
    let projection = serde_json::json!({ "seed": observed.seed, "pin": { "x": pin.x, "y": pin.y, "tileId": pin.tile_id }, "geometry": { "width": observed.width, "height": observed.height } });
    assert_eq!(projection, case["expected"]);
    semio_os_store_test_support::assert_document_text_round_trip(&store).await;
    semio_os_store_test_support::assert_document_pack_round_trip(&store).await;
    let mut turns = 0;
    while !store.close_owned_terminal_is_empty() {
        store.close_owned_step_cold(semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 256 }).expect("replay store closes through original owners");
        turns += 1;
        assert!(turns < 100_000, "replay store retirement terminates");
    }
}
