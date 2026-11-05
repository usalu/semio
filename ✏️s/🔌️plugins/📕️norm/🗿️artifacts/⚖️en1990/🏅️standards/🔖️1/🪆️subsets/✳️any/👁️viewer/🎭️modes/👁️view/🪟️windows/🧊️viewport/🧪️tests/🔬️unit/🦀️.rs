use crate::modes::view::windows::viewport::*;
use semio_ui_locale::Locale;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_surface_for_the_default_document() {
    assert!(render(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), Locale::En).is_ok());
    let mut failing = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    failing.members[0].rd_str = 1.0;
    assert!(render(&failing, Locale::De).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn render_with_a_combination_frame_is_ok() {
    use semio_s_plugin_norm_app_surface::viewport3d::{playback::PlaybackState, toolbar::ViewportView};
    let view = ViewportView { playback: PlaybackState { track: Some("combination".into()), value: 2.0, ..Default::default() }, ..Default::default() };
    assert!(render_with(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), &view, Locale::En).is_ok());
    assert!(render_with(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), &view, Locale::De).is_ok());
}
