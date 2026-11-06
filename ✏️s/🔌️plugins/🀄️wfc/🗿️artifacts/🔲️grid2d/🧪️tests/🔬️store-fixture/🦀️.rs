//! 🧪️ Store-fixture law — every bundled example survives the exact envelope round trip a mounted
//! artifact store performs, and a decode-in-place hands back a bounded retirement that reaches its
//! terminal state before it is dropped.

use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::io::binary::snapshot::decode_into;
use crate::standards::v1::subsets::any::io::binary::snapshot::encode;
use crate::schema::snapshot::Grid2dSnapshot;

/// 🎟️ The exact frame capacity one displaced snapshot's retirement is born with.
fn admission() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<Grid2dSnapshot>(), maximum_depth: 1, ..Default::default() }
}

/// ♻️ Drains a retirement with grants quoted from its own next demand and returns the step count.
fn retire_all(retirement: &mut Box<dyn store::ErasedSnapshotRetirement>) -> usize {
    let mut steps = 0;
    while !retirement.terminal_is_empty() {
        let demand = retirement.next_demand(1 << 16).expect("a retained snapshot quotes its next demand");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = retirement.close_step(grant).expect("a funded step makes progress");
        assert!(step.progress().fits(grant));
        steps += 1;
        assert!(steps < 100_000, "the retirement ladder must terminate");
    }
    steps
}

fn examples() -> Vec<Grid2dSnapshot> {
    crate::examples::grid2d::sources().iter().map(|source| <Grid2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&source.document()).expect("example parses")).collect()
}

#[test]
fn every_example_survives_the_pack_envelope() {
    for document in examples() {
        let bytes = encode(&document);
        assert!(bytes.len() > 64);
        assert_eq!(crate::io::binary::snapshot::decode(&bytes).expect("pack decodes"), document);
    }
}

#[test]
fn a_decode_in_place_retires_the_displaced_document_in_bounded_steps() {
    let documents = examples();
    let mut live = documents[0].clone();
    let incoming = encode(&documents[1]);
    let (mut retirement, birth) = decode_into(&mut live, &incoming, admission()).expect("decode in place");
    assert!(birth.fits(admission()));
    assert_eq!(live, documents[1], "the live projection is replaced");
    assert!(retire_all(&mut retirement) >= 1, "a real displacement releases at least one collection");
}

#[test]
fn a_starved_retirement_makes_no_progress_but_never_fails() {
    let documents = examples();
    let mut live = documents[0].clone();
    let (mut retirement, _) = decode_into(&mut live, &encode(&documents[1]), admission()).expect("decode in place");
    let starved = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..admission() };
    assert_eq!(retirement.close_step(starved).expect("a starved step is not an error"), semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
    retire_all(&mut retirement);
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
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    let mut turns = 0;
    while !store.close_owned_terminal_is_empty() {
        store.close_owned_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("replay store closes through original owners");
        turns += 1;
        assert!(turns < 100_000, "replay store retirement terminates");
    }
}
