use crate::modes::view::windows::viewport::*;
use semio_ui_locale::Locale;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_surface_for_the_default_document() {
    assert!(render(&En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), Locale::En).is_ok());
    assert!(render(&En1998Snapshot::noncompliant_de_office(), Locale::De).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn render_with_shows_one_eigenmode_at_a_fixed_track_value() {
    use semio_s_plugin_norm_app_surface::viewport3d::{playback::PlaybackState, toolbar::ViewportView};
    let view = ViewportView { playback: PlaybackState { track: Some("mode".into()), value: 2.0, ..Default::default() }, ..Default::default() };
    assert!(render_with(&En1998Snapshot::compliant_de_office(), &view, Locale::En).is_ok());
}
