use crate::modes::view::windows::viewport::*;
use semio_ui_locale::Locale;
use semio_s_plugin_norm_app_surface::viewport3d::playback::PlaybackState;
use semio_s_plugin_norm_app_surface::viewport3d::toolbar::ViewportView;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_surface_for_the_default_document() {
    assert!(render(&En1995Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), Locale::En).is_ok());
    assert!(render(&En1995Snapshot::noncompliant_building(), Locale::De).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn render_with_a_playing_track_at_a_fixed_parameter_is_ok() {
    let document = { let mut d = En1995Snapshot::compliant_building_beam(); d.members[0].fire_duration_s = 1800.0; d };
    let view = ViewportView { playback: PlaybackState { track: Some("fire".into()), value: 4e+03, ..Default::default() }, ..Default::default() };
    assert!(render_with(&document, &view, Locale::En).is_ok());
    assert!(render_with(&document, &view, Locale::De).is_ok());
}
