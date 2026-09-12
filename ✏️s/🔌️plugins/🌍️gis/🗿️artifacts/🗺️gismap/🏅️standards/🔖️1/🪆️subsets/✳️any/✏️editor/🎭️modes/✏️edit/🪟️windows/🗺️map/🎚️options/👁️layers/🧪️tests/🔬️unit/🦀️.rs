use super::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn the_group_carries_one_toggle_per_declared_layer() {
    let config = MapWindowConfig::default();
    let WindowMeasure::Group { children, default_open, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::default())) else {
        panic!("layers is a group measure");
    };
    assert_eq!(children.len(), GIS_MAP_LAYER_IDS.len());
    assert_eq!(default_open, Some(true));
    assert!(children.iter().all(|child| matches!(child, WindowMeasure::Toggle { pressed: true, .. })), "every layer starts visible");
}

#[semio_framework_async_macros::async_test]
async fn hiding_a_layer_unpresses_just_that_toggle() {
    let mut config = MapWindowConfig::default();
    config.layer_visibility.insert("water".into(), false);
    let WindowMeasure::Group { children, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::default())) else {
        panic!("layers is a group measure");
    };
    let water = children.iter().find(|child| matches!(child, WindowMeasure::Toggle { id, .. } if id.ends_with(".water"))).expect("water toggle");
    assert!(matches!(water, WindowMeasure::Toggle { pressed: false, .. }));
}
