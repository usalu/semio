
use super::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn weight_entries_default_to_one_and_honour_explicit_overrides() {
    let mut config = Gis2dConfig::default();
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::default());
    let defaults = layer_weight_entries(&config, labels);
    assert!(defaults.iter().all(|(_, _, value)| *value == 1.0));
    let Some((first_id, _, _)) = defaults.first().cloned() else { return };
    config.layer_stroke_scale.insert(first_id.clone(), 2.0);
    let overridden = layer_weight_entries(&config, gis2d_labels(&semio_framework_plugin::ViewModel::default()));
    assert_eq!(overridden.iter().find(|(id, _, _)| id == &first_id).map(|(_, _, value)| *value), Some(2.0));
}

#[semio_framework_async_macros::async_test]
async fn the_group_is_collapsed_by_default_and_mirrors_the_entry_list() {
    let config = Gis2dConfig::default();
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::default());
    let WindowMeasure::Group { children, default_open, .. } = measure(&config, labels) else {
        panic!("layer weights is a group measure");
    };
    assert_eq!(default_open, Some(false));
    assert_eq!(children.len(), layer_weight_entries(&config, labels).len());
}
