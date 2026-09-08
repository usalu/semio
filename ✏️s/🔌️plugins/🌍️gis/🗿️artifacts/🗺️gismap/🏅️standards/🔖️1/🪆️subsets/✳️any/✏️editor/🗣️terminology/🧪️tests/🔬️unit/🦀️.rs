
use super::*;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_from_the_shared_view_state() {
    assert_eq!(gis2d_labels(&semio_framework_plugin::ViewModel::default()).map_view.as_str(), "Map View");
    assert_eq!(gis2d_labels(&semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() }).map_view.as_str(), "Kartenansicht");
}

#[semio_framework_async_macros::async_test]
async fn every_declared_layer_id_resolves_to_a_non_empty_label() {
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::default());
    for (id, _, _) in crate::editor::gis2d::GIS_MAP_LAYER_IDS {
        assert!(!gis2d_layer_label(id, labels).is_empty(), "layer {id} has no label");
    }
    assert_eq!(gis2d_layer_label("bogusLayer", labels), "");
}
