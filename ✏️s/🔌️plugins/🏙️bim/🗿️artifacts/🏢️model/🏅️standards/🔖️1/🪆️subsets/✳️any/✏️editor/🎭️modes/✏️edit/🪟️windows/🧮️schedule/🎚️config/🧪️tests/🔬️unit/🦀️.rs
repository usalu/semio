use super::*;
use protocol::os_spr::protocol_laws::assert_diff_algebra_between_law;
use protocol::{OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn pinned() -> BimScheduleWindowConfig {
    BimScheduleWindowConfig { schedule: "sch-doors".into(), editing: true }
}

fn snapshot(config: BimScheduleWindowConfig) -> BimScheduleWindowConfigMutation {
    BimScheduleWindowConfigMutation::Snapshot { config }
}

#[semio_framework_async_macros::async_test]
async fn the_inverse_sums_to_the_negative_diff_and_between_is_the_state_delta() {
    crate::render::window_config::assert_window_config_laws(&BimScheduleWindowConfig::default(), &snapshot(pinned())).await;
    assert_diff_algebra_between_law::<BimScheduleWindowConfig, BimScheduleWindowConfigDiff>(&pinned(), &BimScheduleWindowConfig::default()).await;
}

#[semio_framework_async_macros::async_test]
async fn the_config_round_trips_through_text_pack_and_the_op_codecs() {
    let config = pinned();
    assert_eq!(BimScheduleWindowConfig::parse_dsl(&config.print_dsl()).expect("text"), config);
    assert_eq!(BimScheduleWindowConfig::decode_pack(&config.encode_pack()).expect("pack"), config);
    let mutation = snapshot(config);
    assert_eq!(BimScheduleWindowConfigMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(BimScheduleWindowConfigMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}
