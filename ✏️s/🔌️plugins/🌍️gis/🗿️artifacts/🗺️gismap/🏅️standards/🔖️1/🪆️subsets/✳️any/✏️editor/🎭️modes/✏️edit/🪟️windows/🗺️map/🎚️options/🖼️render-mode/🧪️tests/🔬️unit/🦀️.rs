use super::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn the_measure_mirrors_the_config_value_and_offers_all_three_modes() {
    let config = Gis2dConfig::default();
    let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::default())) else {
        panic!("render mode is a select measure");
    };
    assert_eq!(value, "combined");
    assert_eq!(items.len(), 3);
}
