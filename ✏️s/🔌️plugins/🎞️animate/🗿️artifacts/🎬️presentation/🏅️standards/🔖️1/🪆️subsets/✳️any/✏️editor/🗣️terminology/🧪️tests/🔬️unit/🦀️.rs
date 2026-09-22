/// 🌱️ Relocated from the deleted `set-selected-ids` command's test mod (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — exercises the same app-wide label
/// resolution, unrelated to selection.
#[semio_framework_async_macros::async_test]
async fn animate_presentation_labels_resolve_native_by_default() {
    use crate::editor::animate::unit_tests::context::{presentation_app, render};
    use crate::editor::animate::{PRESENTATION_PLAY_BODY_CATALOGUE, PRESENTATION_PLAY_BODY_DETAILS};
    let mut app = presentation_app().await;
    let catalogue = render(&mut app, PRESENTATION_PLAY_BODY_CATALOGUE).await;
    assert!(catalogue.contains("Tile templates"));
    assert!(catalogue.contains("Split 2×2 grid"));
    assert!(catalogue.contains("Active source"));
    assert!(!catalogue.contains("Kachelvorlagen"));
    let _ = PRESENTATION_PLAY_BODY_DETAILS;
}

#[semio_framework_async_macros::async_test]
async fn animate_presentation_labels_translate_panels_in_german() {
    use crate::editor::animate::unit_tests::context::{presentation_app, render_with_view};
    use crate::editor::animate::{PRESENTATION_PLAY_BODY_CATALOGUE, PRESENTATION_PLAY_BODY_ARTIFACT};
    // 🌍️ The locale lives on the `ViewModel` the render is given; the default one resolves NATIVE
    // English, so this law has to hand the app a German view model rather than the default.
    let german = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::locale_from_str("de"), ..Default::default() };
    let mut app = presentation_app().await;
    let catalogue_json = render_with_view(&mut app, PRESENTATION_PLAY_BODY_CATALOGUE, &german).await;
    assert!(catalogue_json.contains("Kachelvorlagen"));
    assert!(catalogue_json.contains("2×2-Raster teilen"));
    assert!(catalogue_json.contains("Aktive Quelle"));
    assert!(!catalogue_json.contains("Tile templates"));

    let document_json = render_with_view(&mut app, PRESENTATION_PLAY_BODY_ARTIFACT, &german).await;
    assert!(document_json.contains("Kacheln"));
}
