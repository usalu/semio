//! 🏪️ Store fixture — a real `Grid3dStore` takes every mutation of the roster against a bundled
//! example and lands on a document that still round-trips, with every store closed before drop.

use crate::mutations::{change_seed, create_tile, delete_tile, inverse_grid3d_mutation, mask_cell, pin_cell, unmask_cell, unpin_cell};
use crate::schema::snapshot::{Grid3dCell, Grid3dPinnedCell, Grid3dSnapshot, Grid3dTile};
use pack::snapshot::ArtifactPack;

#[test]
fn a_mounted_sequence_of_edits_lands_on_a_document_that_still_round_trips() {
    let mut document = crate::examples::blocks::snapshot();
    let steps = vec![
        change_seed(4_242),
        create_tile(Grid3dTile { id: "zzz-glass".into(), label: Some("Glass".into()), weight: 0.5, media: Default::default() }),
        pin_cell(Grid3dPinnedCell { x: 1, y: 0, z: 0, tile_id: "wall".into() }),
        mask_cell(Grid3dCell { x: 2, y: 2, z: 3 }),
        unmask_cell(3, 2, 3),
        unpin_cell(0, 0, 0),
        delete_tile("zzz-glass".into()),
    ];
    for step in &steps {
        vcs::apply_mutation(&document, step).map(|(applied_state, _)| { document = applied_state; }).expect("every step of the roster applies");
    }
    let pack = ArtifactPack::encode_pack(&document);
    assert_eq!(<Grid3dSnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), document);
    let text = crate::standards::v1::subsets::any::io::text::snapshot::print_dsl(&document);
    assert_eq!(crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(&text).expect("dsl"), document);
}

#[test]
fn undoing_the_whole_sequence_returns_the_document_to_the_committed_example() {
    let base = crate::examples::blocks::snapshot();
    let steps = vec![
        change_seed(4_242),
        create_tile(Grid3dTile { id: "zzz-glass".into(), label: None, weight: 0.5, media: Default::default() }),
        pin_cell(Grid3dPinnedCell { x: 1, y: 0, z: 0, tile_id: "wall".into() }),
        mask_cell(Grid3dCell { x: 2, y: 2, z: 3 }),
    ];
    let mut document = base.clone();
    let mut inverses = Vec::new();
    for step in &steps {
        inverses.push(inverse_grid3d_mutation(&document, step).expect("valid retained mutation inverse fixture"));
        vcs::apply_mutation(&document, step).map(|(applied_state, _)| { document = applied_state; }).expect("forward applies");
    }
    for inverse in inverses.iter().rev() {
        for step in inverse {
            vcs::apply_mutation(&document, step).map(|(applied_state, _)| { document = applied_state; }).expect("inverse applies");
        }
    }
    assert_eq!(document, base, "the whole edit ladder is point-invertible");
}


#[semio_framework_async_macros::async_test]
async fn authored_seed_and_pins_survive_actual_pack_spr_replay() {
    let corpus: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/🀄️wfc/🧪️tests/🗄️store-replay/🔣️.json")).expect("portable replay corpus");
    assert_eq!(corpus["schema"], "semio.wfc.store-replay/v1");
    let case = corpus["cases"].as_array().expect("replay cases").iter().find(|case| case["artifact"] == "grid3d").expect("artifact replay case");
    let document = crate::examples::blocks::snapshot();
    assert_eq!(case["example"], "blocks");
    assert_ne!(document.seed, 99);
    assert!(!document.pinned.iter().any(|pin| pin.x == 1 && pin.y == 0 && pin.z == 0));
    let schema = document.schema.clone();
    let envelope = store::os_store::document_materialize::create_document_envelope(&schema, "grid3d", document, None, &store::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = store::ArtifactStore::<Grid3dSnapshot, crate::mutations::Grid3dMutation>::new::<store::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("replay store opens");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Grid3dSnapshot, crate::mutations::Grid3dMutation>().expect("funded replay owners")).unwrap_or_else(|(error, _)| panic!("replay owners refused: {error}"));
    for mutation in [crate::mutations::change_seed(99), crate::mutations::pin_cell(Grid3dPinnedCell { x: 1, y: 0, z: 0, tile_id: "wall".into() })] {
        store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("authored edit applies");
    }
    let observed = store.snapshot().expect("edited replay projection");
    let pin = observed.pinned.iter().find(|pin| pin.x == 1 && pin.y == 0 && pin.z == 0).expect("authored pin");
    let projection = serde_json::json!({ "seed": observed.seed, "pin": { "x": pin.x, "y": pin.y, "z": pin.z, "tileId": pin.tile_id }, "geometry": { "width": observed.width, "height": observed.height, "depth": observed.depth } });
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
