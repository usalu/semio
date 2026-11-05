use crate::modes::view::windows::viewport::*;
use semio_ui_locale::Locale;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_surface_for_the_default_document() {
    assert!(render(&En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), Locale::En).is_ok());
    assert!(render(&En1991Snapshot::full_showcase(), Locale::De).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn render_with_a_fire_frame_is_ok() {
    use semio_s_plugin_norm_app_surface::viewport3d::{playback::PlaybackState, toolbar::ViewportView};
    let view = ViewportView { playback: PlaybackState { track: Some("fire-time".into()), value: 1800.0, ..Default::default() }, ..Default::default() };
    assert!(render_with(&En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), &view, Locale::En).is_ok());
    assert!(render_with(&En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), &view, Locale::De).is_ok());
}
