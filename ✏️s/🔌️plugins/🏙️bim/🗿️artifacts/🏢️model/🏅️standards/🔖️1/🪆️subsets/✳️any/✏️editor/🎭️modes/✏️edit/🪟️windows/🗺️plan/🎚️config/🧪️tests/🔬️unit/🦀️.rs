use super::*;
use protocol::{DiffAlgebra, Mutation, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

fn pinned() -> BimPlanWindowConfig {
    BimPlanWindowConfig { view: "v-first".into(), framed: true, viewport: store::Viewport2d { x: 3.0, y: -2.0, zoom: 55.0 } }
}

fn replace(config: BimPlanWindowConfig) -> BimPlanWindowConfigMutation {
    BimPlanWindowConfigMutation::Replace(Replace { config })
}

#[semio_framework_async_macros::async_test]
async fn the_default_shows_no_view_and_waits_to_be_framed() {
    let config = BimPlanWindowConfig::default();
    assert!(config.view.is_empty() && !config.framed);
}

#[semio_framework_async_macros::async_test]
async fn the_replace_mutation_diff_names_only_the_fields_that_differ() {
    let base = pinned();
    let outcome = replace(BimPlanWindowConfig { view: "v-other".into(), ..base.clone() }).diff(&base);
    assert_eq!(outcome.diff(), &BimPlanWindowConfigDiff { view: Some("v-other".into()), ..BimPlanWindowConfigDiff::default() });
    assert!(replace(base.clone()).diff(&base).diff().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_config_round_trips_through_text_pack_and_the_op_codecs() {
    let config = pinned();
    assert_eq!(BimPlanWindowConfig::parse_dsl(&config.print_dsl()).expect("text"), config);
    assert_eq!(BimPlanWindowConfig::decode_pack(&config.encode_pack()).expect("pack"), config);
    let mutation = replace(config);
    assert_eq!(BimPlanWindowConfigMutation::parse_op(&mutation.print_op()).expect("op text"), mutation);
    assert_eq!(BimPlanWindowConfigMutation::decode_op(&mutation.encode_op().expect("encode")).expect("op binary"), mutation);
}
