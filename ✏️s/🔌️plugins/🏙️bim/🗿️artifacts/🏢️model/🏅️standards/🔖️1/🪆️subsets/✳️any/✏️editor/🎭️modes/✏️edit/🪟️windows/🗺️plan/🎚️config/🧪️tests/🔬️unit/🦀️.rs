use super::*;
use protocol::os_spr::protocol_laws::assert_diff_algebra_between_law;
use protocol::{Mutation, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn pinned() -> BimPlanWindowConfig {
    BimPlanWindowConfig { storey: "st-first".into(), cut_height: 1.5, framed: true, viewport: store::Viewport2d { x: 3.0, y: -2.0, zoom: 55.0 } }
}

fn snapshot(config: BimPlanWindowConfig) -> BimPlanWindowConfigMutation {
    BimPlanWindowConfigMutation::Snapshot { config }
}

#[semio_framework_async_macros::async_test]
async fn the_default_shows_no_storey_and_waits_to_be_framed() {
    let config = BimPlanWindowConfig::default();
    assert!(config.storey.is_empty() && !config.framed);
    assert!((config.cut_height - 1.2).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn the_snapshot_mutation_replaces_the_whole_configuration() {
    let outcome = snapshot(pinned()).diff(&BimPlanWindowConfig::default());
    assert_eq!(outcome.diff(), &pinned());
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_sums_to_the_negative_diff_and_between_is_the_state_delta() {
    crate::render::window_config::assert_window_config_laws(&BimPlanWindowConfig::default(), &snapshot(pinned())).await;
    assert_diff_algebra_between_law::<BimPlanWindowConfig, BimPlanWindowConfig>(&pinned(), &BimPlanWindowConfig::default()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_config_round_trips_through_text_pack_and_the_op_codecs() {
    let config = pinned();
    assert_eq!(BimPlanWindowConfig::parse_dsl(&config.print_dsl()).expect("text"), config);
    assert_eq!(BimPlanWindowConfig::decode_pack(&config.encode_pack()).expect("pack"), config);
    let mutation = snapshot(config);
    assert_eq!(BimPlanWindowConfigMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(BimPlanWindowConfigMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}
