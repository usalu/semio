
use super::*;
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn gis2d_config_default_matches_the_existing_action_arg_sticky_defaults() {
    let config = Gis2dConfig::default();
    assert_eq!(config.render_mode, "combined");
    assert_eq!(config.vector_style, "colored");
    assert_eq!(config.lod_mode, "automatic");
    assert_eq!(config.locale, "en-US");
}

#[semio_framework_async_macros::async_test]
async fn gis2d_config_default_lod_mode_matches_the_tiled_map_surface_constant() {
    assert_eq!(Gis2dConfig::default().lod_mode, semio_framework_surface::tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC);
}

#[semio_framework_async_macros::async_test]
async fn layer_visible_defaults_to_true_and_honours_explicit_entries() {
    let mut config = Gis2dConfig::default();
    assert!(layer_visible(&config, "water"), "a layer with no entry is visible");
    config.layer_visibility.insert("water".into(), false);
    assert!(!layer_visible(&config, "water"));
}

#[semio_framework_async_macros::async_test]
async fn gis2d_config_dsl_round_trips_default_and_populated() {
    store::os_store::test_support::assert_dsl_round_trip(&Gis2dConfig::default());
    let mut populated = Gis2dConfig::default();
    populated.layer_visibility.insert("water".into(), false);
    populated.layer_stroke_scale.insert("roads".into(), 1.5);
    store::os_store::test_support::assert_dsl_round_trip(&populated);
    store::os_store::test_support::assert_dsl_pack_equivalence(&populated);
}

#[semio_framework_async_macros::async_test]
async fn gis2d_config_operation_diff_writes_the_targeted_field_and_leaves_the_rest() {
    let base = Gis2dConfig::default();
    let next = Gis2dConfigMutation::SetRenderMode(SetRenderMode { value: "vector".into() }).diff(&base).diff().apply(&base).expect("apply");
    assert_eq!(next.render_mode, "vector");
    assert_eq!(next.vector_style, base.vector_style, "untouched fields survive the diff");
}

#[semio_framework_async_macros::async_test]
async fn gis2d_config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = Gis2dConfig::default();
    let operation = Gis2dConfigMutation::SetLayerVisibility(SetLayerVisibility { layer_id: "water".into(), visible: Some(false) });
    let next = operation.diff(&base).diff().apply(&base).expect("apply");
    assert_eq!(next.layer_visibility.get("water"), Some(&false));
    let backwards = operation.inverse(&base);
    assert_eq!(backwards, vec![Gis2dConfigMutation::SetLayerVisibility(SetLayerVisibility { layer_id: "water".into(), visible: None })]);
    let restored = backwards[0].diff(&next).diff().apply(&next).expect("restore");
    assert_eq!(restored, base, "the per-field inverse restores the exact pre-operation config, including the absent map entry");
}

/// ⚖️ `SetLayerStrokeScale`'s inverse has the same absent-entry-vs-default subtlety as
/// `SetLayerVisibility` above, covered separately since it defaults to `1.0` not `true`.
#[semio_framework_async_macros::async_test]
async fn gis2d_config_layer_stroke_scale_backwards_restores_an_absent_entry() {
    let base = Gis2dConfig::default();
    let operation = Gis2dConfigMutation::SetLayerStrokeScale(SetLayerStrokeScale { layer_id: "roads".into(), value: Some(2.0) });
    let next = operation.diff(&base).diff().apply(&base).expect("apply");
    assert_eq!(next.layer_stroke_scale.get("roads"), Some(&2.0));
    let backwards = operation.inverse(&base);
    let restored = backwards[0].diff(&next).diff().apply(&next).expect("restore");
    assert_eq!(restored, base);
    assert!(!restored.layer_stroke_scale.contains_key("roads"));
}

#[semio_framework_async_macros::async_test]
async fn gis2d_config_operation_lines_round_trip() {
    store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLayerVisibility(SetLayerVisibility { layer_id: "water".into(), visible: Some(false) }));
    store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetCamera(SetCamera { camera_json: r#"{"x":1,"y":2,"zoom":3}"#.into() }));
    store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetRenderMode(SetRenderMode { value: "vector".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetVectorStyle(SetVectorStyle { value: "figureGround".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLodMode(SetLodMode { value: "automatic".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLayerStrokeScale(SetLayerStrokeScale { layer_id: "roads".into(), value: Some(1.5) }));
    store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
}
