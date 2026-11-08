use crate::modes::view::windows::viewport::*;
use semio_ui_locale::Locale;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_surface_for_the_default_document() {
    assert!(render(&Din18599Snapshot::default(), Locale::En).is_ok());
    assert!(render(&Din18599Snapshot::fire_demanding(), Locale::De).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn renders_the_toolbar_and_a_frame_at_a_fixed_mode() {
    use semio_s_plugin_norm_app_surface::viewport3d::playback::PlaybackState;
    use semio_s_plugin_norm_app_surface::viewport3d::toolbar::ViewportView;
    let view = ViewportView { playback: PlaybackState { track: Some("mode".into()), value: 2.0, ..Default::default() }, ..Default::default() };
    assert!(render_with(&Din18599Snapshot::unpropped_building(), &view, Locale::En).is_ok());
}
