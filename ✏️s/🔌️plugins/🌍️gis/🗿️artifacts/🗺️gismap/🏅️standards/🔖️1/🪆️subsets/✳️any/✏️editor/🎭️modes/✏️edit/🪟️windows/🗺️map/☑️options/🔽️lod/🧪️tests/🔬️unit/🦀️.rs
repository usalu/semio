use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use semio_framework_plugin::WindowMeasure;
use semio_framework_surface::tiled_map::MAP_LOD_MODE_AUTOMATIC;
#[allow(unused_imports)]
use crate::editor::gis2d::maphost;
#[allow(unused_imports)]
use crate::editor::gis2d::constants_and_io::gis2d_window_action;
#[allow(unused_imports)]
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MeasureSelectItem;
#[allow(unused_imports)]
use crate::tile_coordinates::gis_policy::gis_map_lod_scale_json;
#[allow(unused_imports)]
use serde_json::Value;
use crate::editor::gis2d::modes::edit::windows::map::options::lod_mode::{resolved_automatic_lod_id};
use crate::editor::gis2d::modes::edit::windows::map::options::lod_mode::*;
use crate::editor::gis2d::terminology::gis2d_labels;
use crate::GisMapSnapshot;

#[semio_framework_async_macros::async_test]
async fn the_tier_list_always_starts_with_the_automatic_mode() {
    let config = MapWindowConfig::default();
    let document = GisMapSnapshot::default();
    let entries = lod_select_entries(gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &document, &config);
    assert_eq!(entries[0].0, MAP_LOD_MODE_AUTOMATIC);
    assert_eq!(lod_arg_options().len(), entries.len(), "the palette arg schema and the window select share one vocabulary");
}

#[semio_framework_async_macros::async_test]
async fn the_measure_mirrors_the_config_value() {
    let config = MapWindowConfig::default();
    let document = GisMapSnapshot::default();
    let WindowMeasure::Select { value, items, .. } = measure(&document, &config, gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) else {
        panic!("lod mode is a select measure");
    };
    assert_eq!(value, MAP_LOD_MODE_AUTOMATIC);
    assert!(!items.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn automatic_mode_shows_the_resolved_lod_band_in_parentheses() {
    let document = GisMapSnapshot::default();
    let config = MapWindowConfig::default();
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let WindowMeasure::Select { items, .. } = measure(&document, &config, &labels) else {
        panic!("lod mode is a select measure");
    };
    let automatic = items.first().expect("automatic entry");
    assert_eq!(automatic.value, MAP_LOD_MODE_AUTOMATIC);
    let expected_id = resolved_automatic_lod_id(&document, &config).expect("automatic mode resolves a band");
    assert_eq!(automatic.label, format!("{} ({expected_id})", labels.lod_automatic.as_str()));
}

#[semio_framework_async_macros::async_test]
async fn a_forced_lod_mode_leaves_the_automatic_entry_unsuffixed() {
    let document = GisMapSnapshot::default();
    let config = MapWindowConfig { lod_mode: "city".into(), ..MapWindowConfig::default() };
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let WindowMeasure::Select { items, value, .. } = measure(&document, &config, &labels) else {
        panic!("lod mode is a select measure");
    };
    assert_eq!(value, "city");
    assert_eq!(items.first().expect("automatic entry").label, labels.lod_automatic.as_str());
}
