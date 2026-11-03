use crate::modes::view::windows::viewport::*;
use semio_framework_ui_locale::Locale;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_surface_for_the_default_document() {
    assert!(render(&En1997Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), Locale::En).is_ok());
    assert!(render(&semio_s_artifact_norm_en1997_model::noncompliant_demo(), Locale::De).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn render_with_scrubs_the_consolidation_track_at_a_fixed_time() {
    use semio_s_plugin_norm_app_surface::viewport3d::{playback::PlaybackState, toolbar::ViewportView};
    let view = ViewportView { playback: PlaybackState { track: Some("consolidation".into()), value: 0.0, ..Default::default() }, ..Default::default() };
    assert!(render_with(&semio_s_artifact_norm_en1997_model::compliant_demo(), &view, Locale::En).is_ok());
}
