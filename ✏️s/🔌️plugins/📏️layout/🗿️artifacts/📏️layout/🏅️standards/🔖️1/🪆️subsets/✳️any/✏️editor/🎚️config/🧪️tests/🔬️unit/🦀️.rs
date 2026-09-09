use super::*;

#[semio_framework_async_macros::async_test]
async fn layout_config_default_matches_the_existing_runtime_defaults() {
    let config = LayoutConfig::default();
    assert_eq!(config.active_page_id, "page-1");
    assert_eq!(config.drop_preview, LayoutDropPreviewState::default());
    assert_eq!(config.camera, LayoutCamera::default());
    assert_eq!(config.preview_camera, LayoutCamera::default());
}

#[semio_framework_async_macros::async_test]
async fn layout_config_dsl_and_pack_round_trip() {
    let config = LayoutConfig {
        active_page_id: "page-2".into(),
        drop_preview: LayoutDropPreviewState { kind: "text".into(), x: 12.0, y: 34.0 },
        engagement_input: "export svg".into(),
        camera: LayoutCamera { x: 5.0, y: 6.0, zoom: 1.25 },
        preview_camera: LayoutCamera { x: 7.0, y: 8.0, zoom: 0.75 },
    };
    store::os_store::test_support::assert_dsl_round_trip(&config);
    store::os_store::test_support::assert_dsl_pack_equivalence(&config);
}

fn sample_config() -> LayoutConfig {
    LayoutConfig {
        active_page_id: "page-2".into(),
        drop_preview: LayoutDropPreviewState { kind: "rect".into(), x: 1.0, y: 2.0 },
        engagement_input: "export png".into(),
        camera: LayoutCamera { x: 10.0, y: 20.0, zoom: 1.5 },
        preview_camera: LayoutCamera { x: 3.0, y: 4.0, zoom: 2.0 },
    }
}

fn config_round_trip(base: &LayoutConfig, operation: &LayoutConfigMutation) -> LayoutConfig {
    let forward = operation.diff(base).diff().clone();
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().clone();
    }
    assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
    forward
}

#[semio_framework_async_macros::async_test]
async fn config_mutations_apply_and_restore_every_field() {
    let base = LayoutConfig::default();
    assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetActivePage(SetActivePage { page_id: "page-9".into() })).active_page_id, "page-9");
    let previewed = config_round_trip(&base, &LayoutConfigMutation::SetDropPreview(SetDropPreview { preview: LayoutDropPreviewState { kind: "rect".into(), x: 5.0, y: 6.0 } }));
    assert_eq!(previewed.drop_preview.kind, "rect");
    assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetEngagementInput(SetEngagementInput { value: "undo".into() })).engagement_input, "undo");
    let cam = config_round_trip(&base, &LayoutConfigMutation::SetCamera(SetCamera { camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 } }));
    assert_eq!(cam.camera, LayoutCamera { x: 1.0, y: 2.0, zoom: 3.0 });
    let preview_cam = config_round_trip(&base, &LayoutConfigMutation::SetPreviewCamera(SetPreviewCamera { camera: LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 } }));
    assert_eq!(preview_cam.preview_camera, LayoutCamera { x: 4.0, y: 5.0, zoom: 6.0 });
}

#[semio_framework_async_macros::async_test]
async fn config_snapshot_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&LayoutConfigMutation::SetActivePage(SetActivePage { page_id: "page-2".into() }));
}

#[semio_framework_async_macros::async_test]
async fn config_mutation_inverses_restore_each_field_without_a_snapshot_sentinel() {
    let base = sample_config();
    assert_eq!(config_round_trip(&base, &LayoutConfigMutation::SetActivePage(SetActivePage { page_id: "page-9".into() })).active_page_id, "page-9");
}
