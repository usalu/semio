use super::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn the_tier_list_always_starts_with_the_automatic_mode() {
    let entries = lod_select_entries(gis2d_labels(&semio_framework_plugin::ViewModel::default()));
    assert_eq!(entries[0].0, GIS_MAP_LOD_MODE_AUTOMATIC);
    assert_eq!(lod_arg_options().len(), entries.len(), "the palette arg schema and the window select share one vocabulary");
}

#[semio_framework_async_macros::async_test]
async fn the_measure_mirrors_the_config_value() {
    let config = Gis2dConfig::default();
    let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::default())) else {
        panic!("lod mode is a select measure");
    };
    assert_eq!(value, GIS_MAP_LOD_MODE_AUTOMATIC);
    assert!(!items.is_empty());
}
