use super::*;
use crate::editor::gis2d::terminology::gis2d_labels;
use crate::GisMapSnapshot;

#[semio_framework_async_macros::async_test]
async fn the_tier_list_always_starts_with_the_automatic_mode() {
    let config = MapWindowConfig::default();
    let document = GisMapSnapshot::default();
    let entries = lod_select_entries(gis2d_labels(&semio_framework_plugin::ViewModel::default()), &document, &config);
    assert_eq!(entries[0].0, GIS_MAP_LOD_MODE_AUTOMATIC);
    assert_eq!(lod_arg_options().len(), entries.len(), "the palette arg schema and the window select share one vocabulary");
}

#[semio_framework_async_macros::async_test]
async fn the_measure_mirrors_the_config_value() {
    let config = MapWindowConfig::default();
    let document = GisMapSnapshot::default();
    let WindowMeasure::Select { value, items, .. } = measure(&document, &config, gis2d_labels(&semio_framework_plugin::ViewModel::default())) else {
        panic!("lod mode is a select measure");
    };
    assert_eq!(value, GIS_MAP_LOD_MODE_AUTOMATIC);
    assert!(!items.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn automatic_mode_shows_the_resolved_lod_band_in_parentheses() {
    let document = GisMapSnapshot::default();
    let config = MapWindowConfig::default();
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::default());
    let WindowMeasure::Select { items, .. } = measure(&document, &config, &labels) else {
        panic!("lod mode is a select measure");
    };
    let automatic = items.first().expect("automatic entry");
    assert_eq!(automatic.value, GIS_MAP_LOD_MODE_AUTOMATIC);
    let expected_id = resolved_automatic_lod_id(&document, &config).expect("automatic mode resolves a band");
    assert_eq!(automatic.label, format!("{} ({expected_id})", labels.lod_automatic.as_str()));
}

#[semio_framework_async_macros::async_test]
async fn a_forced_lod_mode_leaves_the_automatic_entry_unsuffixed() {
    let document = GisMapSnapshot::default();
    let config = MapWindowConfig { lod_mode: "city".into(), ..MapWindowConfig::default() };
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::default());
    let WindowMeasure::Select { items, value, .. } = measure(&document, &config, &labels) else {
        panic!("lod mode is a select measure");
    };
    assert_eq!(value, "city");
    assert_eq!(items.first().expect("automatic entry").label, labels.lod_automatic.as_str());
}
