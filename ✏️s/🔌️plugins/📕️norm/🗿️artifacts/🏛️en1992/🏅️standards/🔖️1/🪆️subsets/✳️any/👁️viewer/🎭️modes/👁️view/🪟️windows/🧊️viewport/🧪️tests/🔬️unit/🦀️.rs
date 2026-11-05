use crate::modes::view::windows::viewport::*;
use semio_ui_locale::Locale;
use semio_s_plugin_norm_app_surface::viewport3d::playback::PlaybackState;
use semio_s_plugin_norm_app_surface::viewport3d::toolbar::ViewportView;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_surface_for_the_default_document() {
    assert!(render(&En1992Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), Locale::En).is_ok());
    assert!(render(&En1992Snapshot::failing_under_reinforced(), Locale::De).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn render_with_a_playing_track_at_a_fixed_parameter_is_ok() {
    let document = En1992Snapshot::compliant_office_frame();
    let view = ViewportView { playback: PlaybackState { track: Some("creep".into()), value: 3e+07, ..Default::default() }, ..Default::default() };
    assert!(render_with(&document, &view, Locale::En).is_ok());
    assert!(render_with(&document, &view, Locale::De).is_ok());
}
