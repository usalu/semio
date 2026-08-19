//! 👁️ Map window option — the per-layer visibility toggle group.

use crate::editor::gis2d::config::{layer_visible, Gis2dConfig};
use crate::editor::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use crate::editor::gis2d::{gis2d_action, GIS_MAP_LAYER_IDS};
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

//#region 🔖️Option
pub const GIS2D_LAYERS_MEASURE_ID: &str = "gis2d-play-window.layers";

pub async fn measure(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> WindowMeasure {
    let children: Vec<WindowMeasure> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, _, icon)| WindowMeasure::Toggle {
            id: format!("gis2d-play-window.layer.{id}"),
            icon_id: (*icon).into(),
            label: Some(gis2d_layer_label(id, labels).into()),
            pressed: layer_visible(cfg, id),
            text: None,
            on_change: gis2d_action("toggleLayerVisibility", Some(json!({ "layerId": id }))),
        })
        .collect();
    WindowMeasure::Group {
        id: GIS2D_LAYERS_MEASURE_ID.into(),
        label: labels.layers_group.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children,
    }
}
//#endregion 🔖️Option

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::terminology::gis2d_labels;

    #[test]
    async fn the_group_carries_one_toggle_per_declared_layer() {
        let config = Gis2dConfig::default();
        let WindowMeasure::Group { children, default_open, .. } = measure(&config, gis2d_labels(&config)) else {
            panic!("layers is a group measure");
        };
        assert_eq!(children.len(), GIS_MAP_LAYER_IDS.len());
        assert_eq!(default_open, Some(true));
        assert!(children.iter().all(|child| matches!(child, WindowMeasure::Toggle { pressed: true, .. })), "every layer starts visible");
    }

    #[test]
    async fn hiding_a_layer_unpresses_just_that_toggle() {
        let mut config = Gis2dConfig::default();
        config.layer_visibility.insert("water".into(), false);
        let WindowMeasure::Group { children, .. } = measure(&config, gis2d_labels(&config)) else {
            panic!("layers is a group measure");
        };
        let water = children.iter().find(|child| matches!(child, WindowMeasure::Toggle { id, .. } if id.ends_with(".water"))).expect("water toggle");
        assert!(matches!(water, WindowMeasure::Toggle { pressed: false, .. }));
    }
}
//#endregion 🧪️Tests
