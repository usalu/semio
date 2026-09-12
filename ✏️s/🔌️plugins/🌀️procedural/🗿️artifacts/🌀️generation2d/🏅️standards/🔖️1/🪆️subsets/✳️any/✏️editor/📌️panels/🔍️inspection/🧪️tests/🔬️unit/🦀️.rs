use super::*;
use crate::editor::generation2d::panels::catalogue as catalogue_panel;
use crate::editor::generation2d::unit_tests::context::{app, close, render_with_view};

/// 🌍️ Labels resolve off `ViewModel::locale` — the renderer is handed `Locale::De` here, exactly as
/// the `🧊️generation3d` twin does, so this law measures the German vocabulary rather than the
/// English default.
#[semio_framework_async_macros::async_test]
async fn generation2d_labels_translate_catalogue_and_inspector_in_german() {
    let mut app = app().await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let catalogue_json = render_with_view(&mut app, catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE, &view_state).await;
    let inspector_json = render_with_view(&mut app, GENERATION2D_PLAY_BODY_INSPECTION, &view_state).await;
    close(app);
    assert!(catalogue_json.contains("Quellen"), "{catalogue_json}");
    assert!(inspector_json.contains("Elemente:"), "{inspector_json}");
}
