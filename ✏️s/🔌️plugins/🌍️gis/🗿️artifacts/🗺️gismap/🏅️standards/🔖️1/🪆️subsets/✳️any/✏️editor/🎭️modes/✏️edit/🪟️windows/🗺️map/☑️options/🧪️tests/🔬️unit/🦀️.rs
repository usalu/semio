//! 🧪️ Aggregated unit tests of the `options` leaves, one module per leaf.

mod layer_weights {
use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use semio_framework_plugin::WindowMeasure;
use super::super::layer_weights::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn weight_entries_default_to_one_and_honour_explicit_overrides() {
    let mut config = MapWindowConfig::default();
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let defaults = layer_weight_entries(&config, labels);
    assert!(defaults.iter().all(|(_, _, value)| *value == 1.0));
    let Some((first_id, _, _)) = defaults.first().cloned() else { return };
    config.layer_stroke_scale.insert(first_id.clone(), 2.0);
    let overridden = layer_weight_entries(&config, gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)));
    assert_eq!(overridden.iter().find(|(id, _, _)| id == &first_id).map(|(_, _, value)| *value), Some(2.0));
}

#[semio_framework_async_macros::async_test]
async fn the_group_is_collapsed_by_default_and_mirrors_the_entry_list() {
    let config = MapWindowConfig::default();
    let labels = gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let WindowMeasure::Group { children, default_open, .. } = measure(&config, labels) else {
        panic!("layer weights is a group measure");
    };
    assert_eq!(default_open, Some(false));
    assert_eq!(children.len(), layer_weight_entries(&config, labels).len());
}
}

mod layers {
use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use crate::editor::gis2d::constants_and_io::GIS_MAP_LAYER_IDS;
use semio_framework_plugin::WindowMeasure;
use super::super::layers::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn the_group_carries_one_toggle_per_declared_layer() {
    let config = MapWindowConfig::default();
    let WindowMeasure::Group { children, default_open, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) else {
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
    let WindowMeasure::Group { children, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) else {
        panic!("layers is a group measure");
    };
    let water = children.iter().find(|child| matches!(child, WindowMeasure::Toggle { id, .. } if id.ends_with(".water"))).expect("water toggle");
    assert!(matches!(water, WindowMeasure::Toggle { pressed: false, .. }));
}
}

mod render_mode {
use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use semio_framework_plugin::WindowMeasure;
use super::super::render_mode::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn the_measure_mirrors_the_config_value_and_offers_all_three_modes() {
    let config = MapWindowConfig::default();
    let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) else {
        panic!("render mode is a select measure");
    };
    assert_eq!(value, "combined");
    assert_eq!(items.len(), 3);
}
}

mod vector_style {
use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use semio_framework_plugin::WindowMeasure;
use super::super::vector_style::*;
use crate::editor::gis2d::terminology::gis2d_labels;

#[semio_framework_async_macros::async_test]
async fn the_measure_mirrors_the_config_value_and_offers_all_three_styles() {
    let config = MapWindowConfig::default();
    let WindowMeasure::Select { value, items, .. } = measure(&config, gis2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) else {
        panic!("vector style is a select measure");
    };
    assert_eq!(value, "colored");
    assert_eq!(items.len(), 3);
}
}
