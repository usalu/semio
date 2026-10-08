use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_between_law, assert_mutation_inverse_sum_law};
use protocol::{Mutation, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn busy() -> BimPresence {
    BimPresence { engagement_input: "Kitchen".into(), storey: "st-first".into(), camera: store::Viewport2d { x: 4.0, y: 3.0, zoom: 2.0 } }
}

#[semio_framework_async_macros::async_test]
async fn the_mutation_changes_only_the_fields_that_differ() {
    let base = BimPresence::default();
    let outcome = base.on_storey("st-ground").diff(&base);
    assert_eq!(outcome.diff().storey.as_deref(), Some("st-ground"));
    assert!(outcome.diff().engagement_input.is_none() && outcome.diff().camera.is_none());
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_sums_to_the_negative_diff() {
    assert_mutation_inverse_sum_law(&busy().looking_through(store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 }), &busy()).await;
    assert_mutation_inverse_sum_law(&BimPresence::default().on_storey("st-ground"), &busy()).await;
}

#[semio_framework_async_macros::async_test]
async fn between_is_the_state_delta() {
    assert_diff_algebra_between_law::<BimPresence, BimPresenceDiff>(&busy(), &BimPresence::default()).await;
}

#[semio_framework_async_macros::async_test]
async fn presence_round_trips_through_text_and_binary() {
    let presence = busy();
    assert_eq!(BimPresence::parse_dsl(&presence.print_dsl()).expect("text"), presence);
    let pack = presence.encode_pack();
    assert_eq!(BimPresence::decode_pack(&pack).expect("pack"), presence);
    assert_eq!(BimPresence::decode_pack(&[]).expect("empty is the default"), BimPresence::default());
    let mutation = presence.on_storey("st-ground");
    assert_eq!(BimPresenceMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(BimPresenceMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}
