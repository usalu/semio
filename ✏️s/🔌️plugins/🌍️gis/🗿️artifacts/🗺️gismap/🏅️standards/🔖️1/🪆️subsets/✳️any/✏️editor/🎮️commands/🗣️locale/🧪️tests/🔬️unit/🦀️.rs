
use super::*;
use crate::editor::gis2d::Gis2dCommand;
use crate::editor::gis2d::modes::edit::windows::map::GIS2D_PLAY_WINDOW_MAIN;
use crate::editor::gis2d::panels::inspection::GIS2D_PLAY_BODY_INSPECTION;
use crate::editor::gis2d::testkit::{app, dispatch, main_window_measures, render};

#[semio_framework_async_macros::async_test]
async fn gis2d_labels_resolve_native_by_default() {
    let mut app = app().await;
    let json = render(&mut app, GIS2D_PLAY_BODY_INSPECTION).await;
    assert!(json.contains("\"Map View\""));
    assert!(json.contains("\"Render Mode\""));
    assert!(json.contains("\"Map Layer\""));
    assert!(!json.contains("Kartenansicht"));
}

/// 🗣️ Locale is `cfg.locale`, set via the typed `SetLocale` config command — no `ViewModel`-pushed
/// locale anywhere.
#[semio_framework_async_macros::async_test]
async fn gis2d_labels_translate_inspector_and_layers_in_german() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })).await;
    assert!(result.mutations.is_empty(), "locale is config state, not a document edit");

    let inspector_json = render(&mut app, GIS2D_PLAY_BODY_INSPECTION).await;
    assert!(inspector_json.contains("Kartenansicht"));
    assert!(inspector_json.contains("Darstellungsmodus"));
    assert!(inspector_json.contains("Kartenebene"));
    assert!(!inspector_json.contains("\"Map View\""));

    let document_json = render(&mut app, crate::editor::gis2d::panels::artifact::GIS2D_PLAY_BODY_DOCUMENT).await;
    assert!(document_json.contains("Wasser"));
    assert!(!document_json.contains("\"Water\""));

    let window_json = serde_json::to_string(&main_window_measures(&mut app).await).expect("measures json");
    assert!(window_json.contains("Ebenen"));
    assert!(window_json.contains("Ebenengewichte"));
    assert_eq!(GIS2D_PLAY_WINDOW_MAIN, "gis2d-main");
}
