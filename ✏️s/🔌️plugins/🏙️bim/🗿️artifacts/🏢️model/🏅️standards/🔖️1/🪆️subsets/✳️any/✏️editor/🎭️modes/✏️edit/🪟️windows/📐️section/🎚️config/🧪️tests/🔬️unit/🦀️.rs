use super::*;
use protocol::os_spr::protocol_laws::assert_diff_algebra_between_law;
use protocol::{OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn pinned() -> BimSectionWindowConfig {
    BimSectionWindowConfig { start_x: -1.0, start_y: 3.0, end_x: 9.0, end_y: 3.5, depth: 2.0, framed: true, viewport: store::Viewport2d { x: 4.0, y: -1.0, zoom: 30.0 } }
}

fn snapshot(config: BimSectionWindowConfig) -> BimSectionWindowConfigMutation {
    BimSectionWindowConfigMutation::Snapshot { config }
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_sums_to_the_negative_diff_and_between_is_the_state_delta() {
    crate::render::window_config::assert_window_config_laws(&BimSectionWindowConfig::default(), &snapshot(pinned())).await;
    assert_diff_algebra_between_law::<BimSectionWindowConfig, BimSectionWindowConfig>(&pinned(), &BimSectionWindowConfig::default()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_config_round_trips_through_text_pack_and_the_op_codecs() {
    let config = pinned();
    assert_eq!(BimSectionWindowConfig::parse_dsl(&config.print_dsl()).expect("text"), config);
    assert_eq!(BimSectionWindowConfig::decode_pack(&config.encode_pack()).expect("pack"), config);
    let mutation = snapshot(config);
    assert_eq!(BimSectionWindowConfigMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(BimSectionWindowConfigMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}
