use super::*;
use protocol::{OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn pinned() -> BimSheetWindowConfig {
    BimSheetWindowConfig { sheet: "sh-plans".into(), framed: true, viewport: store::Viewport2d { x: 4.0, y: -1.0, zoom: 30.0 } }
}

fn replace(config: BimSheetWindowConfig) -> BimSheetWindowConfigMutation {
    BimSheetWindowConfigMutation::Replace { config }
}

#[semio_framework_async_macros::async_test]
async fn the_config_round_trips_through_text_pack_and_the_op_codecs() {
    let config = pinned();
    assert_eq!(BimSheetWindowConfig::parse_dsl(&config.print_dsl()).expect("text"), config);
    assert_eq!(BimSheetWindowConfig::decode_pack(&config.encode_pack()).expect("pack"), config);
    let mutation = replace(config);
    assert_eq!(BimSheetWindowConfigMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(BimSheetWindowConfigMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}
