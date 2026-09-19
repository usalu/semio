use super::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn the_measure_mirrors_the_config_value_and_offers_all_three_styles() {
    let config = MapWindowConfig::default();
    let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::default())) else {
        panic!("vector style is a select measure");
    };
    assert_eq!(value, "colored");
    assert_eq!(items.len(), 3);
}
