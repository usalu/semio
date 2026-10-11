use super::*;
use protocol::{OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn pinned() -> BimWorldWindowConfig {
    BimWorldWindowConfig {
        camera: store::Viewport3dOrbit { position: [9.0, -9.0, 6.0], target: [4.0, 3.0, 1.0], zoom: 1.5, up: None },
        isolated_storey: "st-first".into(),
        hidden_storeys: vec!["st-ground".into()],
        section_enabled: true,
        section_axis: "x".into(),
        section_offset: 2.5,
        framed: true,
        energy_overlay: true,
        energy_mode: "boundary".into(),
        ..BimWorldWindowConfig::default()
    }
}

fn replace(config: BimWorldWindowConfig) -> BimWorldWindowConfigMutation {
    BimWorldWindowConfigMutation::Replace(Replace { config })
}

#[semio_framework_async_macros::async_test]
async fn the_default_shows_every_storey_without_a_section() {
    let config = BimWorldWindowConfig::default();
    assert!(config.isolated_storey.is_empty() && config.hidden_storeys.is_empty() && !config.section_enabled && !config.framed);
}

#[semio_framework_async_macros::async_test]
async fn the_config_round_trips_through_text_pack_and_the_op_codecs() {
    let config = pinned();
    assert_eq!(BimWorldWindowConfig::parse_dsl(&config.print_dsl()).expect("text"), config);
    assert_eq!(BimWorldWindowConfig::decode_pack(&config.encode_pack()).expect("pack"), config);
    let mutation = replace(config);
    assert_eq!(BimWorldWindowConfigMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(BimWorldWindowConfigMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}
